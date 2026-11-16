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

/// Lê o seletor, os botões e o pressostato a cada 81 ms.
pub async fn amostragem(e: &Entradas, adc_periferico: &Adc) -> Infallible {
    let mut detector = DetectorMudanca::novo(e.trilhas_encoder());
    let mut pressostato = FiltroPressostato::novo(e.pressostato());

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
        match adc::converte(adc_periferico) {
            Some(valor) => {
                leituras::set_ad(valor);
                diag::registra_leitura(valor);
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

/// Descobre em que ESCALA o ADC esta lendo e mostra nos tres LEDs.
///
/// Entra no lugar da maquina de estados quando o firmware e compilado
/// com a feature `diag-botoes`:
///
/// ```bash
/// cargo run --release --features diag-botoes
/// ```
///
/// Os triacs ficam desligados o tempo todo — da para rodar com a placa
/// na bancada, sem agua e sem carga.
///
/// ─── Como ler ────────────────────────────────────────────────
///
/// **Segure o botao de ON/OFF** enquanto olha os LEDs. As faixas dos
/// botoes sao estreitas (795..845, por exemplo), entao com um botao
/// apertado so uma escala casa — nao da empate. Em repouso a faixa e
/// larga e duas escalas podem casar ao mesmo tempo.
///
/// | Padrao dos LEDs | Significa |
/// |---|---|
/// | ON/OFF aceso **fixo** | escala ÷1 — o ADC esta como o firmware espera |
/// | mais secas aceso **fixo** | escala ÷2 — esta lendo 11 bits |
/// | enxague duplo aceso **fixo** | escala ÷4 — esta lendo **12 bits** |
/// | ON/OFF + mais secas **fixos** | escala ÷8 |
/// | os tres **piscando juntos** | nenhuma escala casa, mas o valor varia |
/// | **luz correndo** nos tres | o valor nao muda desde a partida |
/// | os tres **apagados** | o ADC nao esta convertendo |
///
/// As quatro primeiras linhas sao diagnostico fechado: a correcao e no
/// `DRES` de [`crate::mcu::adc`], ou dividir o resultado por essa
/// escala. As tres ultimas sao defeito de outra natureza — valor
/// travado, canal errado ou periferico sem converter.
#[cfg(feature = "diag-botoes")]
pub async fn diagnostico_botoes(s: &Saidas) -> Infallible {
    /// Escalas testadas, da mais provavel para a menos.
    const ESCALAS: [u16; 4] = [1, 2, 4, 8];

    const PERIODO_MS: u64 = 120;
    /// Passadas sem o contador de conversoes andar para declarar o ADC
    /// parado (~1,2 s).
    const PACIENCIA: u8 = 10;
    /// Abaixo desta variacao total, o valor e considerado travado.
    const AMPLITUDE_MINIMA: u16 = 10;

    /// Em que escala o valor cai numa faixa conhecida.
    ///
    /// Tenta primeiro as faixas dos botoes, que sao estreitas e nao
    /// dao empate; so depois o repouso, que e largo.
    fn escala(ad: u16) -> Option<u16> {
        if let Some(d) = ESCALAS
            .into_iter()
            .find(|&d| crate::botoes::identifica(ad / d).is_some())
        {
            return Some(d);
        }
        ESCALAS.into_iter().find(|&d| crate::botoes::em_repouso(ad / d))
    }

    s.desliga_todos_triacs();
    s.apaga_leds();

    let mut ultimo_ok = diag::conversoes_ok();
    let mut paradas: u8 = 0;
    let mut fase: u8 = 0;

    let mut gate = PeriodicGate::from(Millis(PERIODO_MS));
    loop {
        gate.next_time().await;

        // Os triacs nunca ligam neste modo.
        s.desliga_todos_triacs();
        fase = (fase + 1) % 4;
        let pisca = fase < 2;

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

        // ── O valor se move? ──
        if diag::amplitude() < AMPLITUDE_MINIMA {
            // Luz correndo: valor travado desde a partida.
            s.led_pause(fase == 0);
            s.led_mais_secas(fase == 1);
            s.led_enxague_duplo(fase == 2);
            continue;
        }

        // ── Em que escala o ADC esta lendo? ──
        match escala(leituras::ad()) {
            Some(1) => {
                s.led_pause(true);
                s.led_mais_secas(false);
                s.led_enxague_duplo(false);
            }
            Some(2) => {
                s.led_pause(false);
                s.led_mais_secas(true);
                s.led_enxague_duplo(false);
            }
            Some(4) => {
                s.led_pause(false);
                s.led_mais_secas(false);
                s.led_enxague_duplo(true);
            }
            Some(_) => {
                s.led_pause(true);
                s.led_mais_secas(true);
                s.led_enxague_duplo(false);
            }
            // Nenhuma escala casa, mas o valor varia.
            None => {
                s.led_pause(pisca);
                s.led_mais_secas(pisca);
                s.led_enxague_duplo(pisca);
            }
        }
    }
}
