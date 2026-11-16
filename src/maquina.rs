//! =============================================================
//! MÁQUINA DE ESTADOS — o cérebro do firmware
//!
//! Função única e síncrona: `trata(evento)`.
//! Não existe `async` aqui dentro, não existe espera, não existe
//! `select!`. Entra um evento, sai uma decisão. É possível ler o
//! comportamento inteiro da máquina de cima a baixo neste arquivo.
//!
//!            ┌──────────┐  on/off   ┌───────────────┐
//!            │ Desligada│──────────▶│ Selecionando  │
//!            └──────────┘◀──────────└───────────────┘
//!                  ▲                      │ 2 s sem toque
//!                  │ on/off               ▼
//!            ┌──────────┐  tampa    ┌───────────────┐
//!            │   Erro   │◀──────────│  Executando   │
//!            └──────────┘           └───────────────┘
//!                                     │          ▲
//!                             tampa   ▼          │ tampa fechada + 2 s
//!                                   ┌───────────────┐
//!                                   │   Pausada     │
//!                                   └───────────────┘
//! =============================================================

use crate::agitacao::{Agitacao, Motores};
use crate::estado_salvo::{self, EstadoSalvo};
use crate::evento::{Botao, Evento};
use crate::opcoes::Opcoes;
use crate::placa::{Atuadores, Entradas, Painel};
use crate::programa::{
    Acao, FimDeCiclo, Etapa, Programa, FASES_AMACIANTE, LIMITE_NIVEL_CAIU, PULSOS, TIMEOUT_DRENAR_S,
    TIMEOUT_ENCHER_S,
};

/// Tiques de 50 ms em 1 segundo.
const TIQUES_POR_SEGUNDO: u16 = 20;
/// Tempo sem tocar em nada que confirma o programa escolhido: 2 s.
const TIQUES_CONFIRMACAO: u16 = 2 * TIQUES_POR_SEGUNDO;
/// Espera de segurança depois que a tampa fecha, antes de religar: 2 s.
const TIQUES_RELIGAR: u16 = 2 * TIQUES_POR_SEGUNDO;
/// Meio período do pisca de aviso: 300 ms.
const TIQUES_PISCA: u16 = 6;
/// Os sensores só são cobrados a partir de 3 s dentro da etapa.
const CARENCIA_SENSOR_S: u16 = 3;

// ─── Animação de inicialização ───────────────────────────────────────────────
/// Dois acendimentos de 1 s, separados por 500 ms apagados.
const TIQUES_ACESO: u16 = TIQUES_POR_SEGUNDO;
const TIQUE_SEGUNDO_ACENDIMENTO: u16 = TIQUES_ACESO + TIQUES_POR_SEGUNDO / 2;
const TIQUES_ANIMACAO: u16 = TIQUE_SEGUNDO_ACENDIMENTO + TIQUES_ACESO;

// ─── Auto-teste ──────────────────────────────────────────────────────────────
/// O gesto de dois botões só abre o auto-teste nos primeiros 15 s depois
/// de energizar — depois disso a combinação é ignorada.
const TIQUES_JANELA_AUTO_TESTE: u16 = 15 * TIQUES_POR_SEGUNDO;
/// Duração do passo da bomba no auto-teste: 60 s (ou o botão Nível atalha).
const TIQUES_BOMBA_AUTO_TESTE: u16 = 60 * TIQUES_POR_SEGUNDO;

// ─── Código de erro piscado no painel ────────────────────────────────────────
// O painel inteiro pisca N vezes, faz uma pausa e repete. N identifica o
// erro — veja `Erro::piscadas()`.
/// Parte acesa de cada piscada: 250 ms.
pub const TIQUES_ERRO_ACESO: u16 = 5;
/// Parte apagada de cada piscada: 250 ms.
pub const TIQUES_ERRO_APAGADO: u16 = 5;
/// Período de uma piscada completa: 500 ms.
pub const TIQUES_ERRO_PISCADA: u16 = TIQUES_ERRO_ACESO + TIQUES_ERRO_APAGADO;
/// Pausa escura entre duas repetições do código: 1,5 s.
///
/// Tem que ser bem maior que uma piscada, senão não se sabe onde a
/// contagem começa.
pub const TIQUES_ERRO_PAUSA: u16 = 30;

/// Condições que param a máquina e fazem o painel piscar o código do erro.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Erro {
    /// Não atingiu o nível dentro do tempo.
    Encher,
    /// Não esvaziou dentro do tempo.
    Drenar,
    /// Nível caiu sozinho (vazamento ou sensor solto).
    Nivel,
}

