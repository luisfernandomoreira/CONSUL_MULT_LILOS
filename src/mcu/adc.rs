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
//! amostragem pede uma conversão a cada 81 ms e espera o fim dela.
//!
//! ─── Três detalhes que o GD32 exige ──────────────────────────
//!
//! Nenhum deles aparece se a gente só traduzir o código do STM32, e
//! qualquer um sozinho faz o ADC devolver lixo constante:
//!
//! 1. **CK_ADC tem limite de 24 MHz.** O núcleo roda a 48 MHz, então o
//!    `ADCPSC` precisa dividir. Aqui dividimos por 4 → 12 MHz.
//! 2. **O disparo por software exige `ETSRC = 111` e `ETERC = 1`.**
//!    Sem isso, escrever `SWRCST` não inicia conversão nenhuma.
//! 3. **Depois de ligar o `ADCON`, a parte analógica precisa
//!    estabilizar** antes da primeira conversão.
//!
//! Fonte: GD32C2x1 User Manual Rev1.2, seção 11 — a mesma sequência
//! usada no projeto BWC07_RTIC, que roda nesta mesma família.

use super::pac::{Adc, Rcu};

/// Canal do ADC ligado a PA6 (os botões do painel).
///
/// Confirmado contra o GD32C2x1: a numeração acompanha o pino, com
/// IN0..IN7 em PA0..PA7. Coincide com o `ADC1_IN6` do STM32C031 do
/// firmware C++.
pub const CANAL_BOTOES: u8 = 6;

/// Canal da **referência interna de tensão** (VREFINT).
///
/// É uma tensão fixa do próprio chip, medida contra a VDDA — ou seja,
/// contra a fonte de 3,3 V, que é a referência do ADC. Por ser fixa, a
/// leitura dela **se move quando a fonte se move**, e é isso que a
/// torna útil: dá para saber se a fonte desta placa está diferente da
/// de outra, sem voltímetro.
///
/// Serve de aferição, não de referência: o conversor continua medindo
/// contra a VDDA.
///
/// ⚠ O número do canal vem da mesma fonte do canal dos botões. Se a
/// leitura vier absurda, confira no User Manual.
#[cfg_attr(not(feature = "diag-botoes"), allow(dead_code))]
pub const CANAL_VREFINT: u8 = 14;

/// Quantos canais a sequência regular endereça (campo RSQ de 4 bits,
/// e SPT0..SPT15 nos registradores de tempo de amostragem).
///
/// Usado pela varredura de diagnóstico.
#[cfg_attr(not(feature = "diag-botoes"), allow(dead_code))]
pub const NUM_CANAIS: u8 = 16;

/// Resolução de 10 bits — mantém as faixas de `botoes.rs` idênticas às
/// do firmware C++. (00 = 12 bits, 01 = 10, 10 = 8, 11 = 6.)
const DRES_10_BITS: u8 = 0b01;

/// Valor de `ETSRC` que seleciona o disparo por software.
const ETSRC_SOFTWARE: u8 = 0b111;

/// `ADCPSC` = CK_SYS ÷ 4. Com o núcleo a 48 MHz dá **12 MHz**, dentro
/// do limite de 24 MHz do periférico.
const ADCPSC_DIV4: u8 = 0b0010;

/// Tempo de amostragem mais longo (160,5 ciclos de CK_ADC) em todos os
/// 16 canais. O divisor resistivo do painel tem impedância alta, então
/// vale usar o máximo.
///
/// Os campos SPT são de 3 bits, lado a lado: SPT0..SPT9 em SAMPT1
/// (bits 0..29) e SPT10..SPT15 em SAMPT0 (bits 0..17).
const TODOS_SPT_SAMPT1: u32 = 0x3FFF_FFFF; // 10 campos de 0b111
const TODOS_SPT_SAMPT0: u32 = 0x0003_FFFF; // 6 campos de 0b111

/// Ciclos de núcleo de espera depois de ligar a parte analógica.
/// O manual pede ~1 µs; a 48 MHz, 2000 ciclos dão ~42 µs de folga.
const ESPERA_ESTABILIZACAO: u32 = 2_000;

/// Ciclos de núcleo entre o EOC e a leitura do dado. O manual exige um
/// ciclo de CK_ADC (12 MHz → 4 ciclos de núcleo); 40 dão folga.
const ESPERA_DADO: u32 = 40;

