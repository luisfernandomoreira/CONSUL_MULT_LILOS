//! =============================================================
//! Persistência do ciclo na flash — retomada após queda de energia
//!
//! Guarda `(programa, etapa, opções)` na última página de 1 KB da flash.
//!
//! ─── Por que gravar em sequência ─────────────────────────────
//! A página cabe 128 registros de 8 bytes. Em vez de apagar a página
//! a cada gravação, escrevemos um registro novo no primeiro espaço
//! livre e só apagamos quando a página enche. Resultado: 1 apagamento
//! a cada 128 gravações — a flash dura ~128× mais.
//!
//! ─── Formato do registro (8 bytes = 1 double word) ───────────
//!   palavra_baixa = MAGICO | programa<<8 | etapa<<16 | opcoes<<24
//!   palavra_alta  = !palavra_baixa        (verificação)
//!
//! Página apagada vale 0xFFFF_FFFF nas duas palavras, e
//! !0xFFFF_FFFF = 0x0000_0000 ≠ 0xFFFF_FFFF — ou seja, um espaço
//! em branco nunca passa por registro válido.
//! =============================================================

use core::cell::Cell;
use critical_section::Mutex;
use lilos::exec::Notify;

use crate::mcu::flash;
use crate::mcu::pac::Fmc;
use crate::opcoes::Opcoes;
use crate::programa::Programa;

/// Última página da flash de 64 KB (página 63).
const PAGINA: u8 = 63;
const ENDERECO_BASE: u32 = flash::FLASH_INICIO + PAGINA as u32 * flash::TAMANHO_PAGINA;
const TAMANHO_REGISTRO: u32 = 8;
const REGISTROS_POR_PAGINA: u32 = flash::TAMANHO_PAGINA / TAMANHO_REGISTRO; // 128

/// Marca que identifica um registro nosso.
const MAGICO: u32 = 0xA5;

/// O que é guardado entre uma energização e outra.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EstadoSalvo {
    pub programa: Programa,
    pub etapa: u8,
    pub opcoes: Opcoes,
    pub ligado: bool,
}

impl EstadoSalvo {
    fn em_palavra(&self) -> u32 {
        MAGICO
            | (self.programa.indice() as u32) << 8
            | (self.etapa as u32) << 16
            | ((self.opcoes.em_byte() | (u8::from(self.ligado) << 4)) as u32) << 24
    }

    fn de_palavra(p: u32) -> Self {
        Self {
            programa: Programa::de_indice(((p >> 8) & 0xFF) as u8),
            etapa: ((p >> 16) & 0xFF) as u8,
            opcoes: Opcoes::de_byte(((p >> 24) & 0xFF) as u8),
            // O bit era zero nos registros anteriores, que ficam desligados.
            ligado: p & (1 << 28) != 0,
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// LEITURA (no boot)
// ═════════════════════════════════════════════════════════════════════════════

/// Procura o último registro válido da página.
///
/// Devolve `None` na primeira energização (página em branco) ou se a
/// página estiver corrompida.
pub fn carrega() -> Option<EstadoSalvo> {
    let mut encontrado = None;

    for i in 0..REGISTROS_POR_PAGINA {
        let endereco = ENDERECO_BASE + i * TAMANHO_REGISTRO;
        let baixa = flash::le(endereco);
        let alta = flash::le(endereco + 4);

        if baixa == 0xFFFF_FFFF && alta == 0xFFFF_FFFF {
            break; // espaço livre: daqui para frente não há mais nada
        }
        if alta == !baixa && (baixa & 0xFF) == MAGICO {
            let estado = EstadoSalvo::de_palavra(baixa);
            let indice_programa = ((baixa >> 8) & 0xFF) as u8;
            if indice_programa < Programa::QUANTIDADE
                && (estado.etapa as usize) < estado.programa.etapas().len()
            {
                encontrado = Some(estado);
            }
        }
    }

    encontrado
}

/// Índice do primeiro registro livre, ou `None` se a página está cheia.
fn primeiro_livre() -> Option<u32> {
    (0..REGISTROS_POR_PAGINA).find(|i| {
        let endereco = ENDERECO_BASE + i * TAMANHO_REGISTRO;
        flash::le(endereco) == 0xFFFF_FFFF && flash::le(endereco + 4) == 0xFFFF_FFFF
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// GRAVAÇÃO (fora da máquina de estados)
// ═════════════════════════════════════════════════════════════════════════════

/// Gravar na flash trava o núcleo por alguns milissegundos. A máquina de
/// estados só deixa o pedido aqui; quem grava é a `tarefas::gravacao`.
static PEDIDO: Mutex<Cell<Option<EstadoSalvo>>> = Mutex::new(Cell::new(None));
static AVISO: Notify = Notify::new();

/// Pede que o estado seja gravado. Retorna na hora — não grava nada aqui.
///
/// Se já houver um pedido pendente, ele é substituído: vale sempre o
/// estado mais recente.
pub fn pede_gravacao(estado: EstadoSalvo) {
    critical_section::with(|cs| PEDIDO.borrow(cs).set(Some(estado)));
    AVISO.notify();
}

/// Reagenda uma falha sem substituir um pedido mais recente da máquina.
pub fn repete_gravacao(estado: EstadoSalvo) {
    critical_section::with(|cs| {
        let pedido = PEDIDO.borrow(cs);
        if pedido.get().is_none() {
            pedido.set(Some(estado));
        }
    });
    AVISO.notify();
}

/// Espera o próximo pedido de gravação.
pub async fn proximo_pedido() -> EstadoSalvo {
    AVISO
        .until_racy(|| critical_section::with(|cs| PEDIDO.borrow(cs).take()))
        .await
}

/// Grava o estado de fato. Chamada só pela tarefa de gravação.
///
/// Devolve `false` se a FMC reportar erro — nesse caso o firmware segue
/// funcionando normalmente, só perde a retomada.
pub fn grava(fmc: &Fmc, estado: EstadoSalvo) -> bool {
    // Não regrava se o último registro já é exatamente este.
    if carrega() == Some(estado) {
        return true;
    }

    let indice = match primeiro_livre() {
        Some(i) => i,
        None => {
            // Página cheia: apaga e recomeça do zero.
            if !flash::apaga_pagina(fmc, PAGINA) {
                return false;
            }
            0
        }
    };

    let palavra = estado.em_palavra();
    flash::grava_double_word(
        fmc,
        ENDERECO_BASE + indice * TAMANHO_REGISTRO,
        palavra,
        !palavra,
    )
}
