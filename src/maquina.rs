//! =============================================================
//! Máquina de estados do ciclo de lavagem
//!
//! Porte da `MaquinaDeLavarFacade` do firmware C++ de fábrica. As
//! etapas, os tempos e as condições são os mesmos; o que mudou é a
//! forma de esperar: onde o C++ chamava `LL_mDelay()` e travava o
//! núcleo, aqui as funções são `async` e devolvem o controle ao
//! executor do lilos, então o relógio do ciclo e o watchdog continuam
//! andando.
//!
//! ─── As sete etapas ──────────────────────────────────────────
//!
//! ```text
//!   ENCHER ──→ AGITAR ──→ MOLHO ──→ ENXAGUAR ──→ CENTRIFUGAR ──→ DESLIGAR
//!     ↑          │          │           │
//!     └──────────┴──────────┘           │   (nível de água caiu:
//!        sem água, volta a encher       │    volta a encher)
//!                                       │
//!   TESTE: entra só quando a chave de modelo está na posição 7
//! ```
//!
//! O número de cada etapa é o que vai para a flash (`flash_etapa`),
//! então a ordem do `enum` não pode mudar sem invalidar o estado
//! gravado nas placas que já estão em campo.
//!
//! ─── O que para o ciclo ──────────────────────────────────────
//!   tampa aberta       → triacs desligados, relógio parado
//!   pausa (ON/OFF)     → idem, até apertar de novo
//!   seletor girado     → recomeça no programa novo
//!   modelo inválido    → LEDs piscando, nada liga
//!   tempo estourado    → LEDs piscando (não enche, não drena)
//! =============================================================

use lilos::time::{sleep_for, Millis, TickTime};

use crate::agita::{self, Padrao};
use crate::botoes::{self, Botao};
use crate::erro::Erro;
use crate::estado_salvo::{self, EstadoSalvo};
use crate::leituras;
use crate::modelo;
use crate::placa::{Entradas, Saidas};
use crate::relogio;
use crate::tabelas;

/// Etapas do ciclo. O valor numérico é o que se grava na flash.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Etapa {
    Encher = 0,
    Agitar = 1,
    Molho = 2,
    Enxaguar = 3,
    Centrifugar = 4,
    Desligar = 5,
    Teste = 6,
    /// Protecao disparou: tudo desligado e o painel piscando o codigo.
    ///
    /// Nunca e gravada na flash. Depois de uma queda de energia a
    /// maquina retoma a etapa que estava rodando e, se a falha for
    /// real, ela dispara de novo em minutos.
    Erro = 7,
}

/// Dentro da etapa MOLHO, alterna entre parado e agitando.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ModoMolho {
    Molho,
    Agitar,
}

/// Período do pisca de tampa aberta — o TIM17 do C++
/// (24 MHz ÷ 24000 ÷ 301).
const PISCA_MS: u64 = 301;

/// Espera com as saídas desligadas depois de girar o seletor ou de
/// fechar a tampa.
const ESPERA_RETOMADA_MS: u64 = 2_000;

/// Quanto o painel fica aceso em cada piscada do codigo de erro.
const ERRO_ACESO_MS: u64 = 250;
/// Quanto fica apagado entre duas piscadas do codigo.
const ERRO_APAGADO_MS: u64 = 250;
/// Pausa escura entre duas repeticoes do codigo.
///
/// Tres vezes mais longa que o intervalo entre piscadas, senao nao se
/// sabe onde a contagem comeca.
const ERRO_PAUSA_MS: u64 = 1_500;

/// Quantas vezes o ciclo pode voltar a encher por queda de nivel antes
/// de virar [`Erro::Nivel`].
///
/// Uma recuperacao e normal — espuma assentando, por exemplo, e o que
/// o firmware C++ faz sempre. Tres seguidas na mesma lavagem e
/// vazamento.
const LIMITE_RECUPERACOES_NIVEL: u8 = 3;

/// Teto de tempo da etapa de enchimento: 50 minutos.
const LIMITE_ENCHER_S: u16 = 3_000;
/// Teto de tempo da drenagem: 6 minutos.
const LIMITE_DRENAR_S: u16 = 360;

// ═════════════════════════════════════════════════════════════════════════════
// FASES — intervalos de tempo com uma ação cada
// ═════════════════════════════════════════════════════════════════════════════

/// O que fazer num intervalo de segundos da etapa.
#[derive(Clone, Copy)]
enum Acao {
    /// Liga ou desliga o pulso de motor (PA9).
    Pulso(bool),
    /// Liga ou desliga a bomba.
    Bomba(bool),
    /// Liga ou desliga o atuador.
    Atuador(bool),
}

#[derive(Clone, Copy)]
struct Fase {
    inicio: u16,
    fim: u16,
    acao: Acao,
}

const fn fase(inicio: u16, fim: u16, acao: Acao) -> Fase {
    Fase { inicio, fim, acao }
}

/// Pulsos de motor da drenagem do enxágue (121 s a 406 s).
///
/// São solavancos curtos para desgrudar a roupa da parede do cesto
/// antes da centrifugação, seguidos da centrifugação longa.
const FASES_DRENAGEM: [Fase; 16] = [
    fase(121, 123, Acao::Pulso(true)),
    fase(124, 145, Acao::Pulso(false)),
    fase(146, 150, Acao::Pulso(true)),
    fase(151, 172, Acao::Pulso(false)),
    fase(173, 177, Acao::Pulso(true)),
    fase(178, 192, Acao::Pulso(false)),
    fase(193, 197, Acao::Pulso(true)),
    fase(198, 209, Acao::Pulso(false)),
    fase(210, 214, Acao::Pulso(true)),
    fase(215, 221, Acao::Pulso(false)),
    fase(222, 226, Acao::Pulso(true)),
    fase(227, 233, Acao::Pulso(false)),
    fase(234, 238, Acao::Pulso(true)),
    fase(239, 245, Acao::Pulso(false)),
    fase(246, 396, Acao::Pulso(true)),
    fase(397, 406, Acao::Pulso(false)),
];

/// Fases da centrifugação final (0 s a 991 s).
const FASES_CENTRIFUGA: [Fase; 21] = [
    fase(0, 90, Acao::Bomba(true)),
    fase(91, 94, Acao::Pulso(true)),
    fase(95, 125, Acao::Pulso(false)),
    fase(126, 129, Acao::Pulso(true)),
    fase(130, 160, Acao::Pulso(false)),
    fase(161, 166, Acao::Pulso(true)),
    fase(167, 177, Acao::Pulso(false)),
    fase(178, 184, Acao::Pulso(true)),
    fase(185, 191, Acao::Pulso(false)),
    fase(192, 197, Acao::Pulso(true)),
    fase(198, 203, Acao::Pulso(false)),
    fase(204, 209, Acao::Pulso(true)),
    fase(210, 215, Acao::Pulso(false)),
    fase(216, 221, Acao::Pulso(true)),
    fase(222, 226, Acao::Pulso(false)),
    fase(227, 232, Acao::Pulso(true)),
    fase(233, 237, Acao::Pulso(false)),
    // Centrifugação longa; o tempo é encurtado mais abaixo conforme
    // o modelo e o botão "roupas mais secas".
    fase(238, 953, Acao::Pulso(true)),
    fase(954, 972, Acao::Pulso(false)),
    fase(973, 981, Acao::Bomba(false)),
    fase(982, 991, Acao::Atuador(false)),
];

