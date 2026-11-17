//! Encoder de programa — 16 posições em 4 trilhas.
//!
//! O seletor de programa da CONSUL MULT é um encoder absoluto de 4
//! trilhas (ENC_1, ENC_3, ENC_4 e ENC_5 — os nomes vêm do C++, que
//! pula o 2). A combinação das trilhas dá a posição 0..15, que é o
//! `tipoLavagem` do firmware.
//!
//! A tabela NÃO é binária direta: é a sequência física das pistas do
//! disco, copiada de `encoder.cpp`. Cada uma das 16 combinações
//! possíveis está mapeada, então nunca cai num valor inválido.

/// Traduz as trilhas na posição do encoder (0..15).
///
/// `trilhas` vem de `placa::Entradas::trilhas_encoder()`, na ordem
/// ENC_1, ENC_3, ENC_4, ENC_5.
pub fn posicao(trilhas: [bool; 4]) -> u8 {
    let padrao = ((trilhas[0] as u8) << 3)
        | ((trilhas[1] as u8) << 2)
        | ((trilhas[2] as u8) << 1)
        | (trilhas[3] as u8);

    match padrao {
        0b1110 => 0,
        0b1100 => 1,
        0b1101 => 2,
        0b0101 => 3,
        0b0100 => 4,
        0b0000 => 5,
        0b0001 => 6,
        0b1001 => 7,
        0b1000 => 8,
        0b1010 => 9,
        0b1011 => 10,
        0b0011 => 11,
        0b0010 => 12,
        0b0110 => 13,
        0b0111 => 14,
        0b1111 => 15,
        // As 16 combinações acima cobrem todos os padrões de 4 bits;
        // este braço existe só para o compilador.
        _ => 0,
    }
}

/// Detecta que o usuário girou o seletor.
///
/// Confirma a posição após duas leituras iguais consecutivas, para
/// rejeitar oscilações de contato. Com amostragem de 81 ms, a mudança
/// é confirmada em aproximadamente 81 a 162 ms.
pub struct DetectorMudanca {
    ultimo: [bool; 4],
    candidato: [bool; 4],
}

impl DetectorMudanca {
    /// Começa já sincronizado com a posição atual, para o boot não
    /// registrar uma troca que não houve.
    pub fn novo(trilhas: [bool; 4]) -> Self {
        Self {
            ultimo: trilhas,
            candidato: trilhas,
        }
    }

    /// Devolve `true` uma vez quando a nova posição se estabiliza.
    pub fn mudou(&mut self, trilhas: [bool; 4]) -> bool {
        if trilhas != self.candidato {
            self.candidato = trilhas;
            return false;
        }
        if trilhas == self.ultimo {
            return false;
        }
        self.ultimo = trilhas;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::DetectorMudanca;

    #[test]
    fn confirma_mudanca_sem_repetir_evento() {
        let mut detector = DetectorMudanca::novo([false; 4]);
        assert!(!detector.mudou([false; 4]));
        assert!(!detector.mudou([true; 4]));
        assert!(detector.mudou([true; 4]));
        assert!(!detector.mudou([true; 4]));
    }

    #[test]
    fn rejeita_ruido_e_reinicia_confirmacao() {
        let inicial = [false; 4];
        let nova = [true; 4];
        let mut detector = DetectorMudanca::novo(inicial);
        assert!(!detector.mudou(nova));
        assert!(!detector.mudou(inicial));
        assert!(!detector.mudou(nova));
        assert!(detector.mudou(nova));
    }
}
