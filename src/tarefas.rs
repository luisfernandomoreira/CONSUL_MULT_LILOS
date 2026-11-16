//! =============================================================
//! TAREFAS — as cinco rotinas concorrentes do lilos
//!
//!   entradas  (10 ms) → lê botões e sensores, publica eventos
//!   tempo     (50 ms) → publica Tique e, a cada 20, Segundo
//!   maquina   (evento)→ consome a fila e decide o que fazer
//!   gravacao  (pedido)→ grava o ciclo na flash, fora da máquina
//!   watchdog  (1 s)   → alimenta o FWDGT
//!
//! Todas são `async fn -> Infallible`: nunca terminam, é o que o
//! executor do lilos exige.
//! =============================================================

use core::convert::Infallible;

use lilos::time::{Millis, PeriodicGate};

use crate::estado_salvo::{self, EstadoSalvo};
use crate::evento::{publica, Botao, Evento};
use crate::maquina::Maquina;
use crate::mcu::pac::{Fmc, Fwdgt, Rcu};
use crate::mcu::wdt;
use crate::placa::{Atuadores, Entradas, Painel};

// ═════════════════════════════════════════════════════════════════════════════
// FILTRO DE RUÍDO (debounce)
// ═════════════════════════════════════════════════════════════════════════════

/// Só aceita uma mudança depois de `limite` leituras iguais seguidas.
///
/// Com amostragem de 10 ms: limite 5 = 50 ms, limite 20 = 200 ms.
struct Debounce {
    estavel: bool,
    contador: u8,
    limite: u8,
}

impl Debounce {
    const fn novo(inicial: bool, limite: u8) -> Self {
        Self {
            estavel: inicial,
            contador: 0,
            limite,
        }
    }