// ═════════════════════════════════════════════════════════════════════════════
// A MÁQUINA
// ═════════════════════════════════════════════════════════════════════════════

pub struct Maquina<'a> {
    s: &'a Saidas,
    e: &'a Entradas,

    etapa: Etapa,
    /// Posição do encoder: qual programa de lavagem está escolhido (0..15).
    tipo_lavagem: u8,
    /// Agitação em modo turbo (o modelo 5 amarra isto ao enxágue duplo).
    bt_modo_agita: bool,

    /// Etapa como vai para a flash.
    flash_etapa: u8,
    /// Centrifuga mais tempo.
    bt_mais_secas: bool,
    /// Faz dois enxágues em vez de um.
    bt_enxague_duplo: bool,
    /// Em pausa: LED de ON/OFF piscando e saídas desligadas.
    led_pause: bool,

    /// Erro de interface (modelo inválido ou falha de flash): LEDs piscam.
    interface_erro: bool,
    /// Tampa já confirmada fechada neste ciclo.
    tampa_fechada: bool,
    /// Houve abertura de tampa durante o ciclo.
    tampa_aberta_durante_ciclo: bool,
    contagem_retomada_tampa_ativa: bool,
    inicio_retomada_tampa: TickTime,
    /// A pausa atual veio de girar o seletor.
    pausa_por_encoder: bool,
    /// Nos modelos ≠ 6, girar o seletor só espera 2 s e recomeça.
    espera_automatica_encoder_ativa: bool,
    inicio_espera_encoder: TickTime,

    /// Zerar o relógio na próxima passada, depois de drenar toda a água.
    reinicia_tempo_apos_drenar_agua: bool,
    /// Lavagem econômica: pausa na entrada do enxágue para reaproveitar água.
    reutilizacao_de_agua: bool,

    /// Última leitura do ADC dos botões.
    ad_button: u16,
    /// Subetapa do enxágue: 1 drenar, 2 entrar água, 3 agitar.
    enxaguar_etapas: u8,
    /// Qual enxágue está sendo executado (1 ou 2).
    num_enxagues_executado: u8,
    /// Nível de água atingido (pressostato filtrado e invertido).
    nivel_agua: bool,

    /// Em auto-teste de fábrica.
    auto_teste: bool,
    /// Passo do auto-teste: 1 válvula, 2 agita, 3 atuador+bomba,
    /// 4 centrifuga, 5 fim.
    avanca_teste: u8,

    // ─── Pisca de tampa aberta (TIM17 no C++) ────────────────────────────────
    pisca_ligado: bool,
    ultimo_pisca: TickTime,

    // ─── Codigo de erro piscado ──────────────────────────────────────────────
    /// Qual protecao disparou, quando a etapa e [`Etapa::Erro`].
    erro_atual: Option<Erro>,
    /// Quando o erro comecou — da a fase do pisca do codigo.
    inicio_erro: TickTime,
    /// Estado atual do pisca, para so escrever no GPIO na transicao.
    erro_aceso: bool,
    /// Quantas vezes esta lavagem voltou a encher por queda de nivel.
    recuperacoes_de_nivel: u8,

    // ─── Estado que no C++ são `static` dentro das funções ───────────────────
    molho_ciclo_atual: u16,
    molho_modo: ModoMolho,
    molho_modo_prev: ModoMolho,
    encher_ultimo_segundo: u16,
    encher_contador_incremento: u16,
    /// Se os LEDs ja foram apagados nesta falha de flash.
    flash_apagou_leds: bool,
    /// `desligaLeds` do `getErroModeloMaquina`.
    modelo_desliga_leds: bool,
    /// `acaoProcessada` do `gerenciaInterface`.
    acao_processada: bool,
    teste_inicializado: bool,
    teste_blink: u8,
    teste_etapa_anterior: u8,
}

impl<'a> Maquina<'a> {
    pub fn nova(s: &'a Saidas, e: &'a Entradas) -> Self {
        let agora = TickTime::now();
        Self {
            s,
            e,
            etapa: Etapa::Encher,
            tipo_lavagem: 0,
            bt_modo_agita: false,
            flash_etapa: 0,
            bt_mais_secas: false,
            bt_enxague_duplo: false,
            led_pause: false,
            interface_erro: false,
            tampa_fechada: false,
            tampa_aberta_durante_ciclo: false,
            contagem_retomada_tampa_ativa: false,
            inicio_retomada_tampa: agora,
            pausa_por_encoder: false,
            espera_automatica_encoder_ativa: false,
            inicio_espera_encoder: agora,
            reinicia_tempo_apos_drenar_agua: false,
            reutilizacao_de_agua: false,
            ad_button: 0,
            enxaguar_etapas: 1,
            num_enxagues_executado: 1,
            nivel_agua: false,
            auto_teste: false,
            avanca_teste: 0,
            pisca_ligado: false,
            ultimo_pisca: agora,
            erro_atual: None,
            inicio_erro: agora,
            erro_aceso: false,
            recuperacoes_de_nivel: 0,
            molho_ciclo_atual: 1,
            molho_modo: ModoMolho::Molho,
            molho_modo_prev: ModoMolho::Molho,
            encher_ultimo_segundo: 0,
            encher_contador_incremento: 0,
            flash_apagou_leds: true,
            modelo_desliga_leds: false,
            acao_processada: false,
            teste_inicializado: false,
            teste_blink: 0,
            teste_etapa_anterior: 0xFF,
        }
    }

    /// Etapa atual. Usado em depuracao e em testes de bancada.
    #[allow(dead_code)]
    pub fn etapa(&self) -> Etapa {
        self.etapa
    }

    pub fn definir_etapa(&mut self, nova: Etapa) {
        self.etapa = nova;
    }

    // ═════════════════════════════════════════════════════════════════════════
    // UMA PASSADA — equivale a uma volta do laço principal do C++
    // ═════════════════════════════════════════════════════════════════════════

    /// Lê o seletor e os botões e executa a etapa atual.
    ///
    /// É o par `selecionaProgramasDeLavagem()` + `ExecutarEtapa()` do
    /// laço `while(1)` do firmware C++.
    pub async fn passo(&mut self) {
        // A falha de flash também deve interromper saídas já ligadas,
        // inclusive no auto-teste. Ao recuperar, refaz a estabilização.
        if estado_salvo::falhou() {
            self.s.desliga_todos_triacs();
            agita::liga_timer(false);
            relogio::liga(false);
            relogio::reset_atrasos();
            self.tampa_fechada = false;
            self.interface_erro = true;
            self.pisca_falha_flash().await;
            return;
        }

        self.nivel_agua = leituras::nivel_agua();

        // Girou o seletor? No auto-teste avança o passo do teste; fora
        // dele, recomeça o ciclo no programa novo.
        if leituras::consome_encoder_mudou() {
            if self.auto_teste {
                self.avanca_teste = self.avanca_teste.saturating_add(1);
                relogio::reset_tudo();
            } else {
                self.selecionar_nova_lavagem().await;
            }
        }

        self.ad_button = leituras::ad();
        self.gerencia_interface();

        self.executar_etapa().await;
    }

