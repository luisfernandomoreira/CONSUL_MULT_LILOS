//! Padrão de agitação do motor (ciclo A → pausa → C → pausa).
//!
//! Chamada uma vez a cada 50 ms (`Evento::Tique`). Devolve quais motores
//! devem estar ligados neste instante — quem liga de fato é a máquina de
//! estados. Assim este módulo não conhece hardware nenhum e é fácil de
//! conferir no papel.
//!
//!  PADRÃO (ciclo de 1,10 s)      TURBO (ciclo de 1,00 s)
//!    motor A : ticks  1..5         motor A : ticks  1..6
//!    pausa   : ticks  6..11        pausa   : ticks  7..10
//!    motor C : ticks 12..16        motor C : ticks 11..16
//!    pausa   : ticks 17..22        pausa   : ticks 17..20
//!
//! Padrão: 250 ms em cada sentido, com pausas de 300 ms.
//! Turbo: 300 ms em cada sentido, com pausas de 200 ms.
//! O tick de reset já conta como o último tick da pausa.

/// Marcos do ciclo, em ticks de 50 ms.
struct Tempos {
    /// Último tick com o motor A ligado (1..=tempo_a).
    tempo_a: u16,
    /// Primeiro tick com o motor C ligado.
    c_inicio: u16,
    /// Último tick com o motor C ligado.
    c_fim: u16,
    /// Tick em que o ciclo reinicia.
    reset: u16,
}

const PADRAO: Tempos = Tempos {
    tempo_a: 5,
    c_inicio: 12,
    c_fim: 16,
    reset: 22,
};

const TURBO: Tempos = Tempos {
    tempo_a: 6,
    c_inicio: 11,
    c_fim: 16,
    reset: 20,
};

/// Pausa de segurança ao trocar turbo ↔ padrão: 40 ticks × 50 ms = 2 s
/// com os dois motores parados, para não inverter o motor em movimento.
const PAUSA_TROCA_TICKS: u8 = 40;

/// Quais motores devem estar ligados neste tick.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Motores {
    pub agitacao: bool,
    pub centrifugacao: bool,
}

#[cfg(test)]
mod tests {
    use super::{Agitacao, Motores};

    fn confere_ciclos(turbo: bool, ligados: usize, pausa: usize) {
        let mut agitacao = Agitacao::nova();
        // Aguarda a primeira energização, incluindo a pausa de troca do turbo.
        let primeiro = (0..100)
            .map(|_| agitacao.tique(turbo))
            .find(|m| m.agitacao)
            .expect("a agitação deve começar");
        assert!(!primeiro.centrifugacao);

        for ciclo in 0..100 {
            for tick in 0..2 * (ligados + pausa) {
                let motores = if ciclo == 0 && tick == 0 {
                    primeiro
                } else {
                    agitacao.tique(turbo)
                };
                let esperado = Motores {
                    agitacao: tick < ligados,
                    centrifugacao: (ligados + pausa..2 * ligados + pausa).contains(&tick),
                };
                assert!(motores == esperado, "ciclo {ciclo}, tick {tick}");
            }
        }
    }

    #[test]
    fn padrao_tem_250ms_por_sentido_e_pausas_de_300ms() {
        confere_ciclos(false, 5, 6);
    }

    #[test]
    fn turbo_tem_300ms_por_sentido_e_pausas_de_200ms() {
        confere_ciclos(true, 6, 4);
    }
}

impl Motores {
    pub const PARADOS: Motores = Motores {
        agitacao: false,
        centrifugacao: false,
    };
}

/// Estado do gerador de agitação.
pub struct Agitacao {
    contador: u16,
    turbo_anterior: bool,
    pausa_troca: u8,
}

impl Agitacao {
    pub const fn nova() -> Self {
        Self {
            contador: 0,
            turbo_anterior: false,
            pausa_troca: 0,
        }
    }

    /// Zera o ciclo — chamar ao sair da agitação (tampa, cancelamento, etc.).
    pub fn reinicia(&mut self) {
        self.contador = 0;
        self.pausa_troca = 0;
    }

    /// Avança um tick de 50 ms e devolve o estado dos motores.
    pub fn tique(&mut self, turbo: bool) -> Motores {
        // Pausa de segurança em andamento após troca de modo.
        if self.pausa_troca > 0 {
            self.pausa_troca -= 1;
            self.contador = 0;
            return Motores::PARADOS;
        }

        // Usuário trocou turbo ↔ padrão: para tudo por 2 s e recomeça.
        if turbo != self.turbo_anterior {
            self.turbo_anterior = turbo;
            self.pausa_troca = PAUSA_TROCA_TICKS;
            self.contador = 0;
            return Motores::PARADOS;
        }

        let t = if turbo { &TURBO } else { &PADRAO };
        let c = self.contador + 1;

        if c >= t.reset {
            self.contador = 0;
            return Motores::PARADOS;
        }

        self.contador = c;

        if c <= t.tempo_a {
            Motores {
                agitacao: true,
                centrifugacao: false,
            }
        } else if c >= t.c_inicio && c <= t.c_fim {
            Motores {
                agitacao: false,
                centrifugacao: true,
            }
        } else {
            Motores::PARADOS
        }
    }
}