    /// Devolve `Some(novo_estado)` só no instante em que a mudança é aceita.
    fn amostra(&mut self, leitura: bool) -> Option<bool> {
        if leitura == self.estavel {
            self.contador = 0;
            return None;
        }
        self.contador += 1;
        if self.contador < self.limite {
            return None;
        }
        self.contador = 0;
        self.estavel = leitura;
        Some(leitura)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: ENTRADAS
// ═════════════════════════════════════════════════════════════════════════════

/// Período de amostragem das entradas.
const AMOSTRAGEM_MS: u64 = 10;
/// 50 ms de estabilidade para aceitar um botão.
const DEBOUNCE_BOTAO: u8 = 5;
/// 200 ms de estabilidade para a tampa e os sensores de nível.
const DEBOUNCE_SENSOR: u8 = 20;

/// Varre botões e sensores e transforma cada mudança em um evento.
///
/// Note que NÃO usamos EXTI: amostrar a 10 ms já dá o debounce de graça
/// e dispensa configurar NVIC, SYSCFG e linhas de interrupção. Menos
/// hardware envolvido, menos coisa para dar errado.
pub async fn entradas(e: &Entradas) -> Infallible {
    let mut bt: [Debounce; 6] = core::array::from_fn(|i| {
        Debounce::novo(e.botao_pressionado(Botao::TODOS[i]), DEBOUNCE_BOTAO)
    });

    let mut tampa = Debounce::novo(e.tampa_aberta(), DEBOUNCE_SENSOR);
    let mut nivel_medio = Debounce::novo(e.nivel_medio_atingido(), DEBOUNCE_SENSOR);
    let mut nivel_alto = Debounce::novo(e.nivel_alto_atingido(), DEBOUNCE_SENSOR);

    // O combo do auto-teste é publicado uma vez só; rearma quando um dos
    // dois botões é solto.
    let mut combo_armado = true;

    let mut relogio = PeriodicGate::from(Millis(AMOSTRAGEM_MS));

    loop {
        relogio.next_time().await;

        // ── Botões ───────────────────────────────────────────────────────────
        for &botao in Botao::TODOS.iter() {
            let i = botao.indice();
            if let Some(pressionado) = bt[i].amostra(e.botao_pressionado(botao)) {
                publica(if pressionado {
                    Evento::BotaoPressionado(botao)
                } else {
                    Evento::BotaoSolto(botao)
                });
            }
        }

        // ── Combinação Seletor + Nível (abre o auto-teste) ───────────────────
        let combo = bt[Botao::Seletor.indice()].estavel && bt[Botao::Nivel.indice()].estavel;
        if combo && combo_armado {
            publica(Evento::ComboAutoTeste);
            combo_armado = false;
        } else if !combo {
            combo_armado = true;
        }

        // ── Tampa e sensores de nível ────────────────────────────────────────
        if let Some(aberta) = tampa.amostra(e.tampa_aberta()) {
            publica(Evento::Tampa { aberta });
        }
        if let Some(atingido) = nivel_medio.amostra(e.nivel_medio_atingido()) {
            publica(Evento::NivelMedio { atingido });
        }
        if let Some(atingido) = nivel_alto.amostra(e.nivel_alto_atingido()) {
            publica(Evento::NivelAlto { atingido });
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: TEMPO
// ═════════════════════════════════════════════════════════════════════════════

/// Base de tempo do ciclo de agitação — 50 ms, igual ao firmware original.
const TIQUE_MS: u64 = 50;
const TIQUES_POR_SEGUNDO: u8 = (1_000 / TIQUE_MS) as u8;

/// Publica `Tique` a cada 50 ms e `Segundo` a cada 20 tiques.
///
/// Os dois saem da mesma base, então nunca ficam fora de fase.
pub async fn tempo() -> Infallible {
    let mut relogio = PeriodicGate::from(Millis(TIQUE_MS));
    let mut contador: u8 = 0;

    loop {
        relogio.next_time().await;
        publica(Evento::Tique);

        contador += 1;
        if contador >= TIQUES_POR_SEGUNDO {
            contador = 0;
            publica(Evento::Segundo);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: MÁQUINA DE ESTADOS
// ═════════════════════════════════════════════════════════════════════════════

/// Consome a fila de eventos para sempre. É só isto:
/// `evento → maquina.trata(evento)`.
///
/// A animação de inicialização do painel NÃO está aqui: ela é o estado
/// `Inicializando` da máquina, movido pelos mesmos `Tique`. Se esta
/// tarefa parasse para animar, os eventos publicados nesse meio-tempo
/// encheriam a fila e seriam descartados logo no boot.
pub async fn maquina(
    painel: &Painel,
    at: &Atuadores,
    entradas: &Entradas,
    salvo: Option<EstadoSalvo>,
) -> Infallible {
    let mut maquina = Maquina::nova(painel, at, entradas);
    if let Some(estado) = salvo {
        maquina.restaura(estado);
    }

    loop {
        let evento = crate::evento::proximo().await;
        maquina.trata(evento);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: GRAVAÇÃO NA FLASH
// ═════════════════════════════════════════════════════════════════════════════

/// Grava o estado do ciclo na flash quando a máquina pede.
///
/// Fica aqui, e não dentro da máquina de estados, porque gravar na flash
/// trava o núcleo por alguns milissegundos. Nesta tarefa isso acontece
/// entre dois eventos, sem atrasar a decisão da máquina.
pub async fn gravacao(fmc: &Fmc) -> Infallible {
    loop {
        let estado = estado_salvo::proximo_pedido().await;
        if !estado_salvo::grava(fmc, estado) {
            lilos::time::sleep_for(Millis(100)).await;
            estado_salvo::repete_gravacao(estado);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TAREFA: WATCHDOG
// ═════════════════════════════════════════════════════════════════════════════

/// Alimenta o FWDGT a cada 1 s. O estouro é de 8 s: se qualquer tarefa
/// travar, o executor para de rodar, esta tarefa para de alimentar e o
/// MCU reinicia sozinho.
pub async fn watchdog(wdt_periferico: &Fwdgt, rcu: &Rcu) -> Infallible {
    wdt::inicia_8s(wdt_periferico, rcu).await;
    let mut relogio = PeriodicGate::from(Millis(1_000));

    loop {
        relogio.next_time().await;
        wdt::alimenta(wdt_periferico);
    }
}
