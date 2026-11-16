//! =============================================================
//! Leituras — o que a tarefa de amostragem viu
//!
//! Fica entre quem lê o hardware (`tarefas::amostragem`, de 81 ms, e
//! `tarefas::agitacao`, de 21 ms) e quem decide (`maquina`). No C++
//! estes valores são membros da `MaquinaDeLavarFacade` atualizados
//! dentro do laço principal; aqui ficam num `static` porque quem lê e
//! quem escreve são tarefas diferentes.
//!
//! Dois deles merecem atenção:
//!
//! * `nivel_agua` já vem **invertido e filtrado**: o pressostato em
//!   nível baixo significa cheio, e a média de 10 amostras evita que
//!   a oscilação da água faça o ciclo pular de etapa. É o
//!   `FiltroDebouncePressostato` do C++.
//!
//! * `encoder_mudou` é **pegajoso**: fica marcado até a máquina
//!   consumi-lo. Girar o seletor recomeça o ciclo no programa novo, e
//!   essa informação não pode se perder porque a máquina estava
//!   ocupada piscando um LED.
//!
//! =============================================================

use core::cell::Cell;
use critical_section::Mutex;

use crate::modelo;

#[derive(Clone, Copy)]
struct Dados {
    ad: u16,
    nivel_agua: bool,
    encoder_mudou: bool,
    posicao_encoder: u8,
    tampa_eletronica_aberta: bool,
    tampa_mecanica_aberta: bool,
}

static DADOS: Mutex<Cell<Dados>> = Mutex::new(Cell::new(Dados {
    ad: 0,
    nivel_agua: false,
    encoder_mudou: false,
    posicao_encoder: 0,
    tampa_eletronica_aberta: false,
    tampa_mecanica_aberta: false,
}));

fn com<R>(f: impl FnOnce(&mut Dados) -> R) -> R {
    critical_section::with(|cs| {
        let celula = DADOS.borrow(cs);
        let mut d = celula.get();
        let r = f(&mut d);
        celula.set(d);
        r
    })
}

// ─── Escrita (tarefas de amostragem) ─────────────────────────────────────────

pub fn set_ad(valor: u16) {
    com(|d| d.ad = valor);
}

/// `true` = nível de água atingido (pressostato já invertido).
pub fn set_nivel_agua(cheio: bool) {
    com(|d| d.nivel_agua = cheio);
}

/// Marca que o seletor girou e guarda a nova posição.
pub fn marca_encoder(posicao: u8) {
    com(|d| {
        d.encoder_mudou = true;
        d.posicao_encoder = posicao;
    });
}

/// Guarda a posição sem marcar mudança — usado no boot.
pub fn set_posicao_encoder(posicao: u8) {
    com(|d| d.posicao_encoder = posicao);
}

pub fn set_tampas(eletronica_aberta: bool, mecanica_aberta: bool) {
    com(|d| {
        d.tampa_eletronica_aberta = eletronica_aberta;
        d.tampa_mecanica_aberta = mecanica_aberta;
    });
}

// ─── Leitura (máquina de estados) ────────────────────────────────────────────

pub fn ad() -> u16 {
    com(|d| d.ad)
}

pub fn nivel_agua() -> bool {
    com(|d| d.nivel_agua)
}

pub fn posicao_encoder() -> u8 {
    com(|d| d.posicao_encoder)
}

/// Lê e limpa a marca de "seletor girou".
pub fn consome_encoder_mudou() -> bool {
    com(|d| {
        let mudou = d.encoder_mudou;
        d.encoder_mudou = false;
        mudou
    })
}

/// Tampa aberta, pelo sensor que este modelo usa.
///
/// Modelos 5 e 6 têm sensor mecânico (PA7); os outros, o eletrônico
/// (PA4). É o `getLeituraTampa(modelo)` do C++.
pub fn tampa_aberta(modelo_maquina: u8) -> bool {
    com(|d| {
        if modelo::usa_tampa_mecanica(modelo_maquina) {
            d.tampa_mecanica_aberta
        } else {
            d.tampa_eletronica_aberta
        }
    })
}
