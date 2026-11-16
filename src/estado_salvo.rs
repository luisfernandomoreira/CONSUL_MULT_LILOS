//! =============================================================
//! Estado salvo na flash — retomar o ciclo depois da queda de luz
//!
//! A lavadora precisa voltar de onde parou quando a energia cai no
//! meio do ciclo. O firmware C++ guarda quatro valores na página 15 da
//! flash do STM32; aqui eles vão na ÚLTIMA página de 1 KB do
//! GD32C231 (0x0800_FC00), que o `memory.x` reserva deixando o código
//! em 63 KB.
//!
//! | Endereço      | Guarda                        |
//! |---------------|-------------------------------|
//! | 0x0800_FC00   | etapa do ciclo (0..5)         |
//! | 0x0800_FC20   | roupas mais secas (0/1)       |
//! | 0x0800_FC40   | enxágue duplo (0/1)           |
//! | 0x0800_FC60   | pausa (0/1)                   |
//!
//! O espaçamento de 0x20 é o mesmo do C++ (`ADD_15`, `ADD_15_1`…),
//! para a conferência lado a lado ficar fácil.
//!
//! ─── Por que isto mora numa tarefa ───────────────────────────
//! Apagar e gravar uma página trava o núcleo por alguns
//! milissegundos. O C++ faz isso dentro do laço principal; aqui a
//! máquina de estados só **pede** a gravação (`pede_gravacao`), e a
//! tarefa `tarefas::gravacao` executa entre dois eventos. A máquina
//! nunca para para esperar a flash.
//!
//! ─── Flash virgem ───────────────────────────────────────────
//! Numa placa nova a flash está em 0xFFFF_FFFF. O C++ lê esse valor
//! direto, o que acende as três opções sozinhas; aqui tratamos
//! "apagado" como "não há estado salvo" e começamos limpo.
//! =============================================================

// A tabela de enderecos abaixo escreve `BASE + 0x00` de proposito: a
// lista fica alinhada com a do firmware C++ e e mais facil de conferir.
#![allow(clippy::identity_op)]

use core::cell::Cell;
use critical_section::Mutex;
use lilos::exec::Notify;

use crate::mcu::flash;
use crate::mcu::pac::Fmc;

/// Página reservada: a última das 64 páginas de 1 KB.
const PAGINA: u8 = 63;
/// Endereço base da página reservada.
const BASE: u32 = flash::FLASH_INICIO + (PAGINA as u32) * flash::TAMANHO_PAGINA;

const END_ETAPA: u32 = BASE + 0x00;
const END_MAIS_SECAS: u32 = BASE + 0x20;
const END_ENXAGUE_DUPLO: u32 = BASE + 0x40;
const END_PAUSA: u32 = BASE + 0x60;

/// Valor de uma palavra apagada.
const APAGADO: u32 = 0xFFFF_FFFF;

/// O que é guardado entre uma queda de luz e a próxima partida.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct EstadoSalvo {
    /// Etapa do ciclo: 0 encher, 1 agitar, 2 molho, 3 enxaguar,
    /// 4 centrifugar, 5 desligado.
    pub etapa: u8,
    pub mais_secas: bool,
    pub enxague_duplo: bool,
    pub pausa: bool,
}

// ═════════════════════════════════════════════════════════════════════════════
// LEITURA
// ═════════════════════════════════════════════════════════════════════════════

/// Lê o estado guardado. `None` se a página estiver virgem ou com
/// valor fora do esperado.
pub fn carrega() -> Option<EstadoSalvo> {
    let etapa = flash::le(END_ETAPA);
    let mais_secas = flash::le(END_MAIS_SECAS);
    let enxague_duplo = flash::le(END_ENXAGUE_DUPLO);
    let pausa = flash::le(END_PAUSA);

    if etapa == APAGADO || mais_secas == APAGADO || enxague_duplo == APAGADO || pausa == APAGADO {
        return None;
    }
    if etapa > 5 {
        return None;
    }

    Some(EstadoSalvo {
        etapa: etapa as u8,
        mais_secas: mais_secas != 0,
        enxague_duplo: enxague_duplo != 0,
        pausa: pausa != 0,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// PEDIDO DE GRAVAÇÃO
// ═════════════════════════════════════════════════════════════════════════════

/// O pedido pendente. Só o mais recente importa: se a máquina mudar de
/// etapa duas vezes antes da tarefa acordar, grava-se só o estado
/// final.
static PEDIDO: Mutex<Cell<Option<EstadoSalvo>>> = Mutex::new(Cell::new(None));
static AVISO: Notify = Notify::new();

/// A última gravação falhou? A máquina usa isto para acender o erro de
/// interface, como o `falhaFlash` do C++.
static FALHOU: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

/// Pede para a tarefa de gravação salvar este estado.
pub fn pede_gravacao(estado: EstadoSalvo) {
    critical_section::with(|cs| PEDIDO.borrow(cs).set(Some(estado)));
    AVISO.notify();
}

/// Espera o próximo pedido de gravação.
pub async fn proximo_pedido() -> EstadoSalvo {
    AVISO
        .until_racy(|| critical_section::with(|cs| PEDIDO.borrow(cs).take()))
        .await
}

/// Recoloca um pedido na fila — usado pela tarefa quando a gravação
/// falha e vale tentar de novo.
pub fn repete_gravacao(estado: EstadoSalvo) {
    pede_gravacao(estado);
}

pub fn falhou() -> bool {
    critical_section::with(|cs| FALHOU.borrow(cs).get())
}

fn marca_falha(falha: bool) {
    critical_section::with(|cs| FALHOU.borrow(cs).set(falha));
}

// ═════════════════════════════════════════════════════════════════════════════
// GRAVAÇÃO
// ═════════════════════════════════════════════════════════════════════════════

/// Grava o estado na flash. Devolve `false` se a FMC der erro ou se a
/// conferência de leitura não bater.
///
/// Se o que está gravado já é igual, não apaga nada — poupa a página,
/// que tem vida útil limitada. É a mesma checagem do C++.
pub fn grava(fmc: &Fmc, estado: EstadoSalvo) -> bool {
    let etapa = estado.etapa as u32;
    let mais_secas = estado.mais_secas as u32;
    let enxague_duplo = estado.enxague_duplo as u32;
    let pausa = estado.pausa as u32;

    if flash::le(END_ETAPA) == etapa
        && flash::le(END_MAIS_SECAS) == mais_secas
        && flash::le(END_ENXAGUE_DUPLO) == enxague_duplo
        && flash::le(END_PAUSA) == pausa
    {
        marca_falha(false);
        return true;
    }

    if !flash::apaga_pagina(fmc, PAGINA) {
        marca_falha(true);
        return false;
    }

    // A FMC do GD32 grava de 8 em 8 bytes; a palavra alta fica em zero.
    let mut ok = true;
    ok &= flash::grava_double_word(fmc, END_ETAPA, etapa, 0);
    ok &= flash::grava_double_word(fmc, END_MAIS_SECAS, mais_secas, 0);
    ok &= flash::grava_double_word(fmc, END_ENXAGUE_DUPLO, enxague_duplo, 0);
    ok &= flash::grava_double_word(fmc, END_PAUSA, pausa, 0);

    // Confere relendo, igual ao C++.
    ok &= flash::le(END_ETAPA) == etapa
        && flash::le(END_MAIS_SECAS) == mais_secas
        && flash::le(END_ENXAGUE_DUPLO) == enxague_duplo
        && flash::le(END_PAUSA) == pausa;

    marca_falha(!ok);
    ok
}