    async fn executar_etapa(&mut self) {
        match self.etapa {
            Etapa::Encher => self.encher().await,
            Etapa::Agitar => self.agitar().await,
            Etapa::Molho => self.molho().await,
            Etapa::Enxaguar => self.enxaguar().await,
            Etapa::Centrifugar => self.centrifugar().await,
            Etapa::Desligar => self.desligar(),
            Etapa::Teste => self.teste(),
            Etapa::Erro => self.pisca_codigo_de_erro(),
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // PROTECOES — entrar em erro e piscar o codigo
    // ═════════════════════════════════════════════════════════════════════════

    /// Trava a maquina com tudo desligado e comeca a piscar o codigo.
    ///
    /// Fica assim ate alguem apertar ON/OFF ou girar o seletor. O
    /// estado NAO vai para a flash: depois de uma queda de energia a
    /// maquina retoma a etapa que estava rodando, e se a falha for real
    /// ela dispara de novo em minutos.
    fn entra_em_erro(&mut self, erro: Erro) {
        self.s.desliga_todos_triacs();
        agita::liga_timer(false);
        agita::zera();
        relogio::liga(false);
        relogio::reset_tudo();

        self.s.apaga_leds();
        self.pisca_desliga();

        self.erro_atual = Some(erro);
        self.inicio_erro = TickTime::now();
        self.erro_aceso = false;
        self.etapa = Etapa::Erro;
    }

    /// Pisca o painel inteiro no codigo do erro: N piscadas, pausa, repete.
    ///
    /// ```text
    /// Drenar (4 piscadas):
    ///   ▀▁▀▁▀▁▀▁________________▀▁▀▁▀▁▀▁________________ ...
    ///   └── 4 × 500 ms ──┘└ 1,5 s ┘
    /// ```
    ///
    /// Os tres LEDs piscam juntos para a contagem ser visivel de longe
    /// e nao se confundir com o pisca de tampa aberta, que e so o LED
    /// de ON/OFF.
    ///
    /// A fase sai do tempo decorrido, nao de um contador: nao importa
    /// de quanto em quanto tempo esta funcao e chamada. O painel so e
    /// escrito quando o nivel muda.
    fn pisca_codigo_de_erro(&mut self) {
        let Some(erro) = self.erro_atual else {
            // Sem erro guardado nao ha codigo para piscar: desliga.
            self.definir_etapa_desligado();
            return;
        };

        self.s.desliga_todos_triacs();

        const POR_PISCADA: u64 = ERRO_ACESO_MS + ERRO_APAGADO_MS;
        let trem = erro.piscadas() * POR_PISCADA;
        let ciclo = trem + ERRO_PAUSA_MS;

        let fase = self.inicio_erro.elapsed().0 % ciclo;
        let aceso = fase < trem && (fase % POR_PISCADA) < ERRO_ACESO_MS;

        if aceso == self.erro_aceso {
            return;
        }
        self.erro_aceso = aceso;
        self.s.led_pause(aceso);
        self.s.led_mais_secas(aceso);
        self.s.led_enxague_duplo(aceso);
    }

    /// Conta uma queda de nivel e devolve `true` se virou erro.
    ///
    /// O firmware C++ simplesmente volta a encher, sempre. Uma
    /// recuperacao e normal; tres na mesma lavagem e vazamento, e e aí
    /// que isto para a maquina.
    fn queda_de_nivel_virou_erro(&mut self) -> bool {
        self.recuperacoes_de_nivel = self.recuperacoes_de_nivel.saturating_add(1);
        if self.recuperacoes_de_nivel >= LIMITE_RECUPERACOES_NIVEL {
            self.entra_em_erro(Erro::Nivel);
            return true;
        }
        false
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: ENCHER
    // ═════════════════════════════════════════════════════════════════════════

    async fn encher(&mut self) {
        let total = relogio::total_segundos();
        if total >= LIMITE_ENCHER_S {
            self.entra_em_erro(Erro::Encher);
            return;
        }

        let m = self.obter_modelo_atual().await;
        if self.interface_erro {
            return;
        }

        let tampa_aberta = leituras::tampa_aberta(m);
        if self.tratar_tampa_aberta(tampa_aberta, self.led_pause, m) {
            self.pisca_liga();
            return;
        }
        if !self.tratar_tampa_fechada(m) {
            return;
        }

        if relogio::delay_5() < relogio::MIN_DELAY_5 {
            self.s.desliga_todos_triacs();
            return;
        }

        // Conta as viradas de segundo — é a base dos pulsos do modelo 1.
        if total < 8 {
            self.encher_contador_incremento = 0;
        }
        if self.encher_ultimo_segundo != total {
            self.encher_ultimo_segundo = total;
            self.encher_contador_incremento = self.encher_contador_incremento.saturating_add(1);
        }

        if self.nivel_agua {
            self.transicionar_para(Etapa::Agitar);
            return;
        }

        self.s.valvula_sabao(true);

        // Modelo 1 nos programas 12 e 13: além da válvula, pulsa o motor
        // para a roupa assentar enquanto a água entra.
        let tipo_especial = self.tipo_lavagem == 12 || self.tipo_lavagem == 13;
        if m == 1 && tipo_especial {
            const DELAY_SABAO_S: u16 = 13;
            if relogio::delay_25() < DELAY_SABAO_S {
                self.s.atuador(true);
                return;
            }

            self.s.atuador(true);
            if self.encher_contador_incremento == 180 {
                self.s.motor_pulse(true);
            } else if self.encher_contador_incremento <= 193 {
                self.s.motor_pulse(false);
            } else {
                self.encher_contador_incremento = 179;
            }
        }
        // Modelos 2–6: basta manter a válvula aberta.
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: AGITAR
    // ═════════════════════════════════════════════════════════════════════════

    async fn agitar(&mut self) {
        let m = self.obter_modelo_atual().await;
        if self.interface_erro {
            return;
        }

        let tempo_agitar = tabelas::tempo_agitar(m, self.tipo_lavagem);

        let tampa_aberta = leituras::tampa_aberta(m);
        if self.tratar_tampa_aberta(tampa_aberta, self.led_pause, m) {
            self.pisca_liga();
            return;
        }
        if !self.tratar_tampa_fechada(m) {
            return;
        }

        if relogio::delay_5() < relogio::MIN_DELAY_5 {
            self.s.desliga_todos_triacs();
            return;
        }

        // Perdeu a agua: o C++ simplesmente volta a encher. Aqui
        // tambem, mas contando — tres vezes na mesma lavagem e
        // vazamento, nao espuma assentando.
        if !self.nivel_agua {
            if self.queda_de_nivel_virou_erro() {
                return;
            }
            self.transicionar_para(Etapa::Encher);
            return;
        }

        let total = relogio::total_segundos();
        if total >= tempo_agitar {
            self.transicionar_para(Etapa::Molho);
        } else {
            if m == 5 {
                self.bt_modo_agita = self.bt_enxague_duplo;
            }
            agita::etapa_agitar(self.s, m, self.tipo_lavagem, self.bt_modo_agita);
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: MOLHO
    // ═════════════════════════════════════════════════════════════════════════

    async fn molho(&mut self) {
        let m = self.obter_modelo_atual().await;
        if self.interface_erro {
            return;
        }

        let tampa_aberta = leituras::tampa_aberta(m);
        if self.tratar_tampa_aberta(tampa_aberta, self.led_pause, m) {
            self.pisca_liga();
            return;
        }
        if !self.tratar_tampa_fechada(m) {
            return;
        }

        if relogio::delay_5() < relogio::MIN_DELAY_5 {
            self.s.desliga_todos_triacs();
            return;
        }

        // Perdeu a agua: o C++ simplesmente volta a encher. Aqui
        // tambem, mas contando — tres vezes na mesma lavagem e
        // vazamento, nao espuma assentando.
        if !self.nivel_agua {
            if self.queda_de_nivel_virou_erro() {
                return;
            }
            self.transicionar_para(Etapa::Encher);
            return;
        }

        // Modelo sem tabela de molho (7 ou 8): a etapa não faz nada,
        // igual ao early-return do C++ quando `totalConfigs == 0`.
        let Some(cfg) = tabelas::config_molho(m, self.tipo_lavagem) else {
            return;
        };

        let total = relogio::total_segundos();

        match self.molho_modo {
            ModoMolho::Molho => {
                // Primeira passada depois de vir do modo AGITAR.
                if self.molho_modo_prev != ModoMolho::Molho {
                    agita::desliga_motores(self.s);
                    agita::liga_timer(false);
                    agita::zera();
                }

                if total >= cfg.tempo_molho {
                    relogio::set_total_segundos(0);
                    self.molho_ciclo_atual = self.molho_ciclo_atual.saturating_add(1);

                    if self.molho_ciclo_atual >= cfg.ciclos {
                        self.molho_ciclo_atual = 1;
                        self.aplica_modo_molho(ModoMolho::Molho);
                        self.avancar_para_enxague(m);
                        return;
                    }

                    self.aplica_modo_molho(ModoMolho::Agitar);
                } else {
                    self.aplica_modo_molho(ModoMolho::Molho);
                }
            }
            ModoMolho::Agitar => {
                if m == 5 {
                    self.bt_modo_agita = self.bt_enxague_duplo;
                }

                let turbo_agora =
                    tabelas::ciclo_e_turbo(m, self.tipo_lavagem, self.molho_ciclo_atual);
                agita::etapa_molho(
                    self.s,
                    m,
                    self.tipo_lavagem,
                    self.bt_modo_agita,
                    turbo_agora,
                );

                if total >= cfg.tempo_agitar {
                    relogio::set_total_segundos(0);
                    self.aplica_modo_molho(ModoMolho::Molho);
                } else {
                    self.aplica_modo_molho(ModoMolho::Agitar);
                }
            }
        }
    }

    /// Troca o modo do molho, ligando ou parando o contador de agitação
    /// **só na borda** — como o `aplicaMudancaDeModo` do C++.
    fn aplica_modo_molho(&mut self, novo: ModoMolho) {
        if self.molho_modo == novo {
            return;
        }
        self.molho_modo_prev = self.molho_modo;
        self.molho_modo = novo;

        match novo {
            ModoMolho::Agitar => {
                agita::zera();
                agita::liga_timer(true);
            }
            ModoMolho::Molho => {
                agita::desliga_motores(self.s);
                agita::liga_timer(false);
                agita::zera();
            }
        }
    }

    fn avancar_para_enxague(&mut self, m: u8) {
        self.enxaguar_etapas = 1; // começa drenando
        self.reutilizacao_de_agua = tabelas::reutiliza_agua(m, self.tipo_lavagem);
        self.transicionar_para(Etapa::Enxaguar);
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: ENXAGUAR
    // ═════════════════════════════════════════════════════════════════════════

    async fn enxaguar(&mut self) {
        let m = self.obter_modelo_atual().await;
        if self.interface_erro {
            return;
        }

        let total = relogio::total_segundos();
        let tampa_aberta = leituras::tampa_aberta(m);

        // Abrir a tampa cancela a reutilização de água: é o gesto de
        // quem tirou a água e quer que o ciclo siga.
        if tampa_aberta {
            self.reutilizacao_de_agua = false;
        }

        if self.tratar_tampa_aberta_enxague(
            tampa_aberta,
            self.led_pause,
            self.reutilizacao_de_agua,
            m,
        ) {
            self.pisca_liga();
            return;
        }
        if !self.tratar_tampa_fechada(m) {
            return;
        }

        if relogio::delay_5() < relogio::MIN_DELAY_5 {
            self.s.desliga_todos_triacs();
            return;
        }

        // No modelo 5 quem manda no enxágue duplo é o botão "mais secas".
        let enxague_duplo = if m == 5 {
            self.bt_mais_secas
        } else {
            self.bt_enxague_duplo
        };
        let num_enxagues: u8 = if enxague_duplo { 2 } else { 1 };

        match self.enxaguar_etapas {
            2 => self.enxague_entrar_agua(m, total, num_enxagues).await,
            3 => self.enxague_agitar(m, total, num_enxagues),
            // 1 e qualquer valor inesperado: drenar.
            _ => self.enxague_drenar(m, total, num_enxagues).await,
        }
    }

    /// Subetapa 1 do enxágue: tirar a água da lavagem.
    async fn enxague_drenar(&mut self, m: u8, total: u16, num_enxagues: u8) {
        let bomba_sozinha = m == 3 || m == 4;

        // O pressostato pode voltar a indicar água durante os pulsos.
        // Corta os dois sentidos antes de qualquer retorno, inclusive
        // durante o atraso de estabilização.
        if self.nivel_agua {
            agita::desliga_motores(self.s);
            agita::liga_timer(false);
        }

        // Atraso longo de estabilização.
        if relogio::delay_25() < relogio::MIN_DELAY_25 {
            if (70..=85).contains(&total) {
                self.s.valvula_sabao(true);
                self.s.atuador(false);
                self.s.bomba(false);
            } else if bomba_sozinha {
                self.s.bomba(true);
            } else {
                self.s.atuador(true);
                self.s.bomba(true);
            }
            return;
        }

        // Ainda tem água: continua drenando e marca para zerar o relógio
        // quando acabar.
        if self.nivel_agua {
            if total >= LIMITE_DRENAR_S {
                self.entra_em_erro(Erro::Drenar);
                return;
            }
            if bomba_sozinha {
                self.s.bomba(true);
            } else {
                self.s.atuador(true);
                self.s.bomba(true);
            }
            self.reinicia_tempo_apos_drenar_agua = true;
            return;
        }

        if self.reinicia_tempo_apos_drenar_agua {
            relogio::set_total_segundos(0);
            self.s.motor_c(false);
            self.s.motor_a(false);
            self.reinicia_tempo_apos_drenar_agua = false;
            return;
        }

        // Primeiros 2 minutos de drenagem.
        if total <= 120 {
            if (70..=85).contains(&total) {
                self.s.valvula_sabao(true);
                self.s.atuador(false);
                self.s.bomba(false);
            } else if bomba_sozinha {
                self.s.bomba(true);
                self.s.valvula_sabao(false);
            } else {
                self.s.valvula_sabao(false);
                self.s.atuador(true);
                self.s.bomba(true);
            }
            return;
        }

        // Com dois enxágues, o primeiro não faz a centrifugação longa:
        // pula direto para o fim das fases.
        if !bomba_sozinha
            && total >= 250
            && num_enxagues == 2
            && self.num_enxagues_executado == 1
        {
            relogio::set_total_segundos(397);
            self.num_enxagues_executado = 2;
            self.s.motor_pulse(true);
            return;
        }

        if self.executa_fase(&FASES_DRENAGEM, total) {
            return;
        }

        // Fora de todas as fases: acabou a drenagem.
        self.s.desliga_todos_triacs();
        relogio::reset_tudo();
        if bomba_sozinha && num_enxagues == 2 && self.num_enxagues_executado == 1 {
            self.num_enxagues_executado = 2;
        }
        self.enxaguar_etapas = 2; // entrar água
    }

    /// Subetapa 2 do enxágue: encher de água limpa.
    async fn enxague_entrar_agua(&mut self, m: u8, total: u16, num_enxagues: u8) {
        if self.nivel_agua {
            // Pequena pausa antes de fechar a válvula, para a leitura do
            // pressostato assentar.
            sleep_for(Millis(200)).await;
            self.s.valvula_sabao(false);
            self.s.valvula_amaciante(false);
            relogio::reset_tudo();
            self.enxaguar_etapas = 3; // agitar
            return;
        }

        if m == 3 || m == 4 {
            if num_enxagues == 2 && self.num_enxagues_executado == 2 {
                self.s.valvula_sabao(true);
            } else {
                // Enxágue único: entra também pelo compartimento do amaciante.
                self.s.valvula_amaciante(true);
                self.s.valvula_sabao(true);
            }
        } else {
            self.s.valvula_sabao(true);
        }

        if total >= LIMITE_ENCHER_S {
            self.entra_em_erro(Erro::Encher);
        }
    }

    /// Subetapa 3 do enxágue: agitar e deixar de molho.
    fn enxague_agitar(&mut self, m: u8, total: u16, num_enxagues: u8) {
        if m == 5 {
            self.bt_modo_agita = self.bt_enxague_duplo;
        }

        // Perdeu água no meio do enxágue: volta a drenar e enche de novo.
        if !self.nivel_agua {
            if self.queda_de_nivel_virou_erro() {
                return;
            }
            self.para_agitacao();
            relogio::reset_tudo();
            self.num_enxagues_executado = 1;
            self.enxaguar_etapas = 1;
            return;
        }

        match total {
            0..=200 => {
                agita::enxague_inicio(self.s, m, self.tipo_lavagem, self.bt_modo_agita);
                return;
            }
            201..=251 => {
                self.para_agitacao(); // molho de 50 s
                return;
            }
            252..=312 => {
                agita::enxague_final(self.s, m, self.tipo_lavagem);
                return;
            }
            313..=333 => {
                self.para_agitacao(); // molho final de 20 s
                return;
            }
            _ => {}
        }

        // Enxágue extra: volta a drenar para repetir tudo.
        if (334..=337).contains(&total)
            && num_enxagues == 2
            && self.num_enxagues_executado == 2
        {
            self.num_enxagues_executado = self.num_enxagues_executado.saturating_add(1);
            relogio::reset_tudo();
            self.enxaguar_etapas = 1;
            return;
        }

        // Fim do enxágue.
        self.num_enxagues_executado = 1;
        self.enxaguar_etapas = 1;
        self.transicionar_para(Etapa::Centrifugar);
    }

    /// Desliga tudo e para o contador de agitação (`offTriacs` do C++).
    fn para_agitacao(&self) {
        self.s.desliga_todos_triacs();
        agita::liga_timer(false);
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: CENTRIFUGAR
    // ═════════════════════════════════════════════════════════════════════════

    async fn centrifugar(&mut self) {
        let m = self.obter_modelo_atual().await;
        if self.interface_erro {
            return;
        }

        // Não basta deixar de emitir novos pulsos: o comando anterior
        // pode ter deixado o motor ligado quando o nível voltou.
        if self.nivel_agua {
            agita::desliga_motores(self.s);
            agita::liga_timer(false);
        }

        let total = relogio::total_segundos();
        let bomba_sozinha = m == 3 || m == 4;

        let tampa_aberta = leituras::tampa_aberta(m);
        if self.tratar_tampa_aberta(tampa_aberta, self.led_pause, m) {
            self.pisca_liga();
            return;
        }
        if !self.tratar_tampa_fechada(m) {
            return;
        }

        if relogio::delay_5() < relogio::MIN_DELAY_5 {
            self.s.desliga_todos_triacs();
            return;
        }

        // Atraso longo: só bomba (e atuador) até a água começar a sair.
        if relogio::delay_25() < relogio::MIN_DELAY_25 {
            if bomba_sozinha {
                self.s.bomba(true);
            } else {
                self.s.atuador(true);
                self.s.bomba(true);
            }
            return;
        }

        // Ainda tem água no cesto: não pode centrifugar.
        if self.nivel_agua {
            if total >= LIMITE_DRENAR_S {
                self.entra_em_erro(Erro::Drenar);
                return;
            }
            if bomba_sozinha {
                self.s.bomba(true);
            } else {
                self.s.atuador(true);
                self.s.bomba(true);
            }
            self.reinicia_tempo_apos_drenar_agua = true;
            return;
        }

        if self.reinicia_tempo_apos_drenar_agua {
            relogio::set_total_segundos(0);
            self.reinicia_tempo_apos_drenar_agua = false;
            return;
        }

        if total <= 90 && !bomba_sozinha {
            self.s.atuador(true);
        }

        // Encurta a centrifugação longa conforme o modelo e o botão
        // "roupas mais secas". Note que `total` é a leitura do começo
        // desta passada: o corte só vale da próxima vez, exatamente
        // como no C++.
        if (238..=953).contains(&total) {
            let mais_secas = if m == 5 { false } else { self.bt_mais_secas };

            if mais_secas {
                // 10 minutos
                if (m <= 2 || (4..=6).contains(&m)) && total >= 838 {
                    relogio::set_total_segundos(954);
                }
                // modelo 3: segue até os 12 minutos
            } else if (m <= 2 || m == 4 || m == 6) && total >= 538 {
                relogio::set_total_segundos(954); // 5 minutos
            } else if (m == 3 || m == 5) && total >= 598 {
                relogio::set_total_segundos(954); // 6 minutos
            }
        }

        if self.executa_fase(&FASES_CENTRIFUGA, total) {
            return;
        }

        self.transicionar_para(Etapa::Desligar);
    }

    /// Procura a fase que contém `total` e executa a ação dela.
    /// Devolve `false` se o tempo passou de todas.
    fn executa_fase(&self, fases: &[Fase], total: u16) -> bool {
        for f in fases {
            if total >= f.inicio && total <= f.fim {
                match f.acao {
                    Acao::Pulso(on) => self.s.motor_pulse(on),
                    Acao::Bomba(on) => self.s.bomba(on),
                    Acao::Atuador(on) => self.s.atuador(on),
                }
                return true;
            }
        }
        false
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: DESLIGAR
    // ═════════════════════════════════════════════════════════════════════════

    fn desligar(&mut self) {
        self.s.apaga_leds();
        self.limpa_erro();

        self.enxaguar_etapas = 1;
        self.num_enxagues_executado = 1;
        self.reinicia_tempo_apos_drenar_agua = false;

        self.s.desliga_todos_triacs();
        agita::liga_timer(false);
        relogio::liga(false);
        self.pisca_desliga();
        relogio::reset_tudo();

        self.tampa_fechada = false;
        self.tampa_aberta_durante_ciclo = false;
        self.contagem_retomada_tampa_ativa = false;
        self.pausa_por_encoder = false;
        self.espera_automatica_encoder_ativa = false;
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ETAPA: TESTE (auto-teste de fábrica, chave na posição 7)
    // ═════════════════════════════════════════════════════════════════════════

    /// Padrão dos LEDs em cada um dos 8 passos do pisca.
    /// Colunas: [enxágue duplo, mais secas, pause].
    const LEDS_TESTE: [[bool; 3]; 8] = [
        [false, false, false],
        [true, false, false],
        [true, true, false],
        [true, true, true],
        [false, false, false],
        [true, true, true],
        [false, false, false],
        [true, true, true],
    ];

    fn teste(&mut self) {
        if !self.teste_inicializado {
            self.pisca_liga();
            self.teste_inicializado = true;
        }

        if self.pisca_estourou() {
            self.teste_blink = self.teste_blink.saturating_add(1);
        }
        if self.teste_blink >= 8 {
            self.teste_blink = 0;
        }

        let padrao = Self::LEDS_TESTE[self.teste_blink as usize];
        self.s.led_enxague_duplo(padrao[0]);
        self.s.led_mais_secas(padrao[1]);
        self.s.led_pause(padrao[2]);

        let total = relogio::total_segundos();

        // Atraso de estabilização entre passos do teste.
        const MIN_DELAY_TESTE: u16 = 2;
        if relogio::delay_5() < MIN_DELAY_TESTE {
            self.s.motor_a(false);
            self.s.motor_c(false);
            self.s.valvula_sabao(false);
            return;
        }

        // Ao trocar de passo, desliga a saída do passo anterior — nunca
        // deixa válvula, motor, bomba e atuador ligados ao mesmo tempo.
        if self.avanca_teste != self.teste_etapa_anterior {
            self.s.desliga_todos_triacs();
            agita::liga_timer(false);
            self.teste_etapa_anterior = self.avanca_teste;
        }

        match self.avanca_teste {
            0 => {
                self.auto_teste = true;
                relogio::reset_tudo();
                self.s.desliga_todos_triacs();
            }
            1 => self.s.valvula_sabao(true),
            2 => agita::executa(self.s, Padrao::PADRAO),
            3 => {
                self.s.atuador(true);
                if total > 3 {
                    self.s.bomba(true);
                }
            }
            4 => {
                if total > 15 {
                    self.s.motor_pulse(true);
                }
            }
            5 => {
                self.teste_inicializado = false;
                self.avanca_teste = 0;
                self.s.desliga_todos_triacs();
            }
            _ => self.avanca_teste = 0,
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // TAMPA E PAUSA
    // ═════════════════════════════════════════════════════════════════════════

    /// Espera de 2 s com tudo desligado depois de girar o seletor.
    ///
    /// Nos modelos diferentes do 6, girar o encoder não deixa a máquina
    /// em pausa: ela só espera 2 s e já começa o programa novo.
    fn tratar_espera_automatica_encoder(&mut self) -> bool {
        if !self.espera_automatica_encoder_ativa {
            return false;
        }

        self.s.desliga_todos_triacs();
        relogio::liga(false);
        agita::liga_timer(false);
        relogio::reset_atrasos();

        if self.inicio_espera_encoder.elapsed() < Millis(ESPERA_RETOMADA_MS) {
            return true;
        }

        self.espera_automatica_encoder_ativa = false;

        // Os 2 s já valeram como atraso de estabilização da etapa.
        relogio::set_delay_5(relogio::MIN_DELAY_5);
        relogio::liga(true);
        false
    }

    /// Trata tampa aberta ou pausa. Devolve `true` quando a etapa deve
    /// parar aqui.
    fn tratar_tampa_aberta(&mut self, tampa_aberta: bool, em_pausa: bool, m: u8) -> bool {
        if self.tratar_espera_automatica_encoder() {
            return true;
        }

        if !(tampa_aberta || em_pausa) {
            return false;
        }

        self.s.desliga_todos_triacs();
        relogio::liga(false);
        agita::liga_timer(false);
        relogio::reset_atrasos();

        // Só uma abertura de verdade conta para a retomada.
        if tampa_aberta {
            self.tampa_fechada = false;
            self.tampa_aberta_durante_ciclo = true;
            self.contagem_retomada_tampa_ativa = false;

            // O modelo 6 exige confirmação no ON/OFF depois de fechar.
            if m == 6 {
                self.led_pause = true;
            }
        }

        if self.pisca_estourou() {
            self.s.pisca_led();
        }
        true
    }

    /// Igual ao anterior, mais a condição de lavagem econômica: o ciclo
    /// espera a tampa ser aberta para o usuário reaproveitar a água.
    fn tratar_tampa_aberta_enxague(
        &mut self,
        tampa_aberta: bool,
        em_pausa: bool,
        reutilizar_agua: bool,
        m: u8,
    ) -> bool {
        if self.tratar_espera_automatica_encoder() {
            return true;
        }

        if !(tampa_aberta || em_pausa || reutilizar_agua) {
            return false;
        }

        self.s.desliga_todos_triacs();
        relogio::liga(false);
        agita::liga_timer(false);
        relogio::reset_atrasos();

        if tampa_aberta {
            self.tampa_fechada = false;
            self.tampa_aberta_durante_ciclo = true;
            self.contagem_retomada_tampa_ativa = false;
            if m == 6 {
                self.led_pause = true;
            }
        }

        if self.pisca_estourou() {
            self.s.pisca_led();
        }
        true
    }

    /// Libera o ciclo depois de a tampa fechar. Devolve `false`
    /// enquanto a espera de segurança não terminou.
    fn tratar_tampa_fechada(&mut self, m: u8) -> bool {
        // Fora do modelo 6, a retomada é automática 2 s depois de fechar.
        if self.tampa_aberta_durante_ciclo && m != 6 {
            self.s.desliga_todos_triacs();

            if !self.contagem_retomada_tampa_ativa {
                self.inicio_retomada_tampa = TickTime::now();
                self.contagem_retomada_tampa_ativa = true;
            }

            if self.inicio_retomada_tampa.elapsed() < Millis(ESPERA_RETOMADA_MS) {
                return false;
            }

            self.tampa_aberta_durante_ciclo = false;
            self.contagem_retomada_tampa_ativa = false;

            // Os 2 s já foram cumpridos com os triacs desligados.
            relogio::set_delay_5(relogio::MIN_DELAY_5);
        }

        // No modelo 6 só se chega aqui depois de o ON/OFF tirar a pausa.
        self.tampa_aberta_durante_ciclo = false;
        self.contagem_retomada_tampa_ativa = false;

        if !self.tampa_fechada {
            relogio::liga(true);
            self.pisca_desliga();
            self.tampa_fechada = true;
            self.s.led_pause(true);
        }

        true
    }

    // ─── Pisca de tampa aberta (TIM17 no C++) ────────────────────────────────

    fn pisca_liga(&mut self) {
        if !self.pisca_ligado {
            self.pisca_ligado = true;
            self.ultimo_pisca = TickTime::now();
        }
    }

    fn pisca_desliga(&mut self) {
        self.pisca_ligado = false;
    }

    /// `true` uma vez a cada `PISCA_MS`, enquanto o pisca estiver ligado.
    fn pisca_estourou(&mut self) -> bool {
        if !self.pisca_ligado {
            return false;
        }
        if self.ultimo_pisca.elapsed() < Millis(PISCA_MS) {
            return false;
        }
        self.ultimo_pisca = TickTime::now();
        true
    }

    // ═════════════════════════════════════════════════════════════════════════
    // MODELO DA MÁQUINA E ERROS
    // ═════════════════════════════════════════════════════════════════════════

    /// Lê a chave de modelo e sinaliza erro se a combinação não existir.
    async fn obter_modelo_atual(&mut self) -> u8 {
        let m = modelo::do_chave(self.e.chave_modelo());
        let erro = self.erro_de_modelo(m).await;

        self.interface_erro = erro || estado_salvo::falhou();

        if erro {
            relogio::reset_tudo();
        }
        m
    }

    /// Modelo inválido: desliga tudo e pisca os três LEDs 4 vezes.
    async fn erro_de_modelo(&mut self, m: u8) -> bool {
        if modelo::valido(m) {
            self.modelo_desliga_leds = false;
            return false;
        }

        self.s.desliga_todos_triacs();
        if !self.modelo_desliga_leds {
            self.s.apaga_leds();
            self.modelo_desliga_leds = true;
        }

        for _ in 0..4 {
            self.s.pisca_led_interface();
            // O watchdog é alimentado pela tarefa `watchdog`, que roda
            // enquanto esta espera — não precisa recarregar aqui.
            sleep_for(Millis(700)).await;
        }
        true
    }

    /// Falha ao gravar o estado na flash: desliga tudo e pisca os tres
    /// LEDs juntos, rapido, enquanto a falha durar.
    ///
    /// Nao e um dos codigos contados de [`Erro`], e de proposito: os
    /// codigos travam a maquina ate alguem apertar ON/OFF, e esta falha
    /// **se cura sozinha** se a gravacao der certo na proxima tentativa
    /// (a tarefa `gravacao` fica tentando). Travar seria pior.
    ///
    /// O pisca e de 250 ms, diferente do pisca lento de 700 ms da chave
    /// de modelo invalida — ver `erro_de_modelo`.
    async fn pisca_falha_flash(&mut self) {
        if self.flash_apagou_leds {
            self.s.apaga_leds();
            self.flash_apagou_leds = false;
        }

        for _ in 0..4 {
            self.s.pisca_led_interface();
            sleep_for(Millis(ERRO_ACESO_MS)).await;
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // SELEÇÃO DE PROGRAMA
    // ═════════════════════════════════════════════════════════════════════════

    /// O usuário girou o seletor: recomeça no programa novo.
    async fn selecionar_nova_lavagem(&mut self) {
        self.tipo_lavagem = leituras::posicao_encoder();
        self.limpa_erro();

        self.s.desliga_todos_triacs();
        relogio::reset_tudo();
        agita::liga_timer(false);

        self.enxaguar_etapas = 1;
        self.num_enxagues_executado = 1;
        self.reutilizacao_de_agua = false;
        self.reinicia_tempo_apos_drenar_agua = true;
        self.reinicia_molho();

        self.flash_etapa = 0;
        relogio::liga(false);

        let m = modelo::do_chave(self.e.chave_modelo());

        if m == 6 {
            // Modelo 6: fica em pausa até o usuário apertar ON/OFF.
            self.led_pause = true;
            self.pausa_por_encoder = true;
            self.espera_automatica_encoder_ativa = false;

            // Pisca o LED de ON/OFF 4× como confirmação da escolha.
            for _ in 0..4 {
                self.s.led_pause(true);
                sleep_for(Millis(300)).await;
                self.s.led_pause(false);
                sleep_for(Millis(300)).await;
            }
        } else {
            // Demais modelos: 2 s com tudo desligado e começa sozinho.
            self.led_pause = false;
            self.pausa_por_encoder = false;
            self.inicio_espera_encoder = TickTime::now();
            self.espera_automatica_encoder_ativa = true;
        }
        // `bt_mais_secas` e `bt_enxague_duplo` são preservados — os LEDs
        // das opções não mudam ao girar o seletor.

        self.define_etapa_por_tipo(m);
        self.grava_dados_flash();
    }

    /// Em que etapa este programa começa.
    ///
    /// A maioria começa enchendo, mas cada modelo tem posições do
    /// encoder que são "só enxaguar" ou "só centrifugar".
    fn define_etapa_por_tipo(&mut self, m: u8) {
        match m {
            1 | 2 | 4 | 6 => match self.tipo_lavagem {
                0..=13 => self.etapa = Etapa::Encher,
                14 => self.etapa = Etapa::Enxaguar,
                15 => self.etapa = Etapa::Centrifugar,
                _ => {}
            },
            3 => {
                self.etapa = match self.tipo_lavagem {
                    7 => Etapa::Enxaguar,
                    8 => Etapa::Centrifugar,
                    _ => Etapa::Encher,
                }
            }
            5 => match self.tipo_lavagem {
                0 => self.etapa = Etapa::Centrifugar,
                1..=14 => self.etapa = Etapa::Encher,
                15 => self.etapa = Etapa::Enxaguar,
                _ => {}
            },
            _ => {}
        }
    }

    /// Partida: lê o seletor e o que ficou gravado na flash para
    /// retomar o ciclo de onde parou.
    pub fn seleciona_programa_inicial(&mut self) {
        self.tipo_lavagem = leituras::posicao_encoder();
        self.limpa_erro();

        self.s.desliga_todos_triacs();
        relogio::reset_tudo();

        self.enxaguar_etapas = 1;
        self.num_enxagues_executado = 1;
        self.reinicia_tempo_apos_drenar_agua = false;
        self.reutilizacao_de_agua = false;

        if let Some(salvo) = estado_salvo::carrega() {
            self.flash_etapa = salvo.etapa;
            self.bt_mais_secas = salvo.mais_secas;
            self.bt_enxague_duplo = salvo.enxague_duplo;
            self.led_pause = salvo.pausa;
        } else {
            // Placa nova, flash virgem: começa do zero.
            self.flash_etapa = 0;
            self.bt_mais_secas = false;
            self.bt_enxague_duplo = false;
            self.led_pause = false;
        }

        let m = modelo::do_chave(self.e.chave_modelo());

        // Os programas de uma etapa só ("só enxaguar" e "só
        // centrifugar") não retomam de uma etapa anterior: vão direto
        // para a sua etapa, a não ser que o ciclo já tenha terminado.
        //
        // A diferença entre os modelos é de propósito, copiada do C++:
        // nos modelos 3 e 5 o "só enxaguar" recomeça mesmo que o ciclo
        // tenha acabado; nos outros, não.
        const JA_TERMINOU: u8 = Etapa::Desligar as u8;
        match m {
            1 | 2 | 4 | 6 => {
                if self.tipo_lavagem == 14 && self.flash_etapa != JA_TERMINOU {
                    self.flash_etapa = Etapa::Enxaguar as u8;
                } else if self.tipo_lavagem == 15 && self.flash_etapa != JA_TERMINOU {
                    self.flash_etapa = Etapa::Centrifugar as u8;
                }
            }
            3 => {
                if self.tipo_lavagem == 7 {
                    self.flash_etapa = Etapa::Enxaguar as u8;
                } else if self.tipo_lavagem == 8 && self.flash_etapa != JA_TERMINOU {
                    self.flash_etapa = Etapa::Centrifugar as u8;
                }
            }
            5 => {
                if self.tipo_lavagem == 15 {
                    self.flash_etapa = Etapa::Enxaguar as u8;
                } else if self.tipo_lavagem == 0 && self.flash_etapa != JA_TERMINOU {
                    self.flash_etapa = Etapa::Centrifugar as u8;
                }
            }
            _ => {}
        }

        self.etapa = match self.flash_etapa {
            0 => Etapa::Encher,
            1 => Etapa::Agitar,
            2 => Etapa::Molho,
            3 => Etapa::Enxaguar,
            4 => Etapa::Centrifugar,
            5 => Etapa::Desligar,
            _ => {
                // Valor inesperado: recomeça o ciclo.
                self.flash_etapa = 0;
                Etapa::Encher
            }
        };
    }

    // ═════════════════════════════════════════════════════════════════════════
    // BOTÕES E LEDS DO PAINEL
    // ═════════════════════════════════════════════════════════════════════════

    /// Lê o botão do ADC e atualiza os LEDs. É o `gerenciaInterface`
    /// do C++.
    ///
    /// A ação só acontece na transição "repouso → botão apertado":
    /// `acao_processada` é armado quando nenhum botão está pressionado
    /// e consumido no aperto, então um botão segurado não repete.
    fn gerencia_interface(&mut self) {
        let etapa_atual = self.etapa;

        if botoes::em_repouso(self.ad_button) {
            if !self.acao_processada {
                self.grava_dados_flash();
                self.acao_processada = true;
            }
        } else if self.acao_processada {
            match botoes::identifica(self.ad_button) {
                Some(Botao::Pause) => {
                    // Travado num codigo de erro: o ON/OFF e a saida.
                    // Desliga a maquina, e o ciclo recomeca do zero no
                    // proximo toque — nao se retoma de cima de uma
                    // protecao que disparou.
                    if etapa_atual == Etapa::Erro {
                        self.limpa_erro();
                        self.definir_etapa_desligado();
                        self.acao_processada = false;
                        return;
                    }

                    self.garantir_ligado(etapa_atual);

                    if self.led_pause {
                        self.led_pause = false;

                        // Na pausa por encoder o relógio ficou parado sem
                        // a tampa ter aberto, então nada o religaria.
                        if self.pausa_por_encoder {
                            relogio::liga(true);
                            self.pausa_por_encoder = false;
                        }
                    } else if etapa_atual != Etapa::Desligar {
                        self.definir_etapa_desligado();
                    }
                    self.acao_processada = false;
                }
                Some(Botao::MaisSecas) => {
                    self.garantir_ligado(etapa_atual);
                    if etapa_atual != Etapa::Desligar {
                        self.bt_mais_secas = !self.bt_mais_secas;
                    }
                    self.acao_processada = false;
                }
                Some(Botao::EnxagueDuplo) => {
                    self.garantir_ligado(etapa_atual);
                    if etapa_atual != Etapa::Desligar {
                        self.bt_enxague_duplo = !self.bt_enxague_duplo;
                    }
                    self.acao_processada = false;
                }
                None => {}
            }
        }

        if !self.interface_erro && !matches!(etapa_atual, Etapa::Desligar | Etapa::Erro) {
            self.s.led_mais_secas(self.bt_mais_secas);
            self.s.led_enxague_duplo(self.bt_enxague_duplo);

            if !self.led_pause && self.tampa_fechada {
                self.s.led_pause(true);
            }
        }
    }

    /// Apertar qualquer botão com a máquina desligada a liga.
    fn garantir_ligado(&mut self, etapa_atual: Etapa) {
        if etapa_atual == Etapa::Desligar {
            self.reinicia_molho();
            self.flash_etapa = 0;
            let m = modelo::do_chave(self.e.chave_modelo());
            self.define_etapa_por_tipo(m);
        }
    }

    /// Uma nova lavagem não deve herdar a fase nem a contagem do molho.
    fn reinicia_molho(&mut self) {
        self.molho_ciclo_atual = 1;
        self.molho_modo = ModoMolho::Molho;
        self.molho_modo_prev = ModoMolho::Molho;
        agita::liga_timer(false);
        agita::zera();
    }

    /// Esquece o erro e zera a contagem de quedas de nivel.
    ///
    /// Chamada ao sair do erro pelo ON/OFF, ao girar o seletor e ao
    /// desligar: em todos os casos uma lavagem nova comeca, e o
    /// historico da anterior nao vale mais.
    fn limpa_erro(&mut self) {
        self.erro_atual = None;
        self.erro_aceso = false;
        self.recuperacoes_de_nivel = 0;
    }

    fn definir_etapa_desligado(&mut self) {
        self.s.desliga_todos_triacs();
        self.flash_etapa = Etapa::Desligar as u8;
        self.etapa = Etapa::Desligar;
    }

    // ═════════════════════════════════════════════════════════════════════════
    // FLASH
    // ═════════════════════════════════════════════════════════════════════════

    /// Pede à tarefa de gravação para salvar o estado atual.
    ///
    /// Diferente do C++, não espera a flash: a gravação acontece na
    /// tarefa `tarefas::gravacao`, entre duas passadas da máquina.
    fn grava_dados_flash(&self) {
        estado_salvo::pede_gravacao(EstadoSalvo {
            etapa: self.flash_etapa,
            mais_secas: self.bt_mais_secas,
            enxague_duplo: self.bt_enxague_duplo,
            pausa: self.led_pause,
        });
    }

    /// Troca de etapa: desliga tudo, zera o relógio e grava.
    fn transicionar_para(&mut self, prox: Etapa) {
        self.s.desliga_todos_triacs();
        relogio::reset_tudo();
        self.flash_etapa = prox as u8;
        self.grava_dados_flash();
        self.etapa = prox;
    }
}
