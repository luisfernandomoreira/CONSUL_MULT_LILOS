//! =============================================================
//! Tabelas de ciclo — os tempos de cada lavadora, por modelo
//!
//! Tudo aqui é dado, não lógica: são as tabelas do firmware C++ de
//! fábrica, copiadas valor por valor. Mexer num número aqui muda o
//! ciclo da máquina, então confira contra o C++ antes.
//!
//! Índices: `[modelo][tipo_lavagem]`, com modelo 1..6 e tipo 0..15.
//! A linha 0 existe só para o modelo poder ser usado direto como
//! índice (nenhuma máquina é modelo 0).
//!
//! Origem de cada tabela:
//!   TEMPO_AGITAR       → `agitacaoControl.cpp`
//!   CONFIG_MOLHO       → `obterConfiguracoesLavagem()` em `main.cpp`
//!   CICLO_TURBO        → `turboTable` em `Molho()`
//!   REUTILIZA_AGUA     → `eco` em `Molho()`
//! =============================================================

/// Quantos programas o encoder oferece.
pub const NUM_TIPOS: usize = 16;

/// Modelos com tabela própria (1..6).
pub const NUM_MODELOS: usize = 7;

// ═════════════════════════════════════════════════════════════════════════════
// TEMPO DE AGITAÇÃO DA ETAPA "AGITAR" (segundos)
// ═════════════════════════════════════════════════════════════════════════════

/// Tempo total da etapa AGITAR, em segundos, por modelo e tipo.
///
/// Zero significa "programa não existe neste modelo".
const TEMPO_AGITAR: [[u16; NUM_TIPOS]; NUM_MODELOS] = [
    // modelo 0 — não existe
    [0; NUM_TIPOS],
    // 1 — CWH12AB V1 e V2
    [
        340, 400, 190, 150, 150, 150, 170, 170, 170, 150, 170, 125, 180, 250, 0, 0,
    ],
    // 2 — CWH12AB V3, V4 e V5
    [
        340, 400, 190, 150, 150, 150, 170, 170, 170, 150, 170, 110, 180, 250, 0, 0,
    ],
    // 3 — CWE10AB / CW11AB / CWG12AB
    [
        150, 150, 480, 150, 120, 150, 240, 0, 0, 150, 150, 150, 150, 150, 170, 150,
    ],
    // 4 — CWS11AB
    [
        150, 150, 150, 150, 150, 150, 150, 150, 150, 480, 150, 130, 250, 150, 0, 0,
    ],
    // 5 — CWE08AB / CWE09AB
    [
        0, 150, 150, 150, 150, 150, 150, 150, 150, 150, 480, 150, 140, 111, 250, 0,
    ],
    // 6 — CWH12BB / CWB09BB
    [
        320, 400, 190, 150, 150, 150, 170, 170, 170, 150, 170, 120, 160, 250, 0, 0,
    ],
];

/// Tempo que a etapa AGITAR deve durar.
///
/// Fora dos modelos 1..6 devolve 120 s — o mesmo fallback do
/// `determinarTempoDeAgitar()` do C++.
pub fn tempo_agitar(modelo: u8, tipo: u8) -> u16 {
    let m = modelo as usize;
    let t = tipo as usize;
    if m >= NUM_MODELOS || m == 0 || t >= NUM_TIPOS {
        return 120;
    }
    TEMPO_AGITAR[m][t]
}

// ═════════════════════════════════════════════════════════════════════════════
// ETAPA "MOLHO" — ciclos de molho alternados com agitação
// ═════════════════════════════════════════════════════════════════════════════

/// Um programa de molho: quantos ciclos, quanto tempo parado e quanto
/// tempo agitando em cada ciclo.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ConfigMolho {
    /// Quantos ciclos molho→agita antes de ir para o enxágue.
    pub ciclos: u16,
    /// Segundos de molho (motores parados) em cada ciclo.
    pub tempo_molho: u16,
    /// Segundos de agitação em cada ciclo.
    pub tempo_agitar: u16,
}

