//! Relógio do sistema.
//!
//! Após o reset o GD32C231 roda em CK_IRC48M / 4 = 12 MHz.
//! `configura_48mhz()` tira o divisor e deixa o núcleo em 48 MHz.
//!
//! Ordem obrigatória (User Manual, Tabela 2-2):
//!   1. aumentar o wait-state da flash  (WSCNT = 1 para > 24 MHz)
//!   2. só então aumentar a frequência

use super::pac::{Fmc, Rcu};

/// Frequência do núcleo depois de `configura_48mhz()`. Usada pelo SysTick do lilos.
pub const FREQUENCIA_HZ: u32 = 48_000_000;

/// Leva o sistema de 12 MHz (reset) para 48 MHz.
pub fn configura_48mhz(fmc: &Fmc, rcu: &Rcu) {
    // 1 wait-state: obrigatório acima de 24 MHz. Liga também o pre-fetch.
    // SAFETY: WSCNT = 1 é um valor válido de 3 bits (User Manual, Tab. 2-2).
    fmc.ws()
        .modify(|_, w| unsafe { w.wscnt().bits(1) }.pfen().set_bit());

    // IRC48MDIV_SYS = 000 → CK_SYS = IRC48M / 1 = 48 MHz.
    // SAFETY: 000 = divisor por 1, valor documentado do campo.
    rcu.ctl0()
        .modify(|_, w| unsafe { w.irc48mdiv_sys().bits(0b000) });

    // AHBPSC = 0 (/1) e APBPSC = 0 (/1) — já é o valor de reset, escrito
    // aqui de forma explícita para o firmware não depender dele.
    // SAFETY: 0 em ambos = sem divisão, que é o valor de reset.
    rcu.cfg0()
        .modify(|_, w| unsafe { w.ahbpsc().bits(0b0000).apbpsc().bits(0b000) });
}

/// Liga o clock das portas GPIO usadas pela placa (A, B e C).
pub fn habilita_gpio(rcu: &Rcu) {
    rcu.ahb2en()
        .modify(|_, w| w.paen().set_bit().pben().set_bit().pcen().set_bit());
}

/// Liga o clock do SYSCFG (necessário só se um dia for usar EXTI).
#[allow(dead_code)]
pub fn habilita_syscfg(rcu: &Rcu) {
    rcu.apben().modify(|_, w| w.syscfgen().set_bit());
}