impl Erro {
    /// Quantas piscadas o painel dá para anunciar este erro.
    ///
    /// É o código de diagnóstico que o técnico conta em campo, sem
    /// gravador e sem display:
    ///
    /// | Piscadas | Erro | O que suspeitar |
    /// |---|---|---|
    /// | 2 | [`Erro::Nivel`] | nível caiu sozinho — vazamento ou sensor solto |
    /// | 3 | [`Erro::Encher`] | não encheu — válvula, torneira ou sensor de nível |
    /// | 4 | [`Erro::Drenar`] | não esvaziou — eletrobomba, mangueira ou filtro |
    ///
    /// Os números não se repetem e começam em 2 de propósito: uma
    /// piscada só se confundiria com o pisca de tampa aberta.
    pub const fn piscadas(self) -> u16 {
        match self {
            Erro::Nivel => 2,
            Erro::Encher => 3,
            Erro::Drenar => 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Estado {
    /// Teste dos LEDs com duas acendidas. Dura 2,5 s e termina em `Desligada`.
    Inicializando { passo: u16 },
    /// Painel apagado. Só o botão on/off responde.
    Desligada,
    /// Mostrando o programa escolhido; confirma sozinho após 2 s.
    Selecionando { restante: u16 },
    /// Executando a tabela de etapas do programa.
    Executando,
    /// Tampa aberta no meio do ciclo — tudo desligado, LED piscando.
    Pausada { religar_em: u16 },
    /// Erro travado até o usuário apertar on/off.
    Erro(Erro),
    /// Modo de fábrica/manutenção — ver a seção AUTO-TESTE mais abaixo.
    AutoTeste { passo: PassoAutoTeste, tiques: u16 },
}

/// Passos do auto-teste, na ordem em que o técnico percorre.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PassoAutoTeste {
    /// Pisca os dois LEDs de nível até o técnico soltar os dois botões.
    AguardandoSoltura,
    /// LED lavagem pesada + válvula de sabão aberta.
    ValvulaSabao,
    /// LED lavagem normal + válvula de amaciante aberta.
    ValvulaAmaciante,
    /// LED lavagem rápida + agitação no padrão normal.
    MotorAgitacao,
    /// LED enxaguar + eletrobomba (60 s, ou o botão Nível atalha).
    Bomba,
    /// LED centrifugar + motor C, com a bomba ainda ligada.
    MotorCentrifuga,
}

/// Estado observável da máquina — devolvido por [`Maquina::fotografia`].
///
/// O firmware em si não usa: quem lê é o simulador de bancada
/// (`ferramentas/simular_ciclo.rs`), que roda esta máquina de estados no PC.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Fotografia {
    pub programa: Programa,
    pub etapa: usize,
    pub segundos_etapa: u16,
    pub opcoes: Opcoes,
    pub executando: bool,
    pub desligada: bool,
    /// `Some(erro)` quando a máquina está travada piscando um código.
    pub erro: Option<Erro>,
}

pub struct Maquina<'a> {
    // A máquina só escreve no hardware (painel e triacs). Ela NÃO lê
    // pino nenhum: tudo o que sabe das entradas chega por evento e fica
    // nos espelhos abaixo. `Entradas` aparece só no construtor, para os
    // espelhos nascerem com o estado real.
    painel: &'a Painel,
    at: &'a Atuadores,

    estado: Estado,
    programa: Programa,
    opcoes: Opcoes,
    retomar_apos_boot: bool,

    // ─── Execução do ciclo ───────────────────────────────────────────────────
    etapa: usize,
    segundos_etapa: u16,
    molho_amaciante: bool,
    espera_cambio: u16,
    agitacao: Agitacao,

    // Pulsos progressivos
    pulso_indice: usize,
    pulso_ligado: bool,
    pulso_segundos: u16,

    // Vigilância do nível
    nivel_caiu: u8,
    encheu_com_nivel_alto: bool,

    // ─── Espelho das entradas (atualizado por eventos) ───────────────────────
    nivel_medio: bool,
    nivel_alto: bool,
    tampa_aberta: bool,
    /// Um bit por botão (ver `Botao::bit()`) — quais estão pressionados agora.
    botoes: u8,

    /// Tiques desde a energização, usado só para a janela do auto-teste.
    /// Para de contar assim que a janela fecha.
    tiques_desde_boot: u16,

    // ─── Pisca de aviso ──────────────────────────────────────────────────────
    pisca_aceso: bool,
    pisca_contador: u16,
}

impl<'a> Maquina<'a> {
    pub fn nova(painel: &'a Painel, at: &'a Atuadores, entradas: &'a Entradas) -> Self {
        // O espelho dos botões precisa nascer com o estado REAL dos pinos:
        // o técnico pode já estar segurando Seletor + Nível no instante em
        // que a máquina é energizada, e nesse caso nenhum evento de
        // "botão pressionado" chega — a mudança nunca aconteceu.
        let mut botoes = 0u8;
        for b in Botao::TODOS {
            if entradas.botao_pressionado(b) {
                botoes |= b.bit();
            }
        }

        painel.acende_tudo();

        Self {
            painel,
            at,
            estado: Estado::Inicializando { passo: 0 },
            programa: Programa::Pesada,
            opcoes: Opcoes::padrao(),
            retomar_apos_boot: false,
            etapa: 0,
            segundos_etapa: 0,
            molho_amaciante: false,
            espera_cambio: 0,
            agitacao: Agitacao::nova(),
            pulso_indice: 0,
            pulso_ligado: false,
            pulso_segundos: 0,
            nivel_caiu: 0,
            encheu_com_nivel_alto: false,
            nivel_medio: entradas.nivel_medio_atingido(),
            nivel_alto: entradas.nivel_alto_atingido(),
            tampa_aberta: entradas.tampa_aberta(),
            botoes,
            tiques_desde_boot: 0,
            pisca_aceso: false,
            pisca_contador: 0,
        }
    }