const fn cfg(ciclos: u16, tempo_molho: u16, tempo_agitar: u16) -> ConfigMolho {
    ConfigMolho {
        ciclos,
        tempo_molho,
        tempo_agitar,
    }
}

const SEM_MOLHO: ConfigMolho = cfg(0, 0, 0);

const CONFIG_MOLHO: [[ConfigMolho; NUM_TIPOS]; NUM_MODELOS] = [
    // modelo 0 — não existe
    [SEM_MOLHO; NUM_TIPOS],
    // 1 — CWH12AB V1 e V2
    [
        cfg(36, 92, 33),
        cfg(22, 79, 43),
        cfg(6, 202, 159),
        cfg(16, 311, 56),
        cfg(8, 275, 93),
        cfg(3, 180, 170),
        cfg(32, 66, 46),
        cfg(17, 76, 65),
        cfg(6, 77, 147),
        cfg(17, 73, 58),
        cfg(6, 102, 145),
        cfg(3, 66, 120),
        cfg(25, 290, 83),
        cfg(38, 54, 58),
        SEM_MOLHO,
        SEM_MOLHO,
    ],
    // 2 — CWH12AB V3, V4 e V5 (igual ao modelo 1)
    [
        cfg(36, 92, 33),
        cfg(22, 79, 43),
        cfg(6, 202, 159),
        cfg(16, 311, 56),
        cfg(8, 275, 93),
        cfg(3, 180, 170),
        cfg(32, 66, 46),
        cfg(17, 76, 65),
        cfg(6, 77, 147),
        cfg(17, 73, 58),
        cfg(6, 102, 145),
        cfg(3, 66, 120),
        cfg(25, 290, 83),
        cfg(38, 54, 58),
        SEM_MOLHO,
        SEM_MOLHO,
    ],
    // 3 — CWE10AB / CW11AB / CWG12AB
    [
        cfg(6, 253, 132),
        cfg(4, 180, 190),
        cfg(11, 256, 119),
        cfg(9, 292, 75),
        cfg(3, 150, 120),
        cfg(20, 306, 48),
        cfg(20, 306, 51),
        SEM_MOLHO,
        SEM_MOLHO,
        cfg(20, 289, 48),
        cfg(9, 269, 75),
        cfg(4, 210, 150),
        cfg(16, 311, 55),
        cfg(8, 275, 93),
        cfg(4, 180, 170),
        cfg(11, 289, 88),
    ],
    // 4 — CWS11AB
    [
        cfg(20, 306, 49),
        cfg(9, 293, 75),
        cfg(3, 210, 150),
        cfg(16, 311, 56),
        cfg(8, 275, 93),
        cfg(3, 180, 170),
        cfg(11, 289, 88),
        cfg(6, 253, 132),
        cfg(3, 120, 127),
        cfg(11, 256, 119),
        cfg(9, 295, 82),
        cfg(3, 150, 120),
        cfg(20, 307, 48),
        cfg(20, 307, 48),
        SEM_MOLHO,
        SEM_MOLHO,
    ],
    // 5 — CWE08AB / CWE09AB
    [
        SEM_MOLHO,
        cfg(18, 300, 51),
        cfg(9, 300, 78),
        cfg(4, 210, 150),
        cfg(16, 310, 53),
        cfg(8, 264, 95),
        cfg(4, 180, 170),
        cfg(11, 280, 89),
        cfg(6, 247, 133),
        cfg(4, 180, 180),
        cfg(11, 256, 119),
        cfg(9, 290, 81),
        cfg(3, 150, 120),
        cfg(17, 290, 50),
        cfg(18, 295, 50),
        cfg(1, 1, 1),
    ],
    // 6 — CWH12BB / CWB09BB
    [
        cfg(21, 315, 50),
        cfg(21, 82, 44),
        cfg(5, 240, 190),
        cfg(32, 69, 37),
        cfg(17, 75, 57),
        cfg(6, 77, 130),
        cfg(32, 64, 46),
        cfg(17, 75, 65),
        cfg(6, 77, 147),
        cfg(17, 77, 69),
        cfg(6, 102, 147),
        cfg(4, 45, 90),
        cfg(26, 283, 83),
        cfg(38, 51, 58),
        SEM_MOLHO,
        SEM_MOLHO,
    ],
];

