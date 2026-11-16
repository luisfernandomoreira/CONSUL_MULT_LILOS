//! =============================================================
//! Programas de lavagem — descritos como TABELAS, não como código
//!
//! Cada programa é uma lista de etapas. Mudar um ciclo de lavagem
//! é mudar uma linha de tabela: não se mexe na máquina de estados,
//! não se mexe nos drivers, não há `async` nenhum aqui.
//!
//! Tempos conferidos com o firmware Embassy original (`programas.rs`).
//! =============================================================

/// Os 5 programas do painel, na ordem em que o botão seletor avança.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Programa {
    Pesada = 0,
    Normal = 1,
    Rapido = 2,
    Enxaguar = 3,
    Centrifugar = 4,
}

impl Programa {
    pub const QUANTIDADE: u8 = 5;

    pub fn indice(self) -> u8 {
        self as u8
    }

    pub fn de_indice(i: u8) -> Self {
        match i % Self::QUANTIDADE {
            0 => Programa::Pesada,
            1 => Programa::Normal,
            2 => Programa::Rapido,
            3 => Programa::Enxaguar,
            _ => Programa::Centrifugar,
        }
    }

    /// Próximo programa na roda (volta a Pesada depois de Centrifugar).
    pub fn proximo(self) -> Self {
        Self::de_indice(self.indice() + 1)
    }

    /// O que a máquina faz quando a tabela deste programa acaba.
    ///
    /// É assim que o ciclo completo acontece: cada programa encadeia no
    /// seguinte e a centrifugação encerra tudo. Mesmo comportamento do
    /// firmware original, onde cada programa sinalizava `SIG_CANCELAR`
    /// ao terminar e o `main` avançava para o próximo.
    ///
    ///   Pesada → Normal → Rapido → Enxaguar → Centrifugar → desliga
    ///
    /// Para um programa passar a terminar sozinho, é só devolver
    /// `FimDeCiclo::Desliga` para ele aqui.
    pub fn fim_de_ciclo(self) -> FimDeCiclo {
        match self {
            Programa::Centrifugar => FimDeCiclo::Desliga,
            _ => FimDeCiclo::Encadeia,
        }
    }

    /// Lista de etapas deste programa.
    pub fn etapas(self) -> &'static [Etapa] {
        match self {
            Programa::Pesada => PESADA,
            Programa::Normal => NORMAL,
            Programa::Rapido => RAPIDO,
            Programa::Enxaguar => ENXAGUAR,
            Programa::Centrifugar => CENTRIFUGAR,
        }
    }
}

/// O que acontece quando a tabela de um programa termina.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FimDeCiclo {
    /// Avança para o próximo programa e começa a rodar na hora.
    Encadeia,
    /// Fim do ciclo completo: desliga a máquina, volta para lavagem
    /// pesada e zera as opções.
    Desliga,
}

// ═════════════════════════════════════════════════════════════════════════════
// AÇÕES
// ═════════════════════════════════════════════════════════════════════════════

/// O que a máquina faz durante uma etapa.
///
/// Acrescentar uma ação nova = uma variante aqui + um braço no
/// `aplica_acao_da_etapa()` da máquina de estados. Só isso.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Acao {
    /// Enchimento da LAVAGEM: só a válvula de sabão, até o nível selecionado.
    EncherSabao,
    /// Enchimento do PRIMEIRO enxágue.
    ///
    /// Abre a válvula de sabão e, se não houver enxágue extra pela frente,
    /// solta o amaciante junto — porque este é o último enxágue do ciclo.
    EncherEnxague,
    /// Enchimento do ÚLTIMO enxágue (só existe com "enxágue extra" ligado).
    /// Aqui o amaciante sempre entra, se a opção estiver marcada.
    EncherAmaciante,
    /// Agitação: motor A e motor C alternados (ver `agitacao.rs`).
    Agitar,
    /// De molho: tudo desligado.
    Molho,
    /// Molho do enxágue, com sequência prolongada se Amaciante estiver ativo.
    MolhoEnxague,
    /// Eletrobomba ligada até os dois sensores lerem vazio.
    Drenar,
    /// Eletrobomba ligada por tempo fixo depois do tanque já estar vazio.
    DrenarExtra,
    /// Pulsos progressivos do motor C para distribuir a roupa (tabela `PULSOS`).
    Pulsos,
    /// Centrifugação: motor C contínuo + eletrobomba.
    Centrifugar,
    /// Só a eletrobomba, para escoar o resto da água.
    BombaPos,
    /// Repouso com todos os atuadores desligados antes de concluir.
    PausaFinal,
}

