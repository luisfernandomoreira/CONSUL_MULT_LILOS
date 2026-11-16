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
///
/// ⚠ Vem do `ADC1_IN6` do STM32C031 (a pinagem é a mesma), **não** do
/// manual do GD32C231. Se os botões não responderem, é o primeiro
/// suspeito: o build `--features diag-botoes` varre os 16 canais e
/// pisca o número daquele que se move quando um botão é apertado.
pub const CANAL_BOTOES: u8 = 6;

/// Quantos canais a sequência regular endereça (campo RSQ de 4 bits,
/// e SPT0..SPT15 nos registradores de tempo de amostragem).
///
/// Usado pela varredura de diagnóstico.
#[cfg_attr(not(feature = "diag-botoes"), allow(dead_code))]
pub const NUM_CANAIS: u8 = 16;

/// Resolução de 10 bits — mantém as faixas de `botoes.rs` idênticas
/// às do firmware C++.
const DRES_10_BITS: u8 = 0b01;

/// Tempo de amostragem mais longo disponível (valor de cada campo SPT).
/// O divisor resistivo do painel tem impedância alta, então vale usar
/// o máximo.
#[allow(dead_code)]
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

    // Tempo de amostragem mais longo em TODOS os 16 canais.
    //
    // Só o canal dos botões é lido em produção, mas configurar todos
    // deixa `converte_canal` pronta para qualquer um — é o que a
    // varredura de diagnóstico usa para descobrir em que canal o PA6
    // realmente está.
    //
    // Os campos SPT são de 3 bits, lado a lado: SPT0..SPT9 em SAMPT1
    // (bits 0..29) e SPT10..SPT15 em SAMPT0 (bits 0..17).
    const TODOS_SPT_SAMPT1: u32 = 0x3FFF_FFFF; // 10 campos de 0b111
    const TODOS_SPT_SAMPT0: u32 = 0x0003_FFFF; // 6 campos de 0b111
    // SAFETY: 0b111 é um valor válido de cada campo SPT.
    adc.sampt1()
        .modify(|_, w| unsafe { w.bits(TODOS_SPT_SAMPT1) });
    adc.sampt0()
        .modify(|_, w| unsafe { w.bits(TODOS_SPT_SAMPT0) });

    // Liga o conversor.
    adc.ctl1().modify(|_, w| w.adcon().set_bit());
}

/// Dispara uma conversão no canal dos botões.
///
/// Devolve `None` se o ADC não terminar — o chamador mantém então a
/// última leitura boa, como faz o firmware C++.
pub fn converte(adc: &Adc) -> Option<u16> {
    converte_canal(adc, CANAL_BOTOES)
}

/// Dispara uma conversão no canal pedido (0..15).
///
/// Não tem `await` no meio: entre escolher o canal e ler o resultado
/// nenhuma outra tarefa roda, então duas tarefas podem usar o ADC em
/// canais diferentes sem atropelar uma à outra.
pub fn converte_canal(adc: &Adc, canal: u8) -> Option<u16> {
    // Primeira (e única) conversão da sequência regular.
    // SAFETY: o chamador passa 0..15, que cabe nos 4 bits do campo.
    adc.rsq2().modify(|_, w| unsafe { w.rsq0().bits(canal) });

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
