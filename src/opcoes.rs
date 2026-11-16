//! Opções escolhidas pelo usuário no painel.
//!
//! É um struct simples: só a máquina de estados mexe nele, então não
//! precisa de atomics nem de mutex. Quem quiser ler as opções recebe
//! uma `&Opcoes` de parâmetro.

/// Estado dos quatro botões de opção.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Opcoes {
    /// `false` = nível médio | `true` = nível alto.
    pub nivel_alto: bool,
    /// Repete o ciclo de enxágue.
    pub enxague_extra: bool,
    /// Agitação mais rápida.
    pub turbo: bool,
    /// Usa a válvula de amaciante.
    pub amaciante: bool,
}

impl Opcoes {
    pub const fn padrao() -> Self {
        Self {
            nivel_alto: false,
            enxague_extra: false,
            turbo: false,
            amaciante: false,
        }
    }

    /// Compacta as opções em 1 byte — formato usado para gravar na flash.
    pub fn em_byte(&self) -> u8 {
        (self.nivel_alto as u8)
            | (self.enxague_extra as u8) << 1
            | (self.turbo as u8) << 2
            | (self.amaciante as u8) << 3
    }

    /// Lê as opções de volta de 1 byte.
    pub fn de_byte(b: u8) -> Self {
        Self {
            nivel_alto: b & 0b0001 != 0,
            enxague_extra: b & 0b0010 != 0,
            turbo: b & 0b0100 != 0,
            amaciante: b & 0b1000 != 0,
        }
    }
}
