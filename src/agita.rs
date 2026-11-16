//! =============================================================
//! Agitação — o vai-e-vem do motor
//!
//! O agitador não gira sempre para o mesmo lado: alterna entre dois
//! sentidos (A e C) numa cadência que define a "força" da lavagem.
//! Cada padrão é um par de janelas sobre um contador de ~21 ms, que é
//! o período do TIM16 no firmware C++ (24 MHz ÷ 24000 ÷ 21).
//!
//! ```text
//!   contador:  0 ....... tempo_a ..... c_inicio ... c_fim ... reset
//!   motor:     [--- A ---]  (parado)   [---- C ----] (parado)   ↺
//! ```
//!
//! Os quatro padrões e seus tempos vêm de `agita.cpp`:
//!
//! | Padrão          | A  | C início | C fim | reset |
//! |-----------------|----|----------|-------|-------|
//! | Padrão          | 13 | 35       | 48    | 70    |
//! | Delicado        | 12 | 74       | 86    | 147   |
//! | Turbo           | 16 | 27       | 43    | 54    |
//! | Turbo delicado  | 12 | 42       | 54    | 84    |
//!
//! O contador mora num `static` pelo mesmo motivo do `relogio`: quem
//! o move é a tarefa de 21 ms, que precisa continuar contando mesmo
//! com a máquina de estados ocupada. No C++ é o `volatile static
//! cnt16` da classe `Agita`, atualizado pela interrupção do TIM16.
//! =============================================================

use core::cell::Cell;
use critical_section::Mutex;

use crate::placa::Saidas;

/// Período do contador de agitação, em milissegundos.
pub const PERIODO_MS: u64 = 21;

#[derive(Clone, Copy)]
struct Estado {
    contador: u16,
    timer_ligado: bool,
}

static ESTADO: Mutex<Cell<Estado>> = Mutex::new(Cell::new(Estado {
    contador: 0,
    timer_ligado: false,
}));

fn com<R>(f: impl FnOnce(&mut Estado) -> R) -> R {
    critical_section::with(|cs| {
        let celula = ESTADO.borrow(cs);
        let mut e = celula.get();
        let r = f(&mut e);
        celula.set(e);
        r
    })
}

/// Chamada pela tarefa de 21 ms. Só conta com o timer ligado.
pub fn tique() {
    com(|e| {
        if e.timer_ligado {
            e.contador = e.contador.wrapping_add(1);
        }
    });
}

/// Liga ou para o contador. Parar NÃO zera o valor — é o que o
/// `ligaTimer16()` do C++ faz.
pub fn liga_timer(ligado: bool) {
    com(|e| e.timer_ligado = ligado);
}

pub fn zera() {
    com(|e| e.contador = 0);
}

pub fn contador() -> u16 {
    com(|e| e.contador)
}

/// Desliga os dois sentidos do motor.
pub fn desliga_motores(s: &Saidas) {
    s.motor_c(false);
    s.motor_a(false);
}

/// Gira no sentido A (desligando C antes, nunca os dois juntos).
fn motor_sentido_a(s: &Saidas) {
    s.motor_c(false);
    s.motor_a(true);
}

/// Gira no sentido C (desligando A antes).
fn motor_sentido_c(s: &Saidas) {
    s.motor_a(false);
    s.motor_c(true);
}

// ═════════════════════════════════════════════════════════════════════════════
// PADRÕES
// ═════════════════════════════════════════════════════════════════════════════

/// Um padrão de agitação: as quatro marcas do contador.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Padrao {
    tempo_a: u16,
    c_inicio: u16,
    c_fim: u16,
    reset: u16,
}

impl Padrao {
    pub const PADRAO: Padrao = Padrao {
        tempo_a: 13,
        c_inicio: 35,
        c_fim: 48,
        reset: 70,
    };
    pub const DELICADO: Padrao = Padrao {
        tempo_a: 12,
        c_inicio: 74,
        c_fim: 86,
        reset: 147,
    };
    pub const TURBO: Padrao = Padrao {
        tempo_a: 16,
        c_inicio: 27,
        c_fim: 43,
        reset: 54,
    };
    pub const TURBO_DELICADO: Padrao = Padrao {
        tempo_a: 12,
        c_inicio: 42,
        c_fim: 54,
        reset: 84,
    };
}

