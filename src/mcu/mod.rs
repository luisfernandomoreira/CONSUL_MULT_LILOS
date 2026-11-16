//! =============================================================
//! Camada MCU — GD32C231K8T6 (Arm Cortex-M23, 48 MHz)
//!
//! Esta é a ÚNICA parte do firmware que toca em periférico.
//! Tudo acima dela (placa, eventos, tarefas) é código portável.
//!
//! O acesso é feito pelo PAC `gd32c2x1-pac`, gerado por svd2rust
//! a partir do SVD oficial da GigaDevice (veja `svd/README.md`).
//! Nenhum endereço ou deslocamento de registrador é escrito à mão
//! aqui: todos vêm do PAC.
//! =============================================================

pub mod clock;
pub mod flash;
pub mod gpio;
pub mod wdt;

/// O PAC, reexportado para o resto do firmware não precisar
/// declarar a dependência outra vez.
pub use gd32c2x1_pac as pac;

/// Substitui um campo de `largura` bits que começa em `deslocamento`.
///
/// Usado nos registradores do GPIO que têm 2 bits por pino (CTL, PUD,
/// OSPD): como o número do pino só é conhecido em tempo de execução,
/// não dá para usar os acessores `ctl0()`..`ctl15()` do PAC.
#[inline(always)]
pub const fn campo(valor_atual: u32, deslocamento: u32, largura: u32, novo: u32) -> u32 {
    let mascara = ((1u32 << largura) - 1) << deslocamento;
    (valor_atual & !mascara) | ((novo << deslocamento) & mascara)
}
