//! =============================================================
//! Empresa  : Alpe Placas Eletrônicas — Divisão Metalúrgica Alado
//! Autor    : Luis Fernando Moreira
//! Projeto  : CONSUL MULT — Firmware Máquina de Lavar
//! Linguagem: Rust
//! RTOS     : lilos (executor async cooperativo)
//! Chip     : GD32C231K8T6 (Arm Cortex-M23, 64 KB flash, 12 KB RAM)
//! Placa    : a mesma PCI do BWG10/BWC07 — o GD32C231K8T6 (LQFP32) é
//!            pino a pino igual ao STM32C031K6T6 (LQFP32) anterior.
//!
//! ─── Como o firmware está organizado ─────────────────────────
//!
//!   mcu/       periféricos do GD32C231 via PAC (clock, GPIO, watchdog, flash)
//!   placa      nomes dos pinos da placa CONSUL MULT
//!   evento     a fila de eventos — o centro de tudo
//!   tarefas    as 5 tarefas do lilos
//!   maquina    a máquina de estados (lê evento, decide, age)
//!   programa   os ciclos de lavagem, em forma de tabela
//!   agitacao   o padrão A/C do motor
//!   opcoes     turbo, nível, amaciante, enxágue extra
//!   estado_salvo  grava o ciclo na flash para retomar após queda de luz
//!
//! Fluxo:
//!
//!   [botões/sensores] → tarefas::entradas ─┐
//!                                          ├→ FILA → tarefas::maquina
//!   [relógio 50 ms]   → tarefas::tempo   ──┘            │
//!                                                       ▼
//!                                            [triacs e LEDs do painel]
//! =============================================================

#![no_std]
#![no_main]
// Os mapas de registradores escrevem `BASE + 0x00` de propósito:
// a lista fica alinhada com a tabela do manual e é mais fácil de conferir.
#![allow(clippy::identity_op)]

use panic_halt as _; // panic → trava o núcleo; o FWDGT reinicia em 8 s

use core::pin::pin;
use cortex_m_rt::entry;

mod agitacao;
mod estado_salvo;
mod evento;
mod maquina;
mod mcu;
mod opcoes;
mod placa;
mod programa;
mod tarefas;

#[entry]
fn main() -> ! {
    // ─── 1. Periféricos do chip (singleton do PAC) ───────────────────────────
    let p = mcu::pac::Peripherals::take().unwrap();

    // ─── 2. Núcleo a 48 MHz ──────────────────────────────────────────────────
    mcu::clock::configura_48mhz(&p.fmc, &p.rcu);

    // ─── 3. Base de tempo do lilos (SysTick de 1 ms) ─────────────────────────
    let mut cp = cortex_m::Peripherals::take().unwrap();
    lilos::time::initialize_sys_tick(&mut cp.SYST, mcu::clock::FREQUENCIA_HZ);

    // ─── 4. Pinos da placa — triacs já saem daqui desligados ─────────────────
    let (painel, atuadores, entradas) = placa::inicializa(&p.rcu);

    // ─── 5. Estado do ciclo anterior, se a energia caiu no meio ──────────────
    let salvo = estado_salvo::carrega();

    // ─── 6. As cinco tarefas ─────────────────────────────────────────────────
    // `pin!` fixa cada future na pilha do `main`, que nunca retorna —
    // por isso as tarefas podem emprestar `painel`, `atuadores` e `entradas`.
    let t_entradas = pin!(tarefas::entradas(&entradas));
    let t_tempo = pin!(tarefas::tempo());
    let t_maquina = pin!(tarefas::maquina(&painel, &atuadores, &entradas, salvo));
    let t_gravacao = pin!(tarefas::gravacao(&p.fmc));
    let t_watchdog = pin!(tarefas::watchdog(&p.fwdgt, &p.rcu));

    // ─── 7. Entrega o controle ao executor (nunca retorna) ───────────────────
    lilos::exec::run_tasks(
        &mut [t_entradas, t_tempo, t_maquina, t_gravacao, t_watchdog],
        lilos::exec::ALL_TASKS,
    )
}
