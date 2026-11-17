//! =============================================================
//! TAREFAS — as rotinas concorrentes do lilos
//!
//!   tempo      ( 1 s ) → conta os segundos do ciclo
//!   agitacao   (21 ms) → move o contador do vai-e-vem do motor
//!   tampa      (21 ms) → filtra os dois sensores de tampa
//!   amostragem (81 ms) → encoder, botões (ADC) e pressostato
//!   maquina    (sinal)→ executa a etapa atual do ciclo
//!   gravacao   (pedido)→ grava o estado na flash
//!   watchdog   ( 1 s ) → alimenta o FWDGT
//!
//! As quatro primeiras substituem os timers TIM14, TIM16 e TIM3 do
//! firmware C++. A `maquina` substitui o `while(1)` do `main()`.
//!
//! Todas são `async fn -> Infallible`: nunca terminam, é o que o
//! executor do lilos exige.
//! =============================================================

use core::convert::Infallible;

use lilos::time::{sleep_for, Millis, PeriodicGate};

use crate::diag;
use crate::encoder::{self, DetectorMudanca};
use crate::estado_salvo;
use crate::evento;
use crate::leituras;
use crate::maquina::{Etapa, Maquina};
use crate::mcu::pac::{Adc, Fmc, Fwdgt, Rcu};
use crate::mcu::{adc, wdt};
use crate::modelo;
use crate::placa::{Entradas, Saidas};
use crate::relogio;
use crate::agita;

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: TEMPO — o relógio do ciclo
// ═════════════════════════════════════════════════════════════════════════════