    /// Retoma um ciclo salvo na flash (programa + etapa + opções).
    ///
    /// Se estava ligada, retoma após concluir o teste dos LEDs.
    pub fn restaura(&mut self, salvo: EstadoSalvo) {
        self.programa = salvo.programa;
        self.etapa = salvo.etapa as usize;
        self.opcoes = salvo.opcoes;
        self.retomar_apos_boot = salvo.ligado;
    }

    /// Enfileira a gravação do estado atual na flash.
    ///
    /// Barato: só deixa um recado para a `tarefas::gravacao`. A gravação
    /// de verdade acontece fora da máquina de estados.
    fn salva(&self) {
        estado_salvo::pede_gravacao(EstadoSalvo {
            programa: self.programa,
            etapa: self.etapa as u8,
            opcoes: self.opcoes,
            ligado: matches!(
                self.estado,
                Estado::Selecionando { .. } | Estado::Executando | Estado::Pausada { .. }
            ),
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // PONTO DE ENTRADA ÚNICO
    // ═════════════════════════════════════════════════════════════════════════

    /// Fotografia do estado interno, só para leitura.
    ///
    /// Existe para diagnóstico e para o simulador de bancada
    /// (`ferramentas/simular_ciclo.rs`), que roda esta máquina de estados
    /// no PC. Não altera nada.
    #[allow(dead_code)]
    pub fn fotografia(&self) -> Fotografia {
        Fotografia {
            programa: self.programa,
            etapa: self.etapa,
            segundos_etapa: self.segundos_etapa,
            opcoes: self.opcoes,
            executando: matches!(self.estado, Estado::Executando),
            desligada: matches!(self.estado, Estado::Desligada),
            erro: match self.estado {
                Estado::Erro(e) => Some(e),
                _ => None,
            },
        }
    }

    pub fn trata(&mut self, evento: Evento) {
        match evento {
            Evento::BotaoPressionado(b) => {
                self.botoes |= b.bit();
                if let Estado::AutoTeste { passo, .. } = self.estado {
                    self.auto_teste_botao(passo, b);
                } else {
                    self.botao(b);
                }
            }
            Evento::BotaoSolto(b) => self.botoes &= !b.bit(),
            Evento::ComboAutoTeste => self.talvez_entra_auto_teste(),
            Evento::Tampa { aberta } => self.tampa_mudou(aberta),
            Evento::NivelMedio { atingido } => self.nivel_medio = atingido,
            Evento::NivelAlto { atingido } => self.nivel_alto = atingido,
            Evento::Tique => self.tique(),
            Evento::Segundo => self.segundo(),
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // BOTÕES
    // ═════════════════════════════════════════════════════════════════════════

    fn botao(&mut self, b: Botao) {
        match b {
            Botao::OnOff => self.botao_on_off(),
            Botao::Seletor => self.botao_seletor(),
            _ => self.botao_de_opcao(b),
        }
    }

    fn botao_on_off(&mut self) {
        match self.estado {
            // Aguarda o teste completo dos LEDs antes de ligar.
            Estado::Inicializando { .. } => {}
            Estado::Desligada => {
                // Liga: mostra o programa e espera 2 s de confirmação.
                self.estado = Estado::Selecionando {
                    restante: TIQUES_CONFIRMACAO,
                };
                self.painel.mostra_programa(self.programa);
                self.painel.mostra_opcoes(&self.opcoes, true);
                self.salva();
            }
            _ => self.desliga(),
        }
    }

    fn botao_seletor(&mut self) {
        match self.estado {
            // Desligada: o seletor não faz nada (igual ao firmware original).
            Estado::Desligada | Estado::Inicializando { .. } | Estado::Erro(_) => {}
            // Em qualquer outro estado: cancela o que estiver rodando
            // e avança para o próximo programa.
            _ => {
                self.at.desliga_todos();
                self.agitacao.reinicia();
                self.programa = self.programa.proximo();
                self.etapa = 0;
                self.segundos_etapa = 0;
                self.estado = Estado::Selecionando {
                    restante: TIQUES_CONFIRMACAO,
                };
                self.painel.mostra_programa(self.programa);
                self.salva();
            }
        }
    }

    fn botao_de_opcao(&mut self, b: Botao) {
        // Desligada, animando ou em erro, os botões de opção não fazem nada.
        if matches!(
            self.estado,
            Estado::Desligada | Estado::Inicializando { .. } | Estado::Erro(_)
        ) {
            return;
        }
        match b {
            Botao::Nivel => self.opcoes.nivel_alto = !self.opcoes.nivel_alto,
            Botao::EnxagueExtra => self.opcoes.enxague_extra = !self.opcoes.enxague_extra,
            Botao::Turbo => self.opcoes.turbo = !self.opcoes.turbo,
            Botao::Amaciante => self.opcoes.amaciante = !self.opcoes.amaciante,
            _ => {}
        }
        self.painel.mostra_opcoes(&self.opcoes, true);
        if b == Botao::Nivel {
            if matches!(
                self.etapa_atual().map(|e| e.acao),
                Some(Acao::EncherSabao | Acao::EncherEnxague | Acao::EncherAmaciante)
            ) {
                self.segundos_etapa = 0;
            } else if self.estado == Estado::Executando
                && matches!(
                    self.programa,
                    Programa::Pesada | Programa::Normal | Programa::Rapido
                )
                && self.opcoes.nivel_alto
                && !self.encheu_com_nivel_alto
                && matches!(
                    self.etapa_atual().map(|e| e.acao),
                    Some(Acao::Agitar | Acao::Molho)
                )
            {
                self.etapa = 0;
                self.segundos_etapa = 0;
                self.aplica_acao_da_etapa();
            }
        }
        if b == Botao::EnxagueExtra
            && self.estado == Estado::Executando
            && matches!(
                self.etapa_atual().map(|e| e.acao),
                Some(Acao::EncherEnxague)
            )
        {
            self.at.valvula_amaciante(!self.opcoes.enxague_extra);
        }
        self.salva();

        // Mexer numa opção também renova a janela de confirmação.
        if let Estado::Selecionando { .. } = self.estado {
            self.estado = Estado::Selecionando {
                restante: TIQUES_CONFIRMACAO,
            };
        }
    }

    fn desliga(&mut self) {
        self.at.desliga_todos();
        self.agitacao.reinicia();
        self.painel.apaga_tudo();
        self.estado = Estado::Desligada;
        self.salva(); // guarda a etapa para retomar depois
    }

    // ═════════════════════════════════════════════════════════════════════════
    // TAMPA
    // ═════════════════════════════════════════════════════════════════════════

    fn tampa_mudou(&mut self, aberta: bool) {
        self.tampa_aberta = aberta;

        // O auto-teste ignora a tampa de propósito: o técnico precisa ver
        // as válvulas e o motor com a máquina aberta. É o mesmo que o
        // firmware original fazia, e o modo só abre com o gesto de dois
        // botões nos primeiros 15 s depois de energizar.
        if let Estado::AutoTeste { .. } = self.estado {
            return;
        }

        if aberta {
            // Segurança: qualquer que seja o estado, tudo desliga na hora.
            self.at.desliga_todos();
            self.agitacao.reinicia();
            if self.estado == Estado::Executando {
                self.estado = Estado::Pausada {
                    religar_em: TIQUES_RELIGAR,
                };
            }
        } else if let Estado::Pausada { .. } = self.estado {
            // Fechou: começa a contagem de 2 s antes de religar os triacs.
            self.estado = Estado::Pausada {
                religar_em: TIQUES_RELIGAR,
            };
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // BASE DE TEMPO RÁPIDA — 50 ms
    // ═════════════════════════════════════════════════════════════════════════

    fn tique(&mut self) {
        if self.tiques_desde_boot < TIQUES_JANELA_AUTO_TESTE {
            self.tiques_desde_boot += 1;
        }

        match self.estado {
            Estado::Inicializando { passo } => self.tique_animacao(passo),

            Estado::Desligada => {}

            Estado::Selecionando { restante } => {
                if restante <= 1 {
                    self.inicia_execucao();
                } else {
                    self.estado = Estado::Selecionando {
                        restante: restante - 1,
                    };
                }
            }

            Estado::Executando => self.tique_executando(),

            Estado::Pausada { religar_em } => {
                self.pisca_led_do_programa();
                if self.tampa_aberta {
                    return; // ainda aberta: não conta nada
                }
                if religar_em <= 1 {
                    self.painel.mostra_programa(self.programa);
                    self.estado = Estado::Executando;
                    if matches!(
                        self.etapa_atual().map(|e| e.acao),
                        Some(Acao::Pulsos | Acao::Centrifugar)
                    ) {
                        self.espera_cambio = 10 * TIQUES_POR_SEGUNDO;
                        self.at.eletrobomba(true);
                    } else {
                        self.aplica_acao_da_etapa();
                    }
                } else {
                    self.estado = Estado::Pausada {
                        religar_em: religar_em - 1,
                    };
                }
            }

            Estado::Erro(erro) => self.pisca_codigo_de_erro(erro),

            Estado::AutoTeste { passo, tiques } => self.auto_teste_tique(passo, tiques),
        }
    }

    /// Testa todos os LEDs sem bloquear o consumo dos eventos.
    fn tique_animacao(&mut self, passo: u16) {
        let passo = passo + 1;
        match passo {
            TIQUES_ACESO => self.painel.apaga_tudo(),
            TIQUE_SEGUNDO_ACENDIMENTO => self.painel.acende_tudo(),
            TIQUES_ANIMACAO => {
                self.painel.apaga_tudo();
                self.estado = Estado::Desligada;
                if self.retomar_apos_boot {
                    self.retomar_apos_boot = false;
                    self.botao_on_off();
                    return;
                }
                // Reconhece também o combo segurado desde a energização.
                let combo = Botao::Seletor.bit() | Botao::Nivel.bit();
                if self.botoes & combo == combo {
                    self.talvez_entra_auto_teste();
                }
                return;
            }
            _ => {}
        }
        self.estado = Estado::Inicializando { passo };
    }

    fn tique_executando(&mut self) {
        if self.espera_cambio > 0 {
            self.espera_cambio -= 1;
            if self.espera_cambio == 0 {
                let centrifuga =
                    matches!(self.etapa_atual().map(|e| e.acao), Some(Acao::Centrifugar));
                self.at.motor_c(centrifuga || self.pulso_ligado);
            }
            return;
        }
        let Some(etapa) = self.etapa_atual() else {
            return;
        };
        // Só a agitação precisa de resolução de 50 ms; o resto anda de 1 em 1 s.
        let agita_molho = etapa.acao == Acao::MolhoEnxague
            && self.molho_amaciante
            && self.fase_amaciante().is_some_and(|(_, agita)| agita);
        if etapa.acao == Acao::Agitar || agita_molho {
            let m = self.agitacao.tique(self.opcoes.turbo);
            self.aplica_motores(m);
        } else if etapa.acao == Acao::MolhoEnxague {
            self.at.motor_a(false);
            self.at.motor_c(false);
            self.agitacao.reinicia();
        }
    }

    fn fase_amaciante(&self) -> Option<(u16, bool)> {
        let mut restante = self.segundos_etapa;
        for &(duracao, agita) in FASES_AMACIANTE {
            if restante < duracao {
                return Some((duracao - restante, agita));
            }
            restante -= duracao;
        }
        None
    }

    fn aplica_motores(&self, m: Motores) {
        // Primeiro retira o comando anterior antes de energizar outro sentido.
        self.at.motor_a(false);
        self.at.motor_c(false);
        self.at.motor_a(m.agitacao);
        self.at.motor_c(m.centrifugacao);
    }

    // ═════════════════════════════════════════════════════════════════════════
    // BASE DE TEMPO LENTA — 1 s
    // ═════════════════════════════════════════════════════════════════════════

    fn segundo(&mut self) {
        if self.estado != Estado::Executando || self.espera_cambio > 0 {
            return;
        }
        let Some(etapa) = self.etapa_atual() else {
            return;
        };

        self.segundos_etapa = self.segundos_etapa.saturating_add(1);

        match etapa.acao {
            Acao::EncherSabao | Acao::EncherEnxague | Acao::EncherAmaciante => {
                self.segundo_enchendo()
            }
            Acao::Agitar | Acao::Molho => self.segundo_com_agua(etapa),
            Acao::MolhoEnxague => {
                if !self.vigia_nivel() {
                    return;
                }
                if self.molho_amaciante {
                    if self.fase_amaciante().is_none() {
                        self.avanca_etapa();
                    }
                } else {
                    self.termina_etapa_por_tempo(etapa);
                }
            }
            Acao::Drenar => self.segundo_drenando(),
            Acao::Pulsos => self.segundo_pulsando(),
            Acao::DrenarExtra | Acao::Centrifugar | Acao::BombaPos | Acao::PausaFinal => {
                self.termina_etapa_por_tempo(etapa)
            }
        }
    }

    fn segundo_enchendo(&mut self) {
        if self.nivel_selecionado_atingido() {
            self.encheu_com_nivel_alto = self.opcoes.nivel_alto;
            self.avanca_etapa();
        } else if self.segundos_etapa >= TIMEOUT_ENCHER_S {
            self.entra_em_erro(Erro::Encher);
        }
    }

    fn segundo_com_agua(&mut self, etapa: Etapa) {
        if self.vigia_nivel() {
            self.termina_etapa_por_tempo(etapa);
        }
    }

    /// A mesma proteção vale para todas as etapas que mantêm água.
    fn vigia_nivel(&mut self) -> bool {
        // Vigia se o nível caiu sozinho — vazamento ou sensor solto.
        if self.segundos_etapa > CARENCIA_SENSOR_S {
            let caiu = if self.encheu_com_nivel_alto {
                !self.nivel_alto
            } else {
                !self.nivel_medio
            };
            if caiu {
                self.nivel_caiu += 1;
                if self.nivel_caiu >= LIMITE_NIVEL_CAIU {
                    self.entra_em_erro(Erro::Nivel);
                    return false;
                }
            } else {
                self.nivel_caiu = 0;
            }
        }
        true
    }

    fn segundo_drenando(&mut self) {
        if !self.nivel_medio && !self.nivel_alto {
            self.avanca_etapa();
        } else if self.segundos_etapa >= TIMEOUT_DRENAR_S {
            self.entra_em_erro(Erro::Drenar);
        }
    }

    fn segundo_pulsando(&mut self) {
        let Some(&(ligado_s, desligado_s)) = PULSOS.get(self.pulso_indice) else {
            self.avanca_etapa();
            return;
        };

        self.pulso_segundos += 1;
        let limite = if self.pulso_ligado {
            ligado_s
        } else {
            desligado_s
        };

        if self.pulso_segundos < limite {
            return;
        }

        self.pulso_segundos = 0;
        if self.pulso_ligado {
            // Terminou a parte ligada: desliga e espera a pausa.
            self.pulso_ligado = false;
            self.at.motor_c(false);
        } else {
            // Terminou a pausa: vai para o próximo pulso.
            self.pulso_indice += 1;
            if self.pulso_indice >= PULSOS.len() {
                self.avanca_etapa();
                return;
            }
            self.pulso_ligado = true;
            self.at.motor_c(true);
        }
    }

    fn termina_etapa_por_tempo(&mut self, etapa: Etapa) {
        if etapa.segundos > 0 && self.segundos_etapa >= etapa.segundos {
            self.avanca_etapa();
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // CICLO: entrar, avançar e aplicar etapas
    // ═════════════════════════════════════════════════════════════════════════

    fn inicia_execucao(&mut self) {
        if self.tampa_aberta {
            self.estado = Estado::Pausada {
                religar_em: TIQUES_RELIGAR,
            };
            return;
        }
        self.estado = Estado::Executando;
        self.painel.mostra_programa(self.programa);
        self.segundos_etapa = 0;
        self.encheu_com_nivel_alto = self.nivel_alto;
        self.espera_cambio = if matches!(
            self.etapa_atual().map(|e| e.acao),
            Some(Acao::Pulsos | Acao::Centrifugar)
        ) {
            10 * TIQUES_POR_SEGUNDO
        } else {
            0
        };
        self.aplica_acao_da_etapa();
        self.salva();
    }

    fn avanca_etapa(&mut self) {
        self.etapa += 1;
        self.segundos_etapa = 0;
        self.nivel_caiu = 0;
        self.espera_cambio = 0;

        // `aplica_acao_da_etapa` já pula as etapas inaplicáveis e encerra
        // o ciclo se a tabela acabou.
        self.aplica_acao_da_etapa();
        if self.estado == Estado::Executando {
            self.salva(); // etapa nova gravada: a queda de luz não perde o ciclo
        }
    }

    /// A tabela do programa acabou.
    ///
    /// Encadeia no programa seguinte ou, se este era o último da cadeia
    /// (a centrifugação), encerra o ciclo completo e desliga.
    ///
    /// Recursão: o caminho `Encadeia` chama `aplica_acao_da_etapa`, que
    /// pode voltar aqui se a tabela nova estiver vazia. Cada volta avança
    /// `programa` uma posição e `Centrifugar` sempre sai por `Desliga`,
    /// então a profundidade é no máximo o número de programas.
    fn ciclo_concluido(&mut self) {
        self.at.desliga_todos();
        self.agitacao.reinicia();
        self.etapa = 0;
        self.segundos_etapa = 0;

        match self.programa.fim_de_ciclo() {
            FimDeCiclo::Encadeia => {
                self.programa = self.programa.proximo();
                self.painel.mostra_programa(self.programa);
                self.painel.mostra_opcoes(&self.opcoes, true);
                self.estado = Estado::Executando;
                self.salva();
                // Começa a etapa 0 do programa novo sem esperar confirmação,
                // como o `main` do original fazia ao avançar o ciclo.
                self.aplica_acao_da_etapa();
            }
            FimDeCiclo::Desliga => {
                self.programa = Programa::Pesada;
                self.opcoes = Opcoes::padrao();
                self.painel.apaga_tudo();
                self.estado = Estado::Desligada;
                self.salva();
            }
        }
    }

    /// Etapa corrente. Consulta pura: NÃO mexe no índice.
    ///
    /// Isso importa: `segundo()` e `tique()` chamam esta função o tempo
    /// todo. Se ela pudesse avançar o índice sozinha, mudar uma opção no
    /// meio do ciclo pularia para a etapa seguinte sem ninguém ligar os
    /// atuadores dela. Quem avança é `avanca_etapa()`, e só ele.
    fn etapa_atual(&self) -> Option<Etapa> {
        self.programa.etapas().get(self.etapa).copied()
    }

    /// Pula as etapas que não se aplicam às opções de agora — hoje, as
    /// marcadas com `ex(...)` quando "enxágue extra" está desligado.
    fn pula_inaplicaveis(&mut self) {
        while let Some(etapa) = self.programa.etapas().get(self.etapa) {
            if etapa.so_enxague_extra && !self.opcoes.enxague_extra {
                self.etapa += 1;
            } else {
                break;
            }
        }
    }

    /// Liga as saídas que a etapa atual exige. Chamado uma vez por etapa
    /// (e de novo ao sair da pausa de tampa).
    fn aplica_acao_da_etapa(&mut self) {
        // O original só faz a drenagem inicial do enxágue se houver água.
        if self.programa == Programa::Enxaguar
            && self.etapa == 0
            && !self.nivel_medio
            && !self.nivel_alto
        {
            self.etapa = 5;
        }
        self.pula_inaplicaveis();
        let Some(etapa) = self.etapa_atual() else {
            self.ciclo_concluido();
            return;
        };

        self.at.desliga_todos();
        self.agitacao.reinicia();

        match etapa.acao {
            // Enchimento da lavagem: só a válvula de sabão.
            Acao::EncherSabao => self.at.valvula_sabao(true),

            // 1º enxágue: o amaciante só entra aqui quando este é o ÚNICO
            // enxágue do ciclo. Havendo enxágue extra pela frente, ele fica
            // guardado para o último — igual ao firmware original.
            Acao::EncherEnxague => {
                self.at.valvula_sabao(true);
                self.at.valvula_amaciante(!self.opcoes.enxague_extra);
            }

            // Último enxágue: é aqui que o amaciante entra.
            Acao::EncherAmaciante => {
                self.at.valvula_sabao(true);
                self.at.valvula_amaciante(true);
            }
            Acao::Agitar => {} // os motores são comandados tique a tique
            Acao::Molho => {}  // tudo parado
            Acao::PausaFinal => {}
            Acao::MolhoEnxague => self.molho_amaciante = self.opcoes.amaciante,
            Acao::Drenar | Acao::DrenarExtra | Acao::BombaPos => self.at.eletrobomba(true),
            Acao::Pulsos => {
                self.pulso_indice = 0;
                self.pulso_ligado = true;
                self.pulso_segundos = 0;
                self.at.eletrobomba(true);
                self.at.motor_c(self.espera_cambio == 0);
            }
            Acao::Centrifugar => {
                self.at.eletrobomba(true);
                self.at.motor_c(self.espera_cambio == 0);
            }
        }
    }

    // ═════════════════════════════════════════════════════════════════════════
    // AUTO-TESTE — modo de fábrica / manutenção
    // ═════════════════════════════════════════════════════════════════════════
    //
    // Como entrar: com a máquina recém-energizada, segurar os botões
    // SELETOR e NÍVEL ao mesmo tempo dentro dos primeiros 15 s.
    //
    // Como andar: cada passo liga UM atuador e acende UM LED. O botão
    // SELETOR avança; no passo da bomba quem atalha é o botão NÍVEL.
    //
    //   LEDs de nível piscando  aguardando soltar os dois botões
    //   LED lavagem pesada      válvula de sabão
    //   LED lavagem normal      válvula de amaciante
    //   LED lavagem rápida      motor em agitação
    //   LED enxaguar            eletrobomba (60 s ou botão Nível)
    //   LED centrifugar         motor C + bomba  → SELETOR encerra
    //
    // ATENÇÃO: este modo ignora o sensor de tampa de propósito — veja
    // o comentário em `tampa_mudou()`.

    /// Trata o gesto de abertura. Só vale nos primeiros 15 s e com a
    /// máquina parada; fora disso a combinação é simplesmente ignorada.
    fn talvez_entra_auto_teste(&mut self) {
        if self.tiques_desde_boot >= TIQUES_JANELA_AUTO_TESTE {
            return;
        }
        if self.estado != Estado::Desligada {
            return;
        }
        self.vai_para_passo(PassoAutoTeste::AguardandoSoltura);
    }

    /// Botão pressionado durante o auto-teste.
    fn auto_teste_botao(&mut self, passo: PassoAutoTeste, botao: Botao) {
        use PassoAutoTeste::*;

        match (passo, botao) {
            // O avanço normal é pelo seletor.
            (ValvulaSabao, Botao::Seletor) => self.vai_para_passo(ValvulaAmaciante),
            (ValvulaAmaciante, Botao::Seletor) => self.vai_para_passo(MotorAgitacao),
            (MotorAgitacao, Botao::Seletor) => self.vai_para_passo(Bomba),
            (MotorCentrifuga, Botao::Seletor) => self.sai_do_auto_teste(),

            // No passo da bomba o seletor é ignorado (evita pular o tempo
            // mínimo que o câmbio precisa para posicionar); quem atalha os
            // 60 s é o botão de nível.
            (Bomba, Botao::Nivel) => self.vai_para_passo(MotorCentrifuga),

            // Saída de emergência em qualquer passo. Não existia no
            // firmware original (lá só se saía tirando da tomada).
            (_, Botao::OnOff) => self.sai_do_auto_teste(),

            _ => {}
        }
    }

    /// Tique de 50 ms durante o auto-teste.
    fn auto_teste_tique(&mut self, passo: PassoAutoTeste, tiques: u16) {
        use PassoAutoTeste::*;

        match passo {
            AguardandoSoltura => {
                self.pisca_leds_de_nivel();
                // Só segue depois que os dois botões do gesto forem soltos,
                // senão o primeiro avanço aconteceria sozinho.
                if !self.botao_preso(Botao::Seletor) && !self.botao_preso(Botao::Nivel) {
                    self.vai_para_passo(ValvulaSabao);
                }
            }

            MotorAgitacao => {
                // Agitação sempre no padrão normal, sem turbo.
                let m = self.agitacao.tique(false);
                self.aplica_motores(m);
            }

            Bomba => {
                if tiques + 1 >= TIQUES_BOMBA_AUTO_TESTE {
                    self.vai_para_passo(MotorCentrifuga);
                    return;
                }
                self.estado = Estado::AutoTeste {
                    passo,
                    tiques: tiques + 1,
                };
            }

            // Nos demais passos o tempo não corre: só o botão avança.
            ValvulaSabao | ValvulaAmaciante | MotorCentrifuga => {}
        }
    }

    /// Entra num passo: desliga tudo e liga só o que aquele passo testa.
    fn vai_para_passo(&mut self, passo: PassoAutoTeste) {
        use PassoAutoTeste::*;

        self.at.desliga_todos();
        self.agitacao.reinicia();
        self.painel.apaga_tudo();
        self.pisca_aceso = false;
        self.pisca_contador = 0;

        match passo {
            AguardandoSoltura => {}
            ValvulaSabao => {
                self.painel.pesada.baixa();
                self.at.valvula_sabao(true);
            }
            ValvulaAmaciante => {
                self.painel.normal.baixa();
                self.at.valvula_amaciante(true);
            }
            MotorAgitacao => self.painel.rapido.baixa(),
            Bomba => {
                self.painel.enxaguar.baixa();
                self.at.eletrobomba(true);
            }
            MotorCentrifuga => {
                self.painel.centrifugar.baixa();
                // A bomba continua ligada, como no firmware original: o
                // câmbio já teve o passo anterior inteiro para posicionar.
                self.at.eletrobomba(true);
                self.at.motor_c(true);
            }
        }

        self.estado = Estado::AutoTeste { passo, tiques: 0 };
    }

    fn sai_do_auto_teste(&mut self) {
        self.at.desliga_todos();
        self.agitacao.reinicia();
        self.painel.apaga_tudo();
        self.estado = Estado::Desligada;
    }

    /// Este botão está pressionado agora? (espelho mantido pelos eventos)
    fn botao_preso(&self, b: Botao) -> bool {
        self.botoes & b.bit() != 0
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ERRO E AVISOS VISUAIS
    // ═════════════════════════════════════════════════════════════════════════

    fn entra_em_erro(&mut self, erro: Erro) {
        self.at.desliga_todos();
        self.agitacao.reinicia();
        self.painel.apaga_tudo();
        self.pisca_aceso = false;
        self.pisca_contador = 0;
        self.estado = Estado::Erro(erro);
        self.salva();
    }

    /// Pisca o painel inteiro no código do erro: N piscadas, pausa, repete.
    ///
    /// N vem de [`Erro::piscadas()`] — é o que o técnico conta em campo
    /// para saber o que falhou sem precisar de gravador nem display.
    ///
    /// Os 10 LEDs piscam juntos (não só os de programa) para a contagem
    /// ser visível de longe e não se confundir com a seleção de ciclo.
    ///
    /// ```text
    /// Drenar (4):  ▀▁▀▁▀▁▀▁________________▀▁▀▁▀▁▀▁___ ...
    ///              └── 4 × 500 ms ──┘└ 1,5 s ┘
    /// ```
    ///
    /// O contador anda um tique por chamada e dá a volta no fim do ciclo.
    /// O painel só é escrito quando o nível muda — entre duas transições
    /// não se toca em GPIO nenhum.
    fn pisca_codigo_de_erro(&mut self, erro: Erro) {
        let trem = erro.piscadas() * TIQUES_ERRO_PISCADA;
        let ciclo = trem + TIQUES_ERRO_PAUSA;

        let t = self.pisca_contador;
        self.pisca_contador = if t + 1 >= ciclo { 0 } else { t + 1 };

        // Aceso só dentro do trem de piscadas, na primeira metade de cada uma.
        let aceso = t < trem && (t % TIQUES_ERRO_PISCADA) < TIQUES_ERRO_ACESO;
        if aceso == self.pisca_aceso {
            return;
        }
        self.pisca_aceso = aceso;
        if aceso {
            self.painel.acende_tudo();
        } else {
            self.painel.apaga_tudo();
        }
    }

    /// Pisca os dois LEDs de nível — sinal de "solte os botões" no auto-teste.
    fn pisca_leds_de_nivel(&mut self) {
        if !self.avanca_pisca() {
            return;
        }
        // Lógica invertida: nível BAIXO acende o LED.
        let nivel = !self.pisca_aceso;
        self.painel.nivel_medio.escreve(nivel);
        self.painel.nivel_alto.escreve(nivel);
    }

    /// Pisca só o LED do programa — sinal de tampa aberta.
    fn pisca_led_do_programa(&mut self) {
        if !self.avanca_pisca() {
            return;
        }
        if self.pisca_aceso {
            self.painel.mostra_programa(self.programa);
        } else {
            self.painel.apaga_programas();
        }
    }

    /// Conta os tiques do pisca. Devolve `true` quando é hora de trocar.
    fn avanca_pisca(&mut self) -> bool {
        self.pisca_contador += 1;
        if self.pisca_contador < TIQUES_PISCA {
            return false;
        }
        self.pisca_contador = 0;
        self.pisca_aceso = !self.pisca_aceso;
        true
    }

    // ═════════════════════════════════════════════════════════════════════════
    // AUXILIARES
    // ═════════════════════════════════════════════════════════════════════════

    fn nivel_selecionado_atingido(&self) -> bool {
        if self.opcoes.nivel_alto {
            self.nivel_medio && self.nivel_alto
        } else {
            self.nivel_medio
        }
    }
}
