//! =============================================================
//! Diagnóstico — espiar o ADC com a placa rodando
//!
//! Esta placa não tem UART nem display, então não há como imprimir
//! nada. O jeito de ver o que o ADC está lendo é deixar os valores em
//! endereços de RAM com **símbolo fixo** (`#[no_mangle]`) e lê-los por
//! SWD com o `probe-rs`, sem parar o núcleo.
//!
//! Nada aqui é lido pela lógica do firmware: são só quatro contadores
//! de observação. Custam 8 bytes de RAM e algumas instruções a cada
//! 81 ms.
//!
//! ─── Como usar ───────────────────────────────────────────────
//!
//! Levante os endereços no ELF (eles mudam a cada compilação):
//!
//! ```bash
//! arm-none-eabi-nm target/thumbv6m-none-eabi/release/consul_mult_lilos \
//!     | grep -i DIAG_
//! ```
//!
//! Com a placa rodando, leia os quatro de uma vez:
//!
//! ```bash
//! probe-rs read b16 0x2000XXXX 4 \
//!     --chip GD32C231K8 \
//!     --chip-description-path target-probe/GD32C231K8.yaml
//! ```
//!
//! ─── Como interpretar ────────────────────────────────────────
//!
//! | O que você vê | O que significa |
//! |---|---|
//! | `FALHA` subindo e `OK` em 0 | o ADC não converte — canal, clock ou habilitação |
//! | `OK` subindo e `AD` sempre 0 | converte, mas o resultado vem zero — entrada ou canal errado |
//! | `MIN`/`MAX` em repouso fora de 7..60 | a faixa de repouso não bate; **nenhum botão funciona** |
//! | `MIN`/`MAX` em repouso dentro de 7..60 | o ADC está bom; o problema é outro |
//!
//! Para descobrir o valor de um botão: segure-o e leia `AD` algumas
//! vezes. Para a faixa de repouso: religue a placa, não toque em nada
//! e leia `MIN` e `MAX`.
//! =============================================================

use core::ptr::addr_of_mut;

/// Última leitura crua do ADC dos botões.
#[no_mangle]
pub static mut DIAG_AD: u16 = 0;

/// Menor leitura vista desde a partida.
#[no_mangle]
pub static mut DIAG_AD_MIN: u16 = u16::MAX;

/// Maior leitura vista desde a partida.
#[no_mangle]
pub static mut DIAG_AD_MAX: u16 = 0;

/// Quantas conversões terminaram bem.
#[no_mangle]
pub static mut DIAG_ADC_OK: u16 = 0;

/// Quantas conversões estouraram o tempo limite.
#[no_mangle]
pub static mut DIAG_ADC_FALHA: u16 = 0;

/// Registra uma conversão bem-sucedida.
///
/// Chamada só pela tarefa de amostragem, que é a única escritora — e,
/// num executor cooperativo de um núcleo, não há outra tarefa no meio
/// da escrita.
pub fn registra_leitura(valor: u16) {
    // SAFETY: escrita volátil num `static mut` que ninguém mais
    // escreve. O `probe-rs` lê por SWD, fora do núcleo.
    unsafe {
        core::ptr::write_volatile(addr_of_mut!(DIAG_AD), valor);

        let min = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MIN));
        if valor < min {
            core::ptr::write_volatile(addr_of_mut!(DIAG_AD_MIN), valor);
        }

        let max = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MAX));
        if valor > max {
            core::ptr::write_volatile(addr_of_mut!(DIAG_AD_MAX), valor);
        }

        let ok = core::ptr::read_volatile(addr_of_mut!(DIAG_ADC_OK));
        core::ptr::write_volatile(addr_of_mut!(DIAG_ADC_OK), ok.wrapping_add(1));
    }
}

/// Registra uma conversão que não terminou.
pub fn registra_falha() {
    // SAFETY: idem `registra_leitura`.
    unsafe {
        let n = core::ptr::read_volatile(addr_of_mut!(DIAG_ADC_FALHA));
        core::ptr::write_volatile(addr_of_mut!(DIAG_ADC_FALHA), n.wrapping_add(1));
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Leitura pelo próprio firmware (modo de diagnóstico dos botões)
// ═════════════════════════════════════════════════════════════════════════════

/// Quantas conversões já terminaram bem.
///
/// O modo de diagnóstico usa isto para saber se o ADC está vivo: se o
/// contador não anda, não é faixa errada — é conversão não acontecendo.
#[cfg(feature = "diag-botoes")]
pub fn conversoes_ok() -> u16 {
    // SAFETY: leitura volátil de um `static mut` que só a tarefa de
    // amostragem escreve.
    unsafe { core::ptr::read_volatile(addr_of_mut!(DIAG_ADC_OK)) }
}

/// Diferenca entre a maior e a menor leitura desde a partida.
///
/// Se isto fica perto de zero depois de alguem apertar os botoes, o
/// ADC nao esta vendo o divisor resistivo: pino, canal ou modo
/// analogico.
#[cfg(feature = "diag-botoes")]
pub fn amplitude() -> u16 {
    // SAFETY: leituras volateis de `static mut` que so a tarefa de
    // amostragem escreve.
    unsafe {
        let min = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MIN));
        let max = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MAX));
        max.saturating_sub(min)
    }
}
