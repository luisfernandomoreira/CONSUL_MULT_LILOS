//! FMC — gravação na flash interna do GD32C231.
//!
//! Usado para guardar o estado do ciclo e retomá-lo depois de uma queda
//! de energia. Só a ÚLTIMA página de 1 KB é usada (0x0800_FC00);
//! o `memory.x` reserva essa página deixando o código em 63 KB.
//!
//! Regras do GD32C2x1 (User Manual, seção 2.3):
//!   • a gravação é sempre de DOUBLE WORD — 2 escritas de 32 bits;
//!   • só se pode gravar em endereço apagado (0xFFFF_FFFF_FFFF_FFFF);
//!   • apagar é por página inteira de 1 KB.
//!
//! Enquanto a FMC grava ou apaga, o núcleo fica parado esperando a
//! flash — por isso a gravação mora na sua própria tarefa e nunca
//! dentro da máquina de estados.

use super::pac::Fmc;

const CHAVE1: u32 = 0x4567_0123;
const CHAVE2: u32 = 0xCDEF_89AB;

/// Início da flash principal.
pub const FLASH_INICIO: u32 = 0x0800_0000;
/// Tamanho de cada página.
pub const TAMANHO_PAGINA: u32 = 1024;

fn espera_livre(fmc: &Fmc) {
    while fmc.stat().read().busy().bit_is_set() {}
}

/// Limpa os flags de erro e o de fim de operação.
///
/// Todos são "escreve 1 para limpar", então `set_bit()` aqui significa
/// LIMPAR o flag — o SVD não marca esses campos como `w1c`, por isso o
/// PAC não gera um `clear_bit_by_one()`.
fn limpa_flags(fmc: &Fmc) {
    fmc.stat().write(|w| {
        w.oberr()
            .set_bit()
            .rperr()
            .set_bit()
            .fstperr()
            .set_bit()
            .pgserr()
            .set_bit()
            .pgmerr()
            .set_bit()
            .pgaerr()
            .set_bit()
            .wperr()
            .set_bit()
            .pgerr()
            .set_bit()
            .oprerr()
            .set_bit()
            .endf()
            .set_bit()
    });
}

fn destrava(fmc: &Fmc) {
    if fmc.ctl().read().lk().bit_is_set() {
        fmc.key().write(|w| unsafe { w.bits(CHAVE1) });
        fmc.key().write(|w| unsafe { w.bits(CHAVE2) });
    }
}

fn trava(fmc: &Fmc) {
    fmc.ctl().modify(|_, w| w.lk().set_bit());
}

/// Houve algum erro na última operação?
fn deu_erro(fmc: &Fmc) -> bool {
    let s = fmc.stat().read();
    s.oberr().bit_is_set()
        || s.rperr().bit_is_set()
        || s.fstperr().bit_is_set()
        || s.pgserr().bit_is_set()
        || s.pgmerr().bit_is_set()
        || s.pgaerr().bit_is_set()
        || s.wperr().bit_is_set()
        || s.pgerr().bit_is_set()
        || s.oprerr().bit_is_set()
}

/// Apaga uma página inteira (1 KB). `pagina` vai de 0 a 63.
///
/// Devolve `false` se a FMC sinalizar erro.
pub fn apaga_pagina(fmc: &Fmc, pagina: u8) -> bool {
    espera_livre(fmc);
    destrava(fmc);
    limpa_flags(fmc);

    // SAFETY: `pagina` é 0..63, dentro dos 6 bits do campo PN.
    fmc.ctl()
        .modify(|_, w| unsafe { w.pn().bits(pagina) }.per().set_bit());
    fmc.ctl().modify(|_, w| w.start().set_bit());
    espera_livre(fmc);

    let ok = !deu_erro(fmc);
    fmc.ctl().modify(|_, w| w.per().clear_bit());
    limpa_flags(fmc);
    trava(fmc);
    ok
}

/// Grava 8 bytes (duas palavras de 32 bits) em `endereco`.
///
/// `endereco` tem de ser múltiplo de 8 e estar apagado.
/// Devolve `false` se a FMC sinalizar erro.
pub fn grava_double_word(fmc: &Fmc, endereco: u32, palavra_baixa: u32, palavra_alta: u32) -> bool {
    espera_livre(fmc);
    destrava(fmc);
    limpa_flags(fmc);

    fmc.ctl().modify(|_, w| w.pg().set_bit());
    // SAFETY: `endereco` é uma posição alinhada dentro da página reservada
    // em `memory.x`, fora da região ocupada pelo código.
    critical_section::with(|_| {
        // O par de escritas deve chegar à FMC sem interrupção entre elas.
        unsafe {
            core::ptr::write_volatile(endereco as *mut u32, palavra_baixa);
            cortex_m::asm::isb();
            core::ptr::write_volatile((endereco + 4) as *mut u32, palavra_alta);
        }
    });
    espera_livre(fmc);

    let ok = !deu_erro(fmc) && le(endereco) == palavra_baixa && le(endereco + 4) == palavra_alta;
    fmc.ctl().modify(|_, w| w.pg().clear_bit());
    limpa_flags(fmc);
    trava(fmc);
    ok
}

/// Lê uma palavra de 32 bits da flash — é acesso normal de memória,
/// não passa pela FMC.
pub fn le(endereco: u32) -> u32 {
    // SAFETY: leitura alinhada dentro da flash do próprio chip.
    unsafe { core::ptr::read_volatile(endereco as *const u32) }
}
