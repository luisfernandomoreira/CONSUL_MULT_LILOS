//! GPIO do GD32C231.
//!
//! Registradores por porta (nomes do PAC, que vêm do SVD oficial):
//!   `ctl`    2 bits/pino: 00 entrada | 01 saída | 10 função alt. | 11 analógico
//!   `omode`  1 bit/pino : 0 push-pull | 1 open-drain
//!   `ospd`   2 bits/pino: velocidade de borda
//!   `pud`    2 bits/pino: 00 nenhum | 01 pull-up | 10 pull-down
//!   `istat`  leitura do pino
//!   `bop`    escreve 1 → pino vai a nível alto   (atômico)
//!   `bc`     escreve 1 → pino vai a nível baixo  (atômico)
//!
//! Os registradores de 2 bits por pino são escritos com `bits()` em vez
//! dos acessores `ctl0()`..`ctl15()` do PAC: o número do pino só é
//! conhecido em tempo de execução, e um `match` de 16 braços por
//! registrador não deixaria nada mais seguro nem mais legível.

use super::{campo, pac};

/// Portas com pinos no encapsulamento LQFP32.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Porta {
    A,
    B,
    C,
}

// No SVD, GPIOA é um periférico à parte (tem valores de reset próprios)
// e GPIOC deriva de GPIOB — então o PAC gera dois tipos de bloco de
// registradores. O layout é idêntico; conferimos isso em tempo de
// compilação para poder tratar as três portas por um tipo só.
const _: () = assert!(
    core::mem::size_of::<pac::gpioa::RegisterBlock>()
        == core::mem::size_of::<pac::gpiob::RegisterBlock>()
);

impl Porta {
    /// Bloco de registradores desta porta.
    #[inline(always)]
    fn bloco(self) -> &'static pac::gpioa::RegisterBlock {
        let ptr: *const pac::gpioa::RegisterBlock = match self {
            Porta::A => pac::Gpioa::ptr(),
            Porta::B => pac::Gpiob::ptr().cast(),
            Porta::C => pac::Gpioc::ptr().cast(),
        };
        // SAFETY: o ponteiro vem do próprio PAC e os dois blocos têm o
        // mesmo layout (conferido no `assert!` acima).
        unsafe { &*ptr }
    }
}

/// Resistor interno de um pino de entrada.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pull {
    Nenhum = 0b00,
    Cima = 0b01,
    Baixo = 0b10,
}

/// Identifica um pino: porta + número (0..15).
///
/// É só um par de valores — pode ser `const`, copiado e passado à vontade.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pino {
    pub porta: Porta,
    pub numero: u8,
}

impl Pino {
    pub const fn novo(porta: Porta, numero: u8) -> Self {
        Self { porta, numero }
    }

    #[inline(always)]
    const fn bit(&self) -> u32 {
        1 << self.numero
    }

    #[inline(always)]
    const fn desloc2(&self) -> u32 {
        (self.numero as u32) * 2
    }
}

// ─── Saída digital ───────────────────────────────────────────────────────────

/// Pino configurado como saída push-pull.
pub struct Saida {
    pino: Pino,
}

impl Saida {
    /// Configura o pino como saída já no `nivel_inicial` pedido.
    ///
    /// O nível é aplicado ANTES de trocar o modo para saída, para o pino
    /// nunca passar por um estado indefinido — importante nos triacs.
    pub fn nova(pino: Pino, nivel_inicial: bool) -> Self {
        let saida = Self { pino };
        saida.escreve(nivel_inicial);

        let g = pino.porta.bloco();
        // SAFETY: valores dentro da largura do campo, conforme o manual.
        unsafe {
            g.omode().modify(|r, w| w.bits(r.bits() & !pino.bit())); // push-pull
            g.ospd()
                .modify(|r, w| w.bits(campo(r.bits(), pino.desloc2(), 2, 0b00))); // borda lenta
            g.ctl()
                .modify(|r, w| w.bits(campo(r.bits(), pino.desloc2(), 2, 0b01)));
            // saída
        }
        saida
    }

    /// Nível alto. Atômico (usa BOP), pode ser chamado de qualquer tarefa.
    #[inline(always)]
    pub fn alta(&self) {
        // SAFETY: BOP é "escreve 1 para agir"; os outros bits ficam em 0.
        unsafe {
            self.pino
                .porta
                .bloco()
                .bop()
                .write(|w| w.bits(self.pino.bit()))
        };
    }

    /// Nível baixo. Atômico (usa BC).
    #[inline(always)]
    pub fn baixa(&self) {
        // SAFETY: idem `alta`.
        unsafe {
            self.pino
                .porta
                .bloco()
                .bc()
                .write(|w| w.bits(self.pino.bit()))
        };
    }

    #[inline(always)]
    pub fn escreve(&self, nivel_alto: bool) {
        if nivel_alto {
            self.alta()
        } else {
            self.baixa()
        }
    }
}

// ─── Entrada digital ─────────────────────────────────────────────────────────

/// Pino configurado como entrada digital.
pub struct Entrada {
    pino: Pino,
}

impl Entrada {
    pub fn nova(pino: Pino, pull: Pull) -> Self {
        let g = pino.porta.bloco();
        // SAFETY: valores dentro da largura do campo, conforme o manual.
        unsafe {
            g.pud()
                .modify(|r, w| w.bits(campo(r.bits(), pino.desloc2(), 2, pull as u32)));
            g.ctl()
                .modify(|r, w| w.bits(campo(r.bits(), pino.desloc2(), 2, 0b00)));
            // entrada
        }
        Self { pino }
    }

    #[inline(always)]
    pub fn esta_alta(&self) -> bool {
        self.pino.porta.bloco().istat().read().bits() & self.pino.bit() != 0
    }

    #[inline(always)]
    pub fn esta_baixa(&self) -> bool {
        !self.esta_alta()
    }
}