/// Uma etapa do ciclo.
#[derive(Clone, Copy)]
pub struct Etapa {
    pub acao: Acao,
    /// Duração em segundos. `0` = dura até o sensor mandar parar
    /// (vale para os `Encher*`, para `Drenar` e para `Pulsos`).
    pub segundos: u16,
    /// Se `true`, a etapa só roda quando a opção "enxágue extra" está ligada.
    pub so_enxague_extra: bool,
}

/// Etapa normal.
const fn e(acao: Acao, segundos: u16) -> Etapa {
    Etapa {
        acao,
        segundos,
        so_enxague_extra: false,
    }
}

/// Etapa que só roda com a opção "enxágue extra" ligada.
const fn ex(acao: Acao, segundos: u16) -> Etapa {
    Etapa {
        acao,
        segundos,
        so_enxague_extra: true,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// LIMITES DE SEGURANÇA
// ═════════════════════════════════════════════════════════════════════════════

/// Sem atingir o nível em 40 min → erro de enchimento.
pub const TIMEOUT_ENCHER_S: u16 = 40 * 60;
/// Sem esvaziar em 4 min 20 s → erro de drenagem.
pub const TIMEOUT_DRENAR_S: u16 = 4 * 60 + 20;
/// Nível caiu em 5 leituras seguidas de 1 s → erro de nível (vazamento/sensor).
pub const LIMITE_NIVEL_CAIU: u8 = 5;

// ═════════════════════════════════════════════════════════════════════════════
// TEMPOS COMUNS
// ═════════════════════════════════════════════════════════════════════════════
//
// Os blocos de drenagem e centrifugação se repetem em vários programas.
// Em vez de esconder isso atrás de macros, as tabelas abaixo escrevem o
// bloco inteiro (dá para ler o ciclo de cima a baixo) mas usam ESTES
// nomes — então ajustar um tempo continua sendo UMA edição só.

/// Bomba ligada mais um tempo depois do tanque já estar vazio.
pub const DRENAR_EXTRA_S: u16 = 85;
/// Bomba sozinha depois de uma centrifugação curta.
pub const BOMBA_POS_S: u16 = 12;
/// Bomba sozinha depois da centrifugação final.
pub const BOMBA_POS_FINAL_S: u16 = 20;
/// Centrifugação curta entre enxágues — só tira o grosso da água.
pub const CENTRIFUGA_CURTA_S: u16 = 20;
/// Centrifugação da lavagem rápida.
pub const CENTRIFUGA_RAPIDO_S: u16 = 30;
/// Centrifugação final do ciclo.
pub const CENTRIFUGA_FINAL_S: u16 = 6 * 60;
/// Agitação de cada enxágue.
pub const AGITAR_ENXAGUE_S: u16 = 5 * 60;
/// Molho de cada enxágue.
pub const MOLHO_ENXAGUE_S: u16 = 30;

/// Sequência original do amaciante, em segundos e com indicação de agitação.
pub const FASES_AMACIANTE: &[(u16, bool)] = &[
    (300, false),
    (70, true),
    (300, false),
    (70, true),
    (300, false),
    (80, true),
    (240, false),
    (60, true),
    (240, false),
    (60, true),
    (240, false),
];

// ═════════════════════════════════════════════════════════════════════════════
// PULSOS progressivos (antes de centrifugar)
// ═════════════════════════════════════════════════════════════════════════════

/// `(segundos ligado, segundos desligado)` — 7 pulsos crescentes.
pub const PULSOS: &[(u16, u16)] = &[
    (3, 12),
    (3, 12),
    (5, 12),
    (6, 10),
    (10, 9),
    (12, 8),
    (16, 7),
];

// ═════════════════════════════════════════════════════════════════════════════
// TABELAS DOS PROGRAMAS
// ═════════════════════════════════════════════════════════════════════════════
//
// Pesada e Normal terminam após agitação/molho, mantendo a água no tanque.
// Rápido drena e centrifuga. Enxaguar e Centrifugar são programas separados.
//
// As etapas marcadas com `ex(...)` só rodam com a opção "enxágue extra".

/// LAVAGEM PESADA — 12 fases de agitação/molho, mantendo a água no tanque.
const PESADA: &[Etapa] = &[
    // ── Lavagem ──────────────────────────────────────────────────────────────
    e(Acao::EncherSabao, 0),
    e(Acao::Agitar, 5 * 60),
    e(Acao::Molho, 5 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 5 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 5 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 5 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 5 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 60),
];

/// LAVAGEM NORMAL — enche, alterna agitação/molho e termina com a água no tanque.
const NORMAL: &[Etapa] = &[
    e(Acao::EncherSabao, 0),
    e(Acao::Agitar, 5 * 60),
    e(Acao::Molho, 4 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 4 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 4 * 60),
    e(Acao::Agitar, 60),
    e(Acao::Molho, 60),
];

/// LAVAGEM RÁPIDA — lava 8 min e centrifuga 30 s, sem enxágue.
const RAPIDO: &[Etapa] = &[
    e(Acao::EncherSabao, 0),
    e(Acao::Agitar, 8 * 60),
    e(Acao::Molho, 20),
    e(Acao::Drenar, 0),
    e(Acao::DrenarExtra, DRENAR_EXTRA_S),
    e(Acao::Pulsos, 0),
    e(Acao::Centrifugar, CENTRIFUGA_RAPIDO_S),
    e(Acao::BombaPos, BOMBA_POS_S),
];

/// ENXAGUAR — esvazia o que houver, enche de novo, agita e centrifuga.
const ENXAGUAR: &[Etapa] = &[
    // ── Esvazia o que estiver no tanque ──────────────────────────────────────
    e(Acao::Drenar, 0),
    e(Acao::DrenarExtra, DRENAR_EXTRA_S),
    e(Acao::Pulsos, 0),
    e(Acao::Centrifugar, CENTRIFUGA_CURTA_S),
    e(Acao::BombaPos, BOMBA_POS_S),
    // ── 1º enxágue ───────────────────────────────────────────────────────────
    e(Acao::EncherEnxague, 0),
    e(Acao::Agitar, AGITAR_ENXAGUE_S),
    e(Acao::MolhoEnxague, MOLHO_ENXAGUE_S),
    // ── 2º enxágue — só com "enxágue extra" ──────────────────────────────────
    ex(Acao::Drenar, 0),
    ex(Acao::DrenarExtra, DRENAR_EXTRA_S),
    ex(Acao::Pulsos, 0),
    ex(Acao::Centrifugar, CENTRIFUGA_CURTA_S),
    ex(Acao::BombaPos, BOMBA_POS_S),
    ex(Acao::EncherAmaciante, 0),
    ex(Acao::Agitar, AGITAR_ENXAGUE_S),
    ex(Acao::MolhoEnxague, MOLHO_ENXAGUE_S),
];

/// CENTRIFUGAR — drena, distribui a roupa e centrifuga 6 min.
const CENTRIFUGAR: &[Etapa] = &[
    e(Acao::Drenar, 0),
    e(Acao::DrenarExtra, DRENAR_EXTRA_S),
    e(Acao::Pulsos, 0),
    e(Acao::Centrifugar, CENTRIFUGA_FINAL_S),
    e(Acao::BombaPos, BOMBA_POS_FINAL_S),
    e(Acao::PausaFinal, 5),
];
