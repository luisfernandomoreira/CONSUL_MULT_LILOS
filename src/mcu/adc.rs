//! ADC do GD32C231 — leitura dos botões do painel.
//!
//! Na CONSUL MULT os três botões (Pausa, Roupas Mais Secas e Enxágue
//! Duplo) não têm um pino cada: estão num divisor resistivo ligado a
//! **PA6**, e o firmware identifica qual foi apertado pela faixa de
//! tensão lida. É o mesmo esquema do firmware C++ (ADC1_IN6), e as
//! faixas de `botoes.rs` são as mesmas — por isso a resolução aqui é
//! fixada em **10 bits** (0..1023), como no original.
//!
//! O ADC é usado em modo de disparo por software: a tarefa de
//! amostragem pede uma conversão a cada 80 ms e espera o fim dela
//! (dezenas de microssegundos com o tempo de amostragem mais longo).
//!
//! ⚠ A ligação PA6 → canal 6 do ADC vem do firmware C++ (`ADC1_IN6`
//! no STM32C031, mesma pinagem). Confira no User Manual do GD32C231
//! antes de confiar nos números em bancada.

use super::pac::{Adc, Rcu};

/// Canal do ADC ligado a PA6 (os botões do painel).
pub const CANAL_BOTOES: u8 = 6;

/// Resolução de 10 bits — mantém as faixas de `botoes.rs` idênticas
/// às do firmware C++.
const DRES_10_BITS: u8 = 0b01;

/// Tempo de amostragem mais longo disponível. O divisor resistivo do
/// painel tem impedância alta, então vale usar o máximo.
const SPT_MAIS_LONGO: u8 = 0b111;

/// Liga o clock do ADC e deixa o periférico pronto para conversões
/// por software no canal dos botões.
///
/// O clock do ADC fica no valor de reset: é uma configuração válida
/// garantida pelo hardware, e o tempo de amostragem longo compensa.
pub fn inicializa(rcu: &Rcu, adc: &Adc) {
    rcu.apben().modify(|_, w| w.adcen().set_bit());

    // Resolução de 10 bits e uma única conversão na sequência regular.
    // SAFETY: 0b01 é um valor válido do campo DRES (2 bits).
    adc.ctl0()
        .modify(|_, w| unsafe { w.dres().bits(DRES_10_BITS) });

    // RL = 0 → sequência regular com 1 conversão.
    // SAFETY: 0 é o comprimento mínimo documentado do campo.
    adc.rsq0().modify(|_, w| unsafe { w.rl().bits(0) });

    // Primeira (e única) conversão da sequência: o canal dos botões.
    // SAFETY: CANAL_BOTOES = 6 cabe nos 4 bits do campo RSQ0.
    adc.rsq2()
        .modify(|_, w| unsafe { w.rsq0().bits(CANAL_BOTOES) });

    // Tempo de amostragem do canal 6.
    // SAFETY: 0b111 é um valor válido do campo SPT6 (3 bits).
    adc.sampt1()
        .modify(|_, w| unsafe { w.spt6().bits(SPT_MAIS_LONGO) });

    // Liga o conversor.
    adc.ctl1().modify(|_, w| w.adcon().set_bit());
}

/// Dispara uma conversão e devolve o valor de 10 bits.
///
/// Devolve `None` se o ADC não terminar — o chamador mantém então a
/// última leitura boa, como faz o firmware C++.
pub fn converte(adc: &Adc) -> Option<u16> {
    // Limpa um EOC pendente de uma conversão anterior.
    // EOC é "escreve 0 para limpar" neste periférico.
    adc.stat().write(|w| w.eoc().clear_bit());

    adc.ctl1().modify(|_, w| w.swrcst().set_bit());

    // O mesmo limite do firmware C++: evita travar se o ADC parar.
    const TENTATIVAS: u32 = 10_000;
    for _ in 0..TENTATIVAS {
        if adc.stat().read().eoc().bit_is_set() {
            let valor = adc.rdata().read().rdata().bits();
            adc.stat().write(|w| w.eoc().clear_bit());
            return Some(valor);
        }
    }
    None
}
