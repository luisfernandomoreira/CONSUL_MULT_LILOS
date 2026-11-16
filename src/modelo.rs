//! Modelo da lavadora — a chave de 4 posições da placa.
//!
//! A mesma PCI atende várias lavadoras Consul. Quem decide qual é a
//! máquina é a chave SW1..SW4, lida no boot e reconferida a cada etapa
//! do ciclo (igual ao `obterModeloAtual()` do C++).
//!
//! | Modelo | Lavadoras                 |
//! |--------|---------------------------|
//! | 1      | CWH12AB V1 e V2           |
//! | 2      | CWH12AB V3, V4 e V5       |
//! | 3      | CWE10AB / CW11AB / CWG12AB|
//! | 4      | CWS11AB                   |
//! | 5      | CWE08AB / CWE09AB         |
//! | 6      | CWH12BB / CWB09BB         |
//! | 7      | auto-teste de fábrica     |
//! | 8      | combinação inválida       |
//!
//! Tabela de `modeloMaquina.cpp`, bit a bit.

/// Modelo reservado ao auto-teste de linha.
pub const AUTO_TESTE: u8 = 7;

/// Converte os quatro bits da chave no número do modelo.
///
/// `chave` vem de `placa::Entradas::chave_modelo()`, na ordem
/// SW1, SW2, SW3, SW4.
pub fn do_chave(chave: [bool; 4]) -> u8 {
    let combinacao = (chave[0] as u8) | ((chave[1] as u8) << 1) | ((chave[2] as u8) << 2) | ((chave[3] as u8) << 3);

    match combinacao {
        0b1000 => 1, // !sw1 !sw2 !sw3  sw4
        0b0100 => 2, // !sw1 !sw2  sw3 !sw4
        0b1100 => 3, // !sw1 !sw2  sw3  sw4
        0b0010 => 4, // !sw1  sw2 !sw3 !sw4
        0b0001 => 5, //  sw1 !sw2 !sw3 !sw4
        0b1111 => 6, //  sw1  sw2  sw3  sw4
        0b0110 => 7, // !sw1  sw2  sw3 !sw4  → auto-teste
        _ => 8,      // qualquer outra → inválido
    }
}

/// É um modelo que o firmware sabe rodar?
///
/// O C++ aceita 1..=7 e trata o resto como erro de seleção (LEDs
/// piscando, triacs desligados).
pub fn valido(modelo: u8) -> bool {
    (1..=7).contains(&modelo)
}

/// Modelos cuja tampa é lida pelo sensor **mecânico** (PA7) em vez do
/// eletrônico (PA4).
pub fn usa_tampa_mecanica(modelo: u8) -> bool {
    modelo == 5 || modelo == 6
}
