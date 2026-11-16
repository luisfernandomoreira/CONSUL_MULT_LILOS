//! Botões do painel — três botões num pino só.
//!
//! Os botões Pausa (ON/OFF), Roupas Mais Secas e Enxágue Duplo estão
//! num divisor resistivo ligado a **PA6**. Cada botão apresenta uma
//! tensão diferente, e o firmware descobre qual foi apertado pela
//! faixa do valor do ADC.
//!
//! As faixas são as mesmas do `gerenciaInterface()` do C++, numa
//! leitura de **10 bits** (0..1023) — por isso `mcu::adc` fixa a
//! resolução em 10 bits.
//!
//! ```text
//!     7 ..  60   nenhum botão apertado (repouso)
//!   795 .. 845   Pausa / ON-OFF
//!   857 .. 907   Roupas Mais Secas
//!   937 .. 987   Enxágue Duplo
//! ```
//!
//! Valores fora de todas as faixas são ignorados: é o que acontece no
//! instante em que o contato fecha ou abre.

/// Qual botão o valor do ADC representa.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Botao {
    /// Pausa o ciclo; com a máquina desligada, liga.
    Pause,
    /// Alterna "roupas mais secas" (centrifuga mais tempo).
    MaisSecas,
    /// Alterna o enxágue duplo.
    EnxagueDuplo,
}

/// Faixa de repouso: nenhum botão apertado.
const REPOUSO: (u16, u16) = (7, 60);
const FAIXA_PAUSE: (u16, u16) = (795, 845);
const FAIXA_MAIS_SECAS: (u16, u16) = (857, 907);
const FAIXA_ENXAGUE_DUPLO: (u16, u16) = (937, 987);

#[inline]
fn dentro(valor: u16, faixa: (u16, u16)) -> bool {
    valor >= faixa.0 && valor <= faixa.1
}

/// O painel está em repouso (nenhum botão apertado)?
pub fn em_repouso(ad: u16) -> bool {
    dentro(ad, REPOUSO)
}

/// Qual botão este valor de ADC identifica, se algum.
pub fn identifica(ad: u16) -> Option<Botao> {
    if dentro(ad, FAIXA_PAUSE) {
        Some(Botao::Pause)
    } else if dentro(ad, FAIXA_MAIS_SECAS) {
        Some(Botao::MaisSecas)
    } else if dentro(ad, FAIXA_ENXAGUE_DUPLO) {
        Some(Botao::EnxagueDuplo)
    } else {
        None
    }
}
