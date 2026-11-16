//! =============================================================
//! O sinal que acorda a máquina de estados
//!
//! O firmware C++ é um laço infinito: lê o hardware e chama a etapa
//! atual, sem parar. Aqui o papel do laço é da tarefa `maquina`, que
//! em vez de girar à toa espera por este sinal.
//!
//! Quem dispara:
//!
//!   tarefas::agitacao   a cada  21 ms  ← o mais rápido; é ele que dá
//!                                        a cadência da máquina
//!   tarefas::tempo      a cada   1 s
//!   tarefas::amostragem a cada  81 ms
//!
//! Vários sinais entre duas passadas valem um só: a máquina não perde
//! nada, porque o que ela lê — `relogio`, `agita` e `leituras` — é
//! estado, não mensagem. É de propósito que não há fila de eventos
//! aqui: um tique perdido numa fila cheia seria tempo de ciclo
//! perdido, e nenhum dos três avisos carrega dado que só exista nele.
//! =============================================================

use core::cell::Cell;
use critical_section::Mutex;
use lilos::exec::Notify;

static PENDENTE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static AVISO: Notify = Notify::new();

/// Pede uma passada da máquina de estados.
///
/// Pode ser chamada de qualquer tarefa e também de uma interrupção.
pub fn acorda() {
    critical_section::with(|cs| PENDENTE.borrow(cs).set(true));
    AVISO.notify();
}

/// Espera até alguém pedir uma passada.
pub async fn espera() {
    // `until_racy` assina o aviso ANTES de testar a marca, para um
    // pedido feito por uma interrupção nunca se perder.
    AVISO
        .until_racy(|| {
            critical_section::with(|cs| {
                if PENDENTE.borrow(cs).replace(false) {
                    Some(())
                } else {
                    None
                }
            })
        })
        .await
}