/// Liga o clock do ADC e deixa o periférico pronto para conversões por
/// software no canal dos botões.
pub fn inicializa(rcu: &Rcu, adc: &Adc) {
    // ─── 1. Clock do ADC: CK_SYS ÷ 4 = 12 MHz ───────────────────────────────
    // O periférico não aceita mais que 24 MHz. Sem esta divisão o ADC
    // roda a 48 MHz, fora de especificação, e devolve lixo.
    // SAFETY: 0b0010 é o código documentado de divisão por 4.
    rcu.cfg1()
        .modify(|_, w| unsafe { w.adcpsc().bits(ADCPSC_DIV4) }.adcsel().clear_bit());

    rcu.apben().modify(|_, w| w.adcen().set_bit());

    // ─── 2. Resolução e modo de conversão ───────────────────────────────────
    // Uma conversão por disparo: sem varredura, sem descontínuo.
    // SAFETY: 0b01 é um valor válido do campo DRES (2 bits).
    adc.ctl0().modify(|_, w| {
        unsafe { w.dres().bits(DRES_10_BITS) }
            .sm()
            .clear_bit()
            .disrc()
            .clear_bit()
    });

    // ─── 3. Disparo por software ────────────────────────────────────────────
    // O SWRCST só age com ETSRC = 111 **e** o gatilho externo habilitado.
    // SAFETY: 0b111 é o valor documentado para disparo por software.
    adc.ctl1().modify(|_, w| {
        unsafe { w.etsrc().bits(ETSRC_SOFTWARE) }
            .eterc()
            .set_bit()
            .ctn()
            .clear_bit()
            .dma()
            .clear_bit()
            .dal()
            .clear_bit()
    });

    // ─── 4. Sequência regular com uma conversão ─────────────────────────────
    // SAFETY: 0 é o comprimento mínimo documentado do campo.
    adc.rsq0().modify(|_, w| unsafe { w.rl().bits(0) });

    // ─── 5. Tempo de amostragem máximo em todos os canais ───────────────────
    // Só o canal dos botões é lido em produção, mas configurar todos
    // deixa `converte_canal` pronta para qualquer um — é o que a
    // varredura de diagnóstico usa.
    // SAFETY: 0b111 é um valor válido de cada campo SPT.
    adc.sampt1()
        .modify(|_, w| unsafe { w.bits(TODOS_SPT_SAMPT1) });
    adc.sampt0()
        .modify(|_, w| unsafe { w.bits(TODOS_SPT_SAMPT0) });

    // ─── 6. Liga e deixa a parte analógica estabilizar ──────────────────────
    adc.ctl1().modify(|_, w| w.adcon().set_bit());
    cortex_m::asm::delay(ESPERA_ESTABILIZACAO);
}

/// Dispara uma conversão no canal dos botões.
///
/// Devolve `None` se o ADC não terminar — o chamador mantém então a
/// última leitura boa, como faz o firmware C++.
pub fn converte(adc: &Adc) -> Option<u16> {
    converte_canal(adc, CANAL_BOTOES)
}

/// Mede a referência interna de tensão.
///
/// Liga o caminho interno do VREFINT, espera a estabilização que o
/// manual pede e converte o canal 14. O caminho fica ligado depois —
/// não custa nada e evita a espera na próxima leitura.
///
/// Como ler o resultado: a leitura é `VREFINT / VDDA`, então ela sobe
/// quando a fonte **cai**. Comparando duas placas:
///
/// | VREFINT | Patamar do botão | Conclusão |
/// |---|---|---|
/// | igual | igual | as fontes são iguais; nada a discutir |
/// | **muda** | igual | a fonte varia, mas o divisor acompanha — leitura **ratiométrica**, as faixas estão seguras |
/// | **muda** | **muda junto, ao contrário** | o divisor **não** acompanha a fonte: as faixas vão errar de placa para placa |
#[cfg(feature = "diag-botoes")]
pub fn converte_vrefint(adc: &Adc) -> Option<u16> {
    let ja_ligado = adc.ctl1().read().inrefen().bit_is_set();
    if !ja_ligado {
        adc.ctl1().modify(|_, w| w.inrefen().set_bit());
        // O buffer da referência interna leva alguns microssegundos.
        cortex_m::asm::delay(ESPERA_ESTABILIZACAO);
    }
    converte_canal(adc, CANAL_VREFINT)
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

    // Limpa um EOC pendente. Os flags deste periférico são "escreve 0
    // para limpar", e o `modify` preserva os demais.
    adc.stat().modify(|_, w| w.eoc().clear_bit());

    adc.ctl1().modify(|_, w| w.swrcst().set_bit());

    // O mesmo limite do firmware C++: evita travar se o ADC parar.
    const TENTATIVAS: u32 = 10_000;
    for _ in 0..TENTATIVAS {
        if adc.stat().read().eoc().bit_is_set() {
            // O dado só é válido um ciclo de CK_ADC depois do EOC.
            cortex_m::asm::delay(ESPERA_DADO);
            let valor = adc.rdata().read().rdata().bits();
            adc.stat().modify(|_, w| w.eoc().clear_bit());
            return Some(valor);
        }
    }
    None
}
