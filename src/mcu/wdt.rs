//! FWDGT — Free Watchdog Timer (o "cão de guarda" independente).
//!
//! Roda no oscilador interno de 32 kHz (IRC32K), que continua funcionando
//! mesmo se o clock principal parar. Se o firmware travar e ninguém chamar
//! `alimenta()` dentro do tempo configurado, o MCU é resetado.

use super::pac::{Fwdgt, Rcu};
use lilos::time::{sleep_for, Millis};

const CHAVE_LIBERA: u16 = 0x5555;
const CHAVE_RECARGA: u16 = 0xAAAA;
const CHAVE_INICIA: u16 = 0xCCCC;

/// Liga o watchdog com tempo de estouro de ~8 s.
///
/// IRC32K / 256 = 125 Hz → 1000 contagens = 8,0 s.
/// Mesmo tempo usado no firmware STM32 original.
pub async fn inicia_8s(wdt: &Fwdgt, rcu: &Rcu) {
    // As atualizações do FWDGT dependem do IRC32K, mesmo antes de iniciar
    // o contador. Habilita sua fonte antes de escrever PSC e RLD.
    rcu.rstsck().modify(|_, w| w.irc32ken().set_bit());
    while rcu.rstsck().read().irc32kstb().bit_is_clear() {
        sleep_for(Millis(1)).await;
    }

    // SAFETY: as três chaves abaixo são os comandos documentados do FWDGT.
    wdt.ctl().write(|w| unsafe { w.cmd().bits(CHAVE_LIBERA) });
    wdt.psc().write(|w| unsafe { w.psc().bits(0b110) }); // /256
    wdt.rld().write(|w| unsafe { w.rld().bits(1000) }); // 1000 / 125 Hz = 8 s

    // Espera PUD e RUD limparem — enquanto estão em 1 a atualização do
    // prescaler/recarga ainda está em andamento.
    loop {
        let status = wdt.stat().read();
        if status.pud().bit_is_clear() && status.rud().bit_is_clear() {
            break;
        }
        // Cede ao executor para manter entradas e temporização funcionando.
        sleep_for(Millis(1)).await;
    }

    wdt.ctl().write(|w| unsafe { w.cmd().bits(CHAVE_RECARGA) });
    wdt.ctl().write(|w| unsafe { w.cmd().bits(CHAVE_INICIA) });
}

/// "Alimenta o cão" — recarrega o contador. Chamar periodicamente.
#[inline(always)]
pub fn alimenta(wdt: &Fwdgt) {
    // SAFETY: 0xAAAA é o comando de recarga do FWDGT.
    wdt.ctl().write(|w| unsafe { w.cmd().bits(CHAVE_RECARGA) });
}