/// Conta um segundo por vez. Equivale à interrupção do TIM14.
pub async fn tempo() -> Infallible {
    let mut gate = PeriodicGate::from(Millis(1_000));
    loop {
        gate.next_time().await;
        relogio::atualiza();
        evento::acorda();
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: AGITAÇÃO — a base de tempo do vai-e-vem
// ═════════════════════════════════════════════════════════════════════════════

/// Move o contador da agitação a cada 21 ms (o TIM16 do C++).
///
/// É a tarefa mais rápida, então é ela que dá a cadência da máquina de
/// estados: a cada tique a etapa atual é reavaliada.
pub async fn agitacao() -> Infallible {
    let mut gate = PeriodicGate::from(Millis(agita::PERIODO_MS));
    loop {
        gate.next_time().await;
        agita::tique();
        evento::acorda();
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: TAMPA — filtro dos dois sensores
// ═════════════════════════════════════════════════════════════════════════════

/// Quantas leituras iguais seguidas confirmam a mudança da tampa.
///
/// ⚠ Aqui o porte **não** é literal. O C++ conta 250 leituras iguais
/// do laço principal, que roda livre — o tempo real depende da
/// velocidade do laço, algo na casa de 10 ms. Com amostragem fixa de
/// 21 ms, duas leituras dão ~42 ms, mesma ordem de grandeza e com
/// rejeição de ruído de chaveamento dos triacs. Vale conferir em
/// bancada se 42 ms atende a abertura de tampa desta máquina.
const LEITURAS_ESTAVEIS_TAMPA: u8 = 2;

/// Só aceita uma mudança depois de `limite` leituras iguais seguidas.
struct Debounce {
    estavel: bool,
    contador: u8,
    limite: u8,
}

impl Debounce {
    fn novo(inicial: bool, limite: u8) -> Self {
        Self {
            estavel: inicial,
            contador: 0,
            limite,
        }
    }

    fn amostra(&mut self, leitura: bool) {
        if leitura == self.estavel {
            self.contador = 0;
            return;
        }
        self.contador += 1;
        if self.contador >= self.limite {
            self.contador = 0;
            self.estavel = leitura;
        }
    }
}

/// Filtra os sensores de tampa eletrônico (PA4) e mecânico (PA7).
///
/// Os dois são filtrados sempre; quem escolhe de qual sensor ler é a
/// máquina, pelo modelo (ver `leituras::tampa_aberta`).
pub async fn tampa(e: &Entradas) -> Infallible {
    let mut eletronica = Debounce::novo(e.tampa_aberta(), LEITURAS_ESTAVEIS_TAMPA);
    let mut mecanica = Debounce::novo(e.tampa_mecanica_aberta(), LEITURAS_ESTAVEIS_TAMPA);

    let mut gate = PeriodicGate::from(Millis(agita::PERIODO_MS));
    loop {
        gate.next_time().await;
        eletronica.amostra(e.tampa_aberta());
        mecanica.amostra(e.tampa_mecanica_aberta());
        leituras::set_tampas(eletronica.estavel, mecanica.estavel);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: AMOSTRAGEM — encoder, botões e pressostato
// ═════════════════════════════════════════════════════════════════════════════

/// Período de amostragem, igual ao TIM3 do C++ (prescaler 24000 sobre
/// 24 MHz dá 1 kHz, autoreload 80 → estouro em 81 contagens).
pub const AMOSTRAGEM_MS: u64 = 81;

/// Média móvel de 10 amostras do pressostato.
///
/// A água bate no tubo do pressostato e o contato oscila; sem o filtro,
/// o ciclo pularia de etapa no meio da agitação. É o
/// `FiltroDebouncePressostato` do C++.
struct FiltroPressostato {
    amostras: [bool; Self::N],
    indice: usize,
}

impl FiltroPressostato {
    const N: usize = 10;

    fn novo(inicial: bool) -> Self {
        Self {
            amostras: [inicial; Self::N],
            indice: 0,
        }
    }

    fn amostra(&mut self, leitura: bool) {
        self.amostras[self.indice] = leitura;
        self.indice += 1;
        if self.indice == Self::N {
            self.indice = 0;
        }
    }

    /// A maioria das amostras está em nível alto?
    fn maioria(&self) -> bool {
        let soma: usize = self.amostras.iter().filter(|&&a| a).count();
        soma > Self::N / 2
    }
}

/// Quanto duas leituras podem diferir e ainda contarem como a mesma.
///
/// Serve para descartar a amostra solitária que cai no meio da subida
/// de tensão de um botão. Sem isso, apertar *Mais Secas* às vezes era
/// lido como *Pausa* — e Pausa desliga a máquina. Ver `botoes.rs`.
const TOLERANCIA_AD: u16 = 20;

/// Lê o seletor, os botões e o pressostato a cada 81 ms.
pub async fn amostragem(e: &Entradas, adc_periferico: &Adc) -> Infallible {
    let mut detector = DetectorMudanca::novo(e.trilhas_encoder());
    let mut pressostato = FiltroPressostato::novo(e.pressostato());

    // Última leitura crua do ADC, para confirmar a próxima.
    let mut ad_anterior: Option<u16> = None;

    let mut gate = PeriodicGate::from(Millis(AMOSTRAGEM_MS));
    loop {
        gate.next_time().await;

        // Girou o seletor? Guarda a posição nova; a máquina consome.
        let trilhas = e.trilhas_encoder();
        if detector.mudou(trilhas) {
            leituras::marca_encoder(encoder::posicao(trilhas));
        }

        // Botões do painel, pelo divisor resistivo em PA6. Se o ADC não
        // responder, mantém a última leitura boa.
        //
        // A leitura só é publicada depois de **se repetir**: quando um
        // contato fecha, a tensão sobe gradualmente, e uma amostra
        // isolada no meio dessa subida pode cair na faixa de outro
        // botão. Duas leituras iguais seguidas significam patamar, não
        // travessia.
        match adc::converte(adc_periferico) {
            Some(valor) => {
                diag::registra_leitura(valor);

                let estavel = ad_anterior
                    .map(|anterior| valor.abs_diff(anterior) <= TOLERANCIA_AD)
                    .unwrap_or(false);
                if estavel {
                    leituras::set_ad(valor);
                }
                ad_anterior = Some(valor);
            }
            None => diag::registra_falha(),
        }

        // Pressostato: filtra e inverte — nível baixo significa cheio.
        pressostato.amostra(e.pressostato());
        leituras::set_nivel_agua(!pressostato.maioria());

        evento::acorda();
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: MÁQUINA DE ESTADOS
// ═════════════════════════════════════════════════════════════════════════════

/// Animação de partida: pisca os três LEDs 4 vezes.
const PISCADAS_BOOT: u8 = 4;
const PISCA_BOOT_MS: u64 = 500;
/// Espera para a chave de modelo e o encoder assentarem antes da
/// primeira leitura.
const ESPERA_BOOT_MS: u64 = 300;

/// Executa o ciclo de lavagem: partida e depois uma passada por sinal.
///
/// A partida é a mesma do `main()` do C++: espera, lê a chave de
/// modelo, decide entre auto-teste e retomada do ciclo gravado, e
/// pisca os LEDs.
pub async fn maquina(s: &Saidas, e: &Entradas) -> Infallible {
    let mut m = Maquina::nova(s, e);

    sleep_for(Millis(ESPERA_BOOT_MS)).await;

    // Chave na posição de auto-teste entra direto no teste de linha;
    // caso contrário, retoma o ciclo que estava gravado na flash.
    if modelo::do_chave(e.chave_modelo()) == modelo::AUTO_TESTE {
        m.definir_etapa(Etapa::Teste);
    } else {
        m.seleciona_programa_inicial();
    }

    for _ in 0..PISCADAS_BOOT {
        s.pisca_led_interface();
        sleep_for(Millis(PISCA_BOOT_MS)).await;
    }
    sleep_for(Millis(ESPERA_BOOT_MS)).await;

    loop {
        evento::espera().await;
        m.passo().await;
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: GRAVAÇÃO NA FLASH
// ═════════════════════════════════════════════════════════════════════════════

/// Grava o estado do ciclo quando a máquina pede.
///
/// Fica aqui, e não dentro da máquina, porque apagar e gravar a página
/// trava o núcleo por alguns milissegundos. Nesta tarefa isso acontece
/// entre duas passadas, sem atrasar a decisão da etapa.
pub async fn gravacao(fmc: &Fmc) -> Infallible {
    loop {
        let estado = estado_salvo::proximo_pedido().await;
        if !estado_salvo::grava(fmc, estado) {
            sleep_for(Millis(100)).await;
            estado_salvo::repete_gravacao(estado);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: WATCHDOG
// ═════════════════════════════════════════════════════════════════════════════

/// Alimenta o FWDGT a cada 1 s. O estouro é de 2,8 s, o mesmo do IWDG
/// no firmware C++: se qualquer tarefa travar, o executor para de
/// rodar, esta tarefa para de alimentar e o MCU reinicia sozinho.
pub async fn watchdog(wdt_periferico: &Fwdgt, rcu: &Rcu) -> Infallible {
    wdt::inicia(wdt_periferico, rcu).await;

    let mut gate = PeriodicGate::from(Millis(1_000));
    loop {
        gate.next_time().await;
        wdt::alimenta(wdt_periferico);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: DIAGNOSTICO DOS BOTOES (build de bancada)
// ═════════════════════════════════════════════════════════════════════════════

/// Pisca nos LEDs o VALOR que o ADC leu quando um botao foi apertado.
///
/// Entra no lugar da maquina de estados quando o firmware e compilado
/// com a feature `diag-botoes`:
///
/// ```bash
/// cargo run --release --features diag-botoes
/// ```
///
/// Os triacs ficam desligados o tempo todo — da para rodar sem agua e
/// sem carga. O watchdog tambem fica de fora, para o `probe-rs` poder
/// parar o nucleo sem resetar a placa.
///
/// ─── Como usar ───────────────────────────────────────────────
///
/// 1. Grave, deixe a placa ligada e **nao toque em nada**. Os tres
///    LEDs piscam juntos, devagar: e o "aguardando um aperto".
/// 2. Aperte e solte **um** botao do painel.
/// 3. Os LEDs passam a soletrar o maior valor visto, em decimal.
///
/// ─── Como ler o numero ───────────────────────────────────────
///
/// Cada rodada comeca com os **tres LEDs acesos juntos** por meio
/// segundo. Depois vem tres digitos — centena, dezena, unidade —, cada
/// um separado por **um pisca do LED de mais secas**:
///
/// ```text
///   [tres acesos]  8 piscas  [separador]  2 piscas  [separador]  0 piscas
///        inicio     centena                dezena                unidade
///   =>  820
/// ```
///
/// Digito zero e nenhum pisca entre dois separadores. O valor esperado
/// para o botao de ON/OFF e algo entre **795 e 845**.
///
/// ─── O que o numero diz ──────────────────────────────────────
///
/// | O que aparece | Significa |
/// |---|---|
/// | tres LEDs **piscando juntos**, nunca soletra | nenhuma leitura saiu da faixa de repouso |
/// | tres LEDs **apagados** | o ADC nao esta convertendo |
/// | soletra um numero **estavel** entre apertos | o divisor esta bom; e so ajustar a faixa em `botoes.rs` |
/// | soletra numero **diferente a cada aperto** | entrada flutuando — confira a solda do PA6 e a continuidade ate o divisor |
///
/// O log completo das ultimas 64 leituras fica em `DIAG_LOG`; o script
/// `ferramentas/le_adc.py` despeja a sequencia.
#[cfg(feature = "diag-botoes")]
pub async fn diagnostico_botoes(s: &Saidas, adc_periferico: &Adc) -> Infallible {
    const PASSO_MS: u64 = 100;
    /// Passos sem o contador de conversoes andar para declarar o ADC
    /// parado (~1 s).
    const PACIENCIA: u8 = 10;

    // Ritmo da soletracao, em passos de PASSO_MS.
    const INICIO: u32 = 5; // tres LEDs acesos
    const APOS_INICIO: u32 = 3;
    const POR_PISCA: u32 = 4; // 2 acesos, 2 apagados
    const SEPARADOR: u32 = 3; // pisca do LED de mais secas
    const APOS_SEPARADOR: u32 = 2;
    const ENTRE_RODADAS: u32 = 15;

    s.desliga_todos_triacs();
    s.apaga_leds();

    let mut passo: u32 = 0;
    let mut valor_mostrado: u16 = 0;
    let mut ultimo_ok = diag::conversoes_ok();
    let mut paradas: u8 = 0;
    let mut varredura: u8 = 0;

    // Detector de patamar: um valor que nao se mexe por algumas
    // amostras seguidas e um botao segurado, nao ruido de transicao.
    const TOLERANCIA: u16 = 8;
    const AMOSTRAS_ESTAVEIS: u8 = 3;
    let mut referencia: u16 = 0;
    let mut estaveis: u8 = 0;

    let mut gate = PeriodicGate::from(Millis(PASSO_MS));
    loop {
        gate.next_time().await;

        // Os triacs nunca ligam neste modo.
        s.desliga_todos_triacs();

        // ── Le o canal dos botoes e guarda no log ──
        match adc::converte_canal(adc_periferico, adc::CANAL_BOTOES) {
            Some(valor) => {
                diag::registra_leitura(valor);
                diag::registra_log(valor);
                leituras::set_ad(valor);

                // Patamar: estavel por AMOSTRAS_ESTAVEIS leituras.
                if valor.abs_diff(referencia) <= TOLERANCIA {
                    estaveis = estaveis.saturating_add(1);
                    if estaveis == AMOSTRAS_ESTAVEIS {
                        diag::registra_plato(valor);
                    }
                } else {
                    referencia = valor;
                    estaveis = 0;
                }
            }
            None => diag::registra_falha(),
        }

        // Uma vez por segundo, varre os outros canais para o script
        // poder comparar — ajuda quando a suspeita volta a ser o canal.
        varredura = varredura.wrapping_add(1);
        if varredura >= 10 {
            varredura = 0;
            for canal in 0..adc::NUM_CANAIS {
                if let Some(valor) = adc::converte_canal(adc_periferico, canal) {
                    diag::registra_canal(canal as usize, valor);
                }
            }
            // A referencia interna: afere a fonte desta placa contra
            // outra, sem voltimetro.
            if let Some(valor) = adc::converte_vrefint(adc_periferico) {
                diag::registra_vrefint(valor);
            }
        }

        // ── O ADC esta convertendo? ──
        let ok = diag::conversoes_ok();
        if ok == ultimo_ok {
            paradas = paradas.saturating_add(1);
        } else {
            paradas = 0;
            ultimo_ok = ok;
        }
        if paradas >= PACIENCIA {
            s.apaga_leds();
            continue;
        }

        // ── Ja houve algum aperto? ──
        let pico = diag::pico();
        if pico == 0 {
            // Nada fora do repouso ainda: pisca os tres, devagar.
            s.led_pause(passo % 10 < 5);
            s.led_mais_secas(passo % 10 < 5);
            s.led_enxague_duplo(passo % 10 < 5);
            passo = passo.wrapping_add(1);
            continue;
        }

        // Valor novo? Recomeca a soletracao do zero.
        if pico != valor_mostrado {
            valor_mostrado = pico;
            passo = 0;
        }

        // ── Soletra o numero ──
        let centena = (valor_mostrado / 100 % 10) as u32;
        let dezena = (valor_mostrado / 10 % 10) as u32;
        let unidade = (valor_mostrado % 10) as u32;

        // Marcos da sequencia, em passos.
        let m_inicio = INICIO;
        let m_apos_inicio = m_inicio + APOS_INICIO;
        let m_centena = m_apos_inicio + centena * POR_PISCA;
        let m_sep1 = m_centena + SEPARADOR;
        let m_apos_sep1 = m_sep1 + APOS_SEPARADOR;
        let m_dezena = m_apos_sep1 + dezena * POR_PISCA;
        let m_sep2 = m_dezena + SEPARADOR;
        let m_apos_sep2 = m_sep2 + APOS_SEPARADOR;
        let m_unidade = m_apos_sep2 + unidade * POR_PISCA;
        let total = m_unidade + ENTRE_RODADAS;

        let p = passo % total;
        passo = passo.wrapping_add(1);

        // Por omissao, tudo apagado.
        let mut pause = false;
        let mut secas = false;
        let mut duplo = false;

        if p < m_inicio {
            // Inicio da rodada: os tres juntos.
            pause = true;
            secas = true;
            duplo = true;
        } else if p < m_apos_inicio {
            // pausa
        } else if p < m_centena {
            pause = (p - m_apos_inicio) % POR_PISCA < 2;
        } else if p < m_sep1 {
            secas = true;
        } else if p < m_apos_sep1 {
            // pausa
        } else if p < m_dezena {
            pause = (p - m_apos_sep1) % POR_PISCA < 2;
        } else if p < m_sep2 {
            secas = true;
        } else if p < m_apos_sep2 {
            // pausa
        } else if p < m_unidade {
            pause = (p - m_apos_sep2) % POR_PISCA < 2;
        }

        s.led_pause(pause);
        s.led_mais_secas(secas);
        s.led_enxague_duplo(duplo);
    }
}
