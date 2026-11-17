//! Botões do painel — três botões num pino só.
//!
//! Os botões Pausa (ON/OFF), Roupas Mais Secas e Enxágue Duplo estão
//! num divisor resistivo ligado a **PA6**. Cada botão apresenta uma
//! tensão diferente, e o firmware descobre qual foi apertado pela
//! faixa do valor do ADC.
//!
//! A leitura é de **10 bits** (0..1023) — por isso `mcu::adc` fixa a
//! resolução em 10 bits.
//!
//! ```text
//!     0 ..  60   nenhum botão apertado (repouso)
//!   764 .. 814   Pausa / ON-OFF        (medido: 789)
//!   826 .. 876   Roupas Mais Secas     (medido: ~851)
//!   906 .. 956   Enxágue Duplo         (medido: ~931)
//! ```
//!
//! Valores fora de todas as faixas são ignorados: é o que acontece no
//! instante em que o contato fecha ou abre.
//!
//! ─── Por que as faixas não são as do C++ ─────────────────────
//!
//! No C++ (STM32) eram `7..60`, `795..845`, `857..907` e `937..987`.
//! Medido na placa com o GD32, o mesmo divisor lê **cerca de 31
//! contagens abaixo**: o repouso dá **0** em vez de ~33, e o botão de
//! ON/OFF dá **789** em vez de ~820.
//!
//! O erro é de deslocamento, não de escala — ganho errado daria a
//! mesma razão nos dois pontos, e 0,962 levaria o repouso a 32, não a
//! zero.
//!
//! A consequência do limite inferior de 7 era grave e silenciosa:
//! o ferrolho que arma a próxima ação só arma quando a leitura cai na
//! faixa de repouso. Lendo 0, ele **nunca armava**, e nenhum botão
//! funcionava — só o modelo 6 revelava isso, porque é o único que
//! depende do botão para retomar depois da tampa.
//!
//! As faixas abaixo são as do C++ **deslocadas** de 31, mantendo a
//! largura de ±25 do original.
//!
//! ─── Por que não alargar em vez de deslocar ──────────────────
//!
//! A primeira tentativa foi alargar as faixas para cobrir as duas
//! escalas, o que gerou um defeito pior: apertar *Mais Secas*
//! desligava a máquina de vez em quando.
//!
//! O motivo é que a tensão **sobe gradualmente** quando o contato
//! fecha. Indo de 0 a ~851, ela atravessa a faixa de Pausa no caminho;
//! com a faixa de Pausa alargada até 845, uma amostra caindo nesse
//! instante virava Pausa — e Pausa com a máquina ligada desliga o
//! ciclo.
//!
//! Com as faixas estreitas e centradas, a travessia cai nas **folgas**
//! entre elas, que não acionam nada. Além disso,
//! `tarefas::amostragem` só publica uma leitura depois de ela se
//! repetir: uma amostra isolada no meio da subida é descartada.
//!
//! Os valores de `MAIS_SECAS` e `ENXAGUE_DUPLO` ainda são inferidos do
//! mesmo deslocamento — o build `--features diag-botoes` grava os
//! patamares de cada botão para confirmar.

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
///
/// Começa em **0**, e não em 7 como no C++: no GD32 esta placa lê zero
/// em repouso, e um limite inferior acima disso impede o ferrolho de
/// armar — nenhum botão funciona.
const REPOUSO: (u16, u16) = (0, 60);
/// Centro medido na placa: **789**. No C++ era 795..845.
const FAIXA_PAUSE: (u16, u16) = (764, 814);
/// Centro inferido do mesmo deslocamento: **~851**. No C++: 857..907.
const FAIXA_MAIS_SECAS: (u16, u16) = (826, 876);
/// Centro inferido do mesmo deslocamento: **~931**. No C++: 937..987.
const FAIXA_ENXAGUE_DUPLO: (u16, u16) = (906, 956);

// ═════════════════════════════════════════════════════════════════════════════
// NORMALIZAÇÃO PELA REFERÊNCIA INTERNA
// ═════════════════════════════════════════════════════════════════════════════

/// Leitura de `VREFINT` na placa onde as faixas acima foram medidas.
///
/// **Zero significa "ainda não medido"**, e nesse caso a normalização
/// fica inerte: `normaliza` devolve a leitura como veio. Para ligar,
/// rode `python -I ferramentas/le_adc.py` na placa de referência e
/// ponha aqui o número que ele mostra como `VREFINT`.
///
/// Sem esse valor não há como normalizar: a correção é uma razão entre
/// o `VREFINT` de agora e o da calibração, e sem o segundo termo não há
/// conta.
pub const VREFINT_REFERENCIA: u16 = 0;

/// Quanto a normalização pode mexer na leitura, em milésimos.
///
/// ─── Por que existe um limite ────────────────────────────────
///
/// A normalização é **certa num caso e errada no outro**:
///
/// * divisor **não** ratiométrico (alimentado por outra referência):
///   a leitura varia com a fonte, e normalizar corrige;
/// * divisor **ratiométrico** (alimentado pela mesma fonte que é a
///   referência do ADC): a leitura já está certa, e normalizar
///   **introduz** um erro do tamanho da variação da fonte.
///
/// As evidências desta placa apontam para o segundo caso — o desvio
/// medido foi deslocamento, não escala (ver o README). Como a dúvida
/// só se encerra medindo duas placas, o limite garante que, estando
/// errado o palpite, o estrago seja menor que a margem da faixa:
///
/// | Limite | Pior erro introduzido | Margem da faixa |
/// |--------|----------------------|-----------------|
/// | 1,0% | 9 contagens | 25 |
/// | **1,5%** | **14 contagens** | 25 |
/// | 3,0% | 28 contagens | 25 — **estouraria** |
///
/// Com 1,5% sobra quase metade da margem para ruído, temperatura e
/// tolerância dos resistores. Se a bancada mostrar que o divisor NÃO é
/// ratiométrico, este limite pode subir.
pub const CORRECAO_MAXIMA_MILESIMOS: u32 = 15;

/// Corrige a leitura pela referência interna de tensão.
///
/// `vrefint` é a leitura do canal 14 — uma tensão fixa do chip medida
/// contra a fonte, então ela sobe quando a fonte cai. A correção é a
/// razão entre o `VREFINT` da calibração e o de agora.
///
/// Devolve a leitura **intocada** quando não há o que fazer: sem valor
/// de calibração, sem leitura de `VREFINT`, ou quando a correção
/// passaria do limite — nesse último caso aplica o limite.
pub fn normaliza(ad: u16, vrefint: u16) -> u16 {
    if VREFINT_REFERENCIA == 0 || vrefint == 0 {
        return ad;
    }

    let ad32 = ad as u32;
    let corrigido = ad32 * VREFINT_REFERENCIA as u32 / vrefint as u32;

    // Trava a correção no limite, nos dois sentidos.
    let limite = ad32 * CORRECAO_MAXIMA_MILESIMOS / 1_000;
    let piso = ad32.saturating_sub(limite);
    let teto = ad32 + limite;

    corrigido.clamp(piso, teto).min(1023) as u16
}

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
