//! Protecoes do ciclo e o codigo que cada uma pisca no painel.
//!
//! Esta placa nao tem display, entao a protecao que disparou se anuncia
//! pelo **numero de piscadas** dos tres LEDs juntos. O tecnico conta as
//! piscadas em campo, sem gravador e sem nada ligado.
//!
//! Os numeros sao os mesmos das outras placas da linha, para nao haver
//! uma tabela por produto.
//!
//! ```text
//! Drenar (4 piscadas):
//!   ▀▁▀▁▀▁▀▁________________▀▁▀▁▀▁▀▁________________ ...
//!   └── 4 × 500 ms ──┘└ 1,5 s ┘
//! ```
//!
//! O modulo e de proposito sem nenhuma dependencia: assim os testes de
//! `ferramentas/testa_logica.rs` conferem no PC que dois erros nunca
//! piscam o mesmo numero.

/// Protecoes que travam a maquina e piscam um codigo no painel.
///
/// Os numeros sao os mesmos das outras placas da linha, para o tecnico
/// nao precisar decorar uma tabela por produto.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Erro {
    /// Nao atingiu o nivel dentro do tempo.
    Encher,
    /// Nao esvaziou dentro do tempo.
    Drenar,
    /// O nivel caiu sozinho numa etapa que retem agua.
    Nivel,
}

impl Erro {
    /// Quantas piscadas o painel da para anunciar este erro.
    ///
    /// E o codigo que o tecnico conta em campo, sem gravador e sem
    /// display:
    ///
    /// | Piscadas | Erro | O que suspeitar |
    /// |---|---|---|
    /// | 2 | [`Erro::Nivel`] | nivel caiu sozinho — vazamento, mangueira solta, pressostato |
    /// | 3 | [`Erro::Encher`] | nao encheu — torneira fechada, valvula, pressostato |
    /// | 4 | [`Erro::Drenar`] | nao esvaziou — bomba, mangueira dobrada, filtro entupido |
    ///
    /// A contagem **comeca em 2** de proposito: uma piscada so se
    /// confundiria com o pisca de tampa aberta, que e o LED de ON/OFF
    /// sozinho.
    pub const fn piscadas(self) -> u64 {
        match self {
            Erro::Nivel => 2,
            Erro::Encher => 3,
            Erro::Drenar => 4,
        }
    }

    /// Texto curto do erro — usado nos testes e no roteiro de bancada.
    #[allow(dead_code)]
    pub const fn nome(self) -> &'static str {
        match self {
            Erro::Nivel => "queda de nivel",
            Erro::Encher => "tempo de encher",
            Erro::Drenar => "tempo de drenar",
        }
    }
}