/// Configuração de molho deste modelo/tipo.
///
/// Devolve `None` para modelo fora de 1..6 — no C++ isso devolve um
/// array vazio e a etapa MOLHO não faz nada, que é o mesmo efeito.
pub fn config_molho(modelo: u8, tipo: u8) -> Option<ConfigMolho> {
    let m = modelo as usize;
    if m == 0 || m >= NUM_MODELOS {
        return None;
    }
    // Índice fora da faixa cai no último programa, como no C++.
    let t = if (tipo as usize) < NUM_TIPOS {
        tipo as usize
    } else {
        NUM_TIPOS - 1
    };
    Some(CONFIG_MOLHO[m][t])
}

// ═════════════════════════════════════════════════════════════════════════════
// CICLO DE MOLHO QUE SAI EM TURBO
// ═════════════════════════════════════════════════════════════════════════════

/// Em qual ciclo de molho a agitação sai em turbo. `-1` = nunca.
///
/// É a última agitação do programa, mais forte, para soltar a roupa
/// antes do enxágue.
const CICLO_TURBO: [[i8; NUM_TIPOS]; NUM_MODELOS] = [
    [-1; NUM_TIPOS],
    [35, 21, 5, 31, 16, 5, 31, 16, 5, 16, 5, 2, 24, 37, -1, -1],
    [35, 21, 5, 31, 16, 5, 31, 16, 5, 16, 5, 2, 24, 37, -1, -1],
    [-1; NUM_TIPOS],
    [19, 8, 2, 15, 7, 2, 10, 5, 2, 10, 8, 2, 19, 19, -1, -1],
    [-1, 17, 8, 3, 15, 7, 3, 10, 5, 3, 10, 8, 2, 16, 17, -1],
    [-1; NUM_TIPOS],
];

/// Este ciclo de molho deve agitar em turbo?
pub fn ciclo_e_turbo(modelo: u8, tipo: u8, ciclo_atual: u16) -> bool {
    let m = modelo as usize;
    if m == 0 || m >= NUM_MODELOS {
        return false;
    }
    let t = if (tipo as usize) < NUM_TIPOS {
        tipo as usize
    } else {
        NUM_TIPOS - 1
    };
    let alvo = CICLO_TURBO[m][t];
    alvo >= 0 && ciclo_atual == alvo as u16
}

// ═════════════════════════════════════════════════════════════════════════════
// LAVAGEM ECONÔMICA (REUTILIZAÇÃO DE ÁGUA)
// ═════════════════════════════════════════════════════════════════════════════

/// Programas que reaproveitam a água: o ciclo pausa na entrada do
/// enxágue até a tampa ser aberta, para o usuário tirar a água.
const REUTILIZA_AGUA: [[bool; NUM_TIPOS]; NUM_MODELOS] = [
    [false; NUM_TIPOS],
    // 1 — tipo 10
    [
        false, false, false, false, false, false, false, false, false, false, true, false, false,
        false, false, false,
    ],
    // 2 — tipo 10
    [
        false, false, false, false, false, false, false, false, false, false, true, false, false,
        false, false, false,
    ],
    // 3 — tipo 3
    [
        false, false, false, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    // 4 — tipo 10
    [
        false, false, false, false, false, false, false, false, false, false, true, false, false,
        false, false, false,
    ],
    // 5 — tipo 11
    [
        false, false, false, false, false, false, false, false, false, false, false, true, false,
        false, false, false,
    ],
    // 6 — tipo 10
    [
        false, false, false, false, false, false, false, false, false, false, true, false, false,
        false, false, false,
    ],
];

/// Este programa é de lavagem econômica?
pub fn reutiliza_agua(modelo: u8, tipo: u8) -> bool {
    let m = modelo as usize;
    if m == 0 || m >= NUM_MODELOS {
        return false;
    }
    let t = if (tipo as usize) < NUM_TIPOS {
        tipo as usize
    } else {
        NUM_TIPOS - 1
    };
    REUTILIZA_AGUA[m][t]
}