/// Aplica um padrão de agitação: liga o contador e comanda o motor
/// conforme a janela em que o contador está.
pub fn executa(s: &Saidas, p: Padrao) {
    liga_timer(true);

    let t = contador();

    if t >= p.reset {
        zera();
        desliga_motores(s);
    } else if t <= p.tempo_a {
        motor_sentido_a(s);
    } else if t >= p.c_inicio && t <= p.c_fim {
        motor_sentido_c(s);
    } else {
        desliga_motores(s);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// ESCOLHA DO PADRÃO POR MODELO E TIPO DE LAVAGEM
// ═════════════════════════════════════════════════════════════════════════════

/// Agitação da etapa AGITAR (`executarTipoAgitacao` do C++).
///
/// Nos modelos 1, 2 e 4 os tipos 14 e 15 não agitam: são os programas
/// "só enxaguar" e "só centrifugar", que nem passam por esta etapa.
pub fn etapa_agitar(s: &Saidas, modelo: u8, tipo: u8, modo_turbo: bool) {
    match modelo {
        1 | 2 | 4 => {
            if tipo <= 11 {
                executa(s, Padrao::PADRAO);
            } else if tipo == 12 {
                executa(s, Padrao::TURBO); // panos de limpeza
            } else if tipo == 13 {
                executa(s, Padrao::DELICADO); // edredom
            }
        }
        3 => match tipo {
            6 => executa(s, Padrao::TURBO),    // panos de limpeza
            2 => executa(s, Padrao::DELICADO), // delicado
            _ => executa(s, Padrao::PADRAO),
        },
        5 => {
            if tipo == 10 || tipo == 13 {
                // delicado / cama e banho
                if modo_turbo {
                    executa(s, Padrao::TURBO_DELICADO);
                } else {
                    executa(s, Padrao::DELICADO);
                }
            } else if modo_turbo {
                executa(s, Padrao::TURBO);
            } else {
                executa(s, Padrao::PADRAO);
            }
        }
        6 => match tipo {
            0 | 12 => executa(s, Padrao::TURBO), // roupa branca / pesado
            5 | 6 | 9 | 13 => executa(s, Padrao::DELICADO),
            _ => executa(s, Padrao::PADRAO),
        },
        _ => {}
    }
}

/// Agitação dentro da etapa MOLHO (`chamaAgitacao` do C++).
///
/// `turbo_agora` vem de `tabelas::ciclo_e_turbo`: é o ciclo final,
/// mais forte, de cada programa.
pub fn etapa_molho(s: &Saidas, modelo: u8, tipo: u8, bt_modo_agita: bool, turbo_agora: bool) {
    match modelo {
        3 => {
            if turbo_agora {
                if tipo == 2 {
                    executa(s, Padrao::TURBO_DELICADO);
                } else {
                    executa(s, Padrao::TURBO);
                }
            } else {
                match tipo {
                    2 => executa(s, Padrao::DELICADO),
                    6 => executa(s, Padrao::TURBO),
                    _ => executa(s, Padrao::PADRAO),
                }
            }
        }
        5 => {
            if tipo == 10 || tipo == 13 {
                if bt_modo_agita {
                    executa(s, Padrao::TURBO_DELICADO);
                } else {
                    executa(s, Padrao::DELICADO);
                }
            } else if turbo_agora || bt_modo_agita {
                executa(s, Padrao::TURBO);
            } else {
                executa(s, Padrao::PADRAO);
            }
        }
        6 => match tipo {
            0 | 12 => executa(s, Padrao::TURBO),
            5 | 6 | 9 | 13 => executa(s, Padrao::DELICADO),
            _ => {
                if turbo_agora {
                    executa(s, Padrao::TURBO);
                } else {
                    executa(s, Padrao::PADRAO);
                }
            }
        },
        // 1, 2 e 4
        _ => {
            if turbo_agora {
                if tipo == 13 {
                    executa(s, Padrao::TURBO_DELICADO);
                } else {
                    executa(s, Padrao::TURBO);
                }
            } else {
                match tipo {
                    12 => executa(s, Padrao::TURBO),
                    13 => executa(s, Padrao::DELICADO),
                    _ => executa(s, Padrao::PADRAO),
                }
            }
        }
    }
}

/// Primeira agitação do enxágue — 0 a 200 s (3 min 20 s).
pub fn enxague_inicio(s: &Saidas, modelo: u8, tipo: u8, bt_modo_agita: bool) {
    match modelo {
        3 => {
            if tipo == 2 {
                executa(s, Padrao::DELICADO);
            } else {
                executa(s, Padrao::PADRAO);
            }
        }
        5 => {
            if tipo == 10 || tipo == 13 {
                if bt_modo_agita {
                    executa(s, Padrao::TURBO_DELICADO);
                } else {
                    executa(s, Padrao::DELICADO);
                }
            } else if bt_modo_agita {
                executa(s, Padrao::TURBO);
            } else {
                executa(s, Padrao::PADRAO);
            }
        }
        6 => {
            if matches!(tipo, 5 | 6 | 9 | 13) {
                executa(s, Padrao::DELICADO);
            } else {
                executa(s, Padrao::PADRAO);
            }
        }
        _ => {
            if tipo == 13 {
                executa(s, Padrao::DELICADO);
            } else {
                executa(s, Padrao::PADRAO);
            }
        }
    }
}

/// Segunda agitação do enxágue — 252 a 312 s. Sempre mais forte.
pub fn enxague_final(s: &Saidas, modelo: u8, tipo: u8) {
    match modelo {
        3 => {
            if tipo == 2 {
                executa(s, Padrao::TURBO_DELICADO);
            } else {
                executa(s, Padrao::TURBO);
            }
        }
        5 => {
            if tipo == 10 || tipo == 13 {
                executa(s, Padrao::TURBO_DELICADO);
            } else {
                executa(s, Padrao::TURBO);
            }
        }
        6 => {
            if tipo == 9 || tipo == 13 {
                executa(s, Padrao::TURBO_DELICADO);
            } else {
                executa(s, Padrao::TURBO);
            }
        }
        _ => {
            if tipo == 13 {
                executa(s, Padrao::TURBO_DELICADO);
            } else {
                executa(s, Padrao::TURBO);
            }
        }
    }
}
