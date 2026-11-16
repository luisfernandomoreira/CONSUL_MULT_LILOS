//! Roda a máquina de estados REAL (`src/maquina.rs`) no PC e confere se ela
//! percorre todas as etapas de cada programa e desliga no fim.
//!
//! O arquivo tem DOIS modos. O `--edition 2021` é obrigatório nos dois:
//! sem ele o `rustc` usa a edição 2015, onde os `use` internos da
//! `maquina.rs` não resolvem.
//!
//! 1. O `main()` — varredura curta, legível, que imprime cada ciclo e o
//!    traço dos códigos de erro. Na raiz do projeto:
//!
//! ```text
//! rustc -O --edition 2021 ferramentas/simular_ciclo.rs -o simular && ./simular
//! ```
//!
//! 2. O `mod testes` (`#[cfg(test)]`, no fim do arquivo) — varredura larga,
//!    sem saída para ler, com os casos que o `main()` não cobre:
//!
//! ```text
//! rustc --test -O --edition 2021 ferramentas/simular_ciclo.rs -o testes && ./testes
//! ```
//!
//! Não é um teste de uma cópia do código: `maquina.rs`, `programa.rs`,
//! `agitacao.rs` e `opcoes.rs` são incluídos do `src/` como módulos.
//! Só o que toca hardware (`placa`), a fila de eventos (`evento`, que
//! depende do lilos) e a gravação na flash (`estado_salvo`) são
//! substituídos por dublês aqui embaixo.
//!
//! Se um dia alguém acrescentar uma variante em `Evento` e tratá-la na
//! máquina, este arquivo para de compilar — que é o aviso desejado.
//!
//! O que o `main()` verifica, para os 5 programas × com e sem enxágue
//! extra × tanque vazio/cheio:
//!
//!   1. a máquina entra em execução depois do liga/desliga;
//!   2. passa por TODAS as etapas aplicáveis da tabela, em ordem,
//!      sem pular nenhuma e sem repetir;
//!   3. chega ao fim da tabela;
//!   4. termina com todos os triacs desligados;
//!   5. no fim de CENTRIFUGAR: volta para lavagem pesada, zera as
//!      opções e fica DESLIGADA — igual ao firmware original;
//!   6. nos outros programas: termina em "concluída", ainda ligada,
//!      esperando o seletor.
//!
//! O que o `mod testes` acrescenta:
//!
//!   * os 5 programas × TODAS as 16 combinações de opções × tanque
//!     vazio/cheio = 160 ciclos, conferindo também as etapas vistas;
//!   * a queda de nível em `MolhoEnxague`, com e sem amaciante e nos dois
//!     níveis — a etapa fica até ~33 min com água retida;
//!   * a tampa pausando e retomando a agitação, sem perder o tempo da
//!     etapa e respeitando os 2 s de religamento;
//!   * o boot restaurando a etapa salva e honrando o bit `ligado`, com a
//!     espera de câmbio antes da centrifugação.
//!
//! Fora de alcance aqui, só na placa: a gravação real na flash e o
//! debounce das entradas.

#![allow(dead_code)]

// ═════════════════════════════════════════════════════════════════════════════
// DUBLÊS — o que a máquina de estados espera do resto do firmware
// ═════════════════════════════════════════════════════════════════════════════

mod evento {
    //! Só os tipos. A fila de verdade depende do lilos e não é usada aqui:
    //! o simulador chama `maquina.trata(...)` diretamente.

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Botao {
        OnOff = 0,
        Seletor = 1,
        Nivel = 2,
        EnxagueExtra = 3,
        Turbo = 4,
        Amaciante = 5,
    }

    impl Botao {
        pub const TODOS: [Botao; 6] = [
            Botao::OnOff,
            Botao::Seletor,
            Botao::Nivel,
            Botao::EnxagueExtra,
            Botao::Turbo,
            Botao::Amaciante,
        ];
        pub const fn indice(self) -> usize {
            self as usize
        }
        pub const fn bit(self) -> u8 {
            1 << (self as u8)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Evento {
        BotaoPressionado(Botao),
        BotaoSolto(Botao),
        ComboAutoTeste,
        Tampa { aberta: bool },
        NivelMedio { atingido: bool },
        NivelAlto { atingido: bool },
        Tique,
        Segundo,
    }
}

mod estado_salvo {
    use crate::opcoes::Opcoes;
    use crate::programa::Programa;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct EstadoSalvo {
        pub programa: Programa,
        pub etapa: u8,
        pub opcoes: Opcoes,
        pub ligado: bool,
    }

    /// No firmware isto enfileira a gravação na flash. Aqui não faz nada.
    pub fn pede_gravacao(_estado: EstadoSalvo) {}
}

mod placa {
    //! Dublê da placa: em vez de mexer em pino, registra o estado.
    //!
    //! Mantém a lógica invertida do hardware real: `baixa()` liga.

    use crate::evento::Botao;
    use crate::opcoes::Opcoes;
    use crate::programa::Programa;
    use std::cell::Cell;

    pub struct Saida {
        pub ligada: Cell<bool>,
    }

    impl Saida {
        pub fn nova() -> Self {
            Self {
                ligada: Cell::new(false),
            }
        }
        pub fn alta(&self) {
            self.ligada.set(false); // nível alto = desligado
        }
        pub fn baixa(&self) {
            self.ligada.set(true); // nível baixo = ligado
        }
        pub fn escreve(&self, nivel_alto: bool) {
            self.ligada.set(!nivel_alto);
        }
        pub fn esta_ligada(&self) -> bool {
            self.ligada.get()
        }
    }

    pub struct Painel {
        pub pesada: Saida,
        pub normal: Saida,
        pub rapido: Saida,
        pub enxaguar: Saida,
        pub centrifugar: Saida,
        pub enxague_extra: Saida,
        pub turbo: Saida,
        pub amaciante: Saida,
        pub nivel_medio: Saida,
        pub nivel_alto: Saida,
    }

    impl Painel {
        pub fn nova() -> Self {
            Self {
                pesada: Saida::nova(),
                normal: Saida::nova(),
                rapido: Saida::nova(),
                enxaguar: Saida::nova(),
                centrifugar: Saida::nova(),
                enxague_extra: Saida::nova(),
                turbo: Saida::nova(),
                amaciante: Saida::nova(),
                nivel_medio: Saida::nova(),
                nivel_alto: Saida::nova(),
            }
        }

        fn programas(&self) -> [&Saida; 5] {
            [
                &self.pesada,
                &self.normal,
                &self.rapido,
                &self.enxaguar,
                &self.centrifugar,
            ]
        }

        pub fn todos(&self) -> [&Saida; 10] {
            [
                &self.pesada,
                &self.normal,
                &self.rapido,
                &self.enxaguar,
                &self.centrifugar,
                &self.enxague_extra,
                &self.turbo,
                &self.amaciante,
                &self.nivel_medio,
                &self.nivel_alto,
            ]
        }

        pub fn apaga_programas(&self) {
            for l in self.programas() {
                l.alta();
            }
        }
        pub fn acende_tudo(&self) {
            for l in self.todos() {
                l.baixa();
            }
        }
        pub fn apaga_tudo(&self) {
            for l in self.todos() {
                l.alta();
            }
        }
        pub fn mostra_programa(&self, p: Programa) {
            self.apaga_programas();
            self.programas()[p.indice() as usize].baixa();
        }
        pub fn mostra_opcoes(&self, o: &Opcoes, ligada: bool) {
            let v = |x: bool| ligada && x;
            self.enxague_extra.escreve(!v(o.enxague_extra));
            self.turbo.escreve(!v(o.turbo));
            self.amaciante.escreve(!v(o.amaciante));
            self.nivel_medio.escreve(!v(ligada));
            self.nivel_alto.escreve(!v(o.nivel_alto));
        }
    }

    pub struct Atuadores {
        pub motor_c: Saida,
        pub motor_a: Saida,
        pub eletrobomba: Saida,
        pub valvula_sabao: Saida,
        pub valvula_amaciante: Saida,
    }

    impl Atuadores {
        pub fn novos() -> Self {
            Self {
                motor_c: Saida::nova(),
                motor_a: Saida::nova(),
                eletrobomba: Saida::nova(),
                valvula_sabao: Saida::nova(),
                valvula_amaciante: Saida::nova(),
            }
        }
        pub fn desliga_todos(&self) {
            self.motor_c.alta();
            self.motor_a.alta();
            self.eletrobomba.alta();
            self.valvula_sabao.alta();
            self.valvula_amaciante.alta();
        }
        pub fn motor_a(&self, ligar: bool) {
            self.motor_a.escreve(!ligar);
        }
        pub fn motor_c(&self, ligar: bool) {
            self.motor_c.escreve(!ligar);
        }
        pub fn eletrobomba(&self, ligar: bool) {
            self.eletrobomba.escreve(!ligar);
        }
        pub fn valvula_sabao(&self, abrir: bool) {
            self.valvula_sabao.escreve(!abrir);
        }
        pub fn valvula_amaciante(&self, abrir: bool) {
            self.valvula_amaciante.escreve(!abrir);
        }
        pub fn algum_ligado(&self) -> bool {
            self.motor_c.esta_ligada()
                || self.motor_a.esta_ligada()
                || self.eletrobomba.esta_ligada()
                || self.valvula_sabao.esta_ligada()
                || self.valvula_amaciante.esta_ligada()
        }
    }

    pub struct Entradas {
        pub nivel_medio: Cell<bool>,
        pub nivel_alto: Cell<bool>,
        pub tampa: Cell<bool>,
    }

    impl Entradas {
        pub fn novas() -> Self {
            Self {
                nivel_medio: Cell::new(false),
                nivel_alto: Cell::new(false),
                tampa: Cell::new(false),
            }
        }
        pub fn botao_pressionado(&self, _b: Botao) -> bool {
            false
        }
        pub fn nivel_medio_atingido(&self) -> bool {
            self.nivel_medio.get()
        }
        pub fn nivel_alto_atingido(&self) -> bool {
            self.nivel_alto.get()
        }
        pub fn tampa_aberta(&self) -> bool {
            self.tampa.get()
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// MÓDULOS REAIS DO FIRMWARE
// ═════════════════════════════════════════════════════════════════════════════

#[path = "../src/agitacao.rs"]
mod agitacao;
#[path = "../src/maquina.rs"]
mod maquina;
#[path = "../src/opcoes.rs"]
mod opcoes;
#[path = "../src/programa.rs"]
mod programa;

use estado_salvo::EstadoSalvo;
use evento::{Botao, Evento};
use opcoes::Opcoes;
use placa::{Atuadores, Entradas, Painel};
use programa::Programa;

// ═════════════════════════════════════════════════════════════════════════════
// MODELO DO TANQUE
// ═════════════════════════════════════════════════════════════════════════════

/// Enchimento/drenagem em unidades arbitrárias de 0 a 100 por segundo.
/// Os tempos são curtos de propósito: o que se verifica aqui é a SEQUÊNCIA
/// de etapas, não a duração delas (isso é `conferir_programas.rs`).
struct Tanque {
    nivel: i32,
}

const NIVEL_MEDIO: i32 = 40;
const NIVEL_ALTO: i32 = 80;

impl Tanque {
    fn novo(nivel: i32) -> Self {
        Self { nivel }
    }

    /// Avança um segundo e devolve `(medio, alto)`.
    fn segundo(&mut self, at: &Atuadores) -> (bool, bool) {
        if at.valvula_sabao.esta_ligada() || at.valvula_amaciante.esta_ligada() {
            self.nivel += 4;
        }
        if at.eletrobomba.esta_ligada() {
            self.nivel -= 5;
        }
        self.nivel = self.nivel.clamp(0, 100);
        (self.nivel >= NIVEL_MEDIO, self.nivel >= NIVEL_ALTO)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SIMULAÇÃO
// ═════════════════════════════════════════════════════════════════════════════

const TIQUES_POR_SEGUNDO: u32 = 20;
/// Teto de segurança: 6 horas simuladas.
const LIMITE_SEGUNDOS: u32 = 6 * 60 * 60;

struct Resultado {
    etapas_vistas: Vec<(Programa, usize)>,
    segundos: u32,
    terminou: bool,
    desligada: bool,
    triacs_ligados: bool,
    programa_final: Programa,
    opcoes_finais: Opcoes,
}

fn simula(programa: Programa, enxague_extra: bool, nivel_inicial: i32) -> Resultado {
    simula_com_opcoes(programa, Opcoes { enxague_extra, ..Opcoes::padrao() }, nivel_inicial)
}

fn simula_com_opcoes(programa: Programa, opcoes: Opcoes, nivel_inicial: i32) -> Resultado {
    let painel = Painel::nova();
    let at = Atuadores::novos();
    let entradas = Entradas::novas();
    // A máquina lê os sensores uma vez no construtor, para nascer com o
    // espelho certo — então o nível inicial tem de estar posto antes.
    entradas.nivel_medio.set(nivel_inicial >= NIVEL_MEDIO);
    entradas.nivel_alto.set(nivel_inicial >= NIVEL_ALTO);
    let mut m = maquina::Maquina::nova(&painel, &at, &entradas);

    // Coloca o programa a testar (e as opções) como se viesse da flash.
    m.restaura(EstadoSalvo {
        programa,
        etapa: 0,
        opcoes,
        ligado: false,
    });

    let mut tanque = Tanque::novo(nivel_inicial);
    let mut medio_antes = nivel_inicial >= NIVEL_MEDIO;
    let mut alto_antes = nivel_inicial >= NIVEL_ALTO;

    // ── Deixa a animação de inicialização terminar ───────────────────────────
    for _ in 0..(10 * TIQUES_POR_SEGUNDO) {
        m.trata(Evento::Tique);
        if m.fotografia().desligada {
            break;
        }
    }

    // ── Liga a máquina ───────────────────────────────────────────────────────
    m.trata(Evento::BotaoPressionado(Botao::OnOff));
    m.trata(Evento::BotaoSolto(Botao::OnOff));

    let mut etapas_vistas: Vec<(Programa, usize)> = Vec::new();
    let mut etapa_anterior: Option<(Programa, usize)> = None;
    let mut entrou_em_execucao = false;
    let mut segundos = 0u32;
    let mut terminou = false;

    // Amostrar a cada TIQUE (50 ms), não a cada segundo: etapas que
    // terminam em menos de um segundo — uma drenagem com o tanque já
    // vazio, por exemplo — existem de verdade e precisam aparecer.
    let registra = |m: &maquina::Maquina,
                    vistas: &mut Vec<(Programa, usize)>,
                    anterior: &mut Option<(Programa, usize)>,
                    entrou: &mut bool| {
        let f = m.fotografia();
        if f.executando {
            *entrou = true;
            let agora = (f.programa, f.etapa);
            if *anterior != Some(agora) {
                vistas.push(agora);
                *anterior = Some(agora);
            }
        }
    };

    while segundos < LIMITE_SEGUNDOS {
        for _ in 0..TIQUES_POR_SEGUNDO {
            m.trata(Evento::Tique);
            registra(
                &m,
                &mut etapas_vistas,
                &mut etapa_anterior,
                &mut entrou_em_execucao,
            );
        }

        // Sensores de nível: só publicam quando mudam, como na placa.
        let (medio, alto) = tanque.segundo(&at);
        if medio != medio_antes {
            m.trata(Evento::NivelMedio { atingido: medio });
            medio_antes = medio;
        }
        if alto != alto_antes {
            m.trata(Evento::NivelAlto { atingido: alto });
            alto_antes = alto;
        }

        m.trata(Evento::Segundo);
        segundos += 1;
        registra(
            &m,
            &mut etapas_vistas,
            &mut etapa_anterior,
            &mut entrou_em_execucao,
        );

        if entrou_em_execucao && !m.fotografia().executando {
            terminou = true;
            break;
        }
    }

    let f = m.fotografia();
    Resultado {
        etapas_vistas,
        segundos,
        terminou,
        desligada: f.desligada,
        triacs_ligados: at.algum_ligado(),
        programa_final: f.programa,
        opcoes_finais: f.opcoes,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// VERIFICAÇÃO
// ═════════════════════════════════════════════════════════════════════════════

/// Como o tanque está quando o usuário aperta o liga/desliga.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cenario {
    /// Tanque vazio — a situação normal.
    Vazio,
    /// Tanque com água parada (ciclo interrompido, energia caiu cheia).
    Cheio,
}

impl Cenario {
    fn nivel(self) -> i32 {
        match self {
            Cenario::Vazio => 0,
            Cenario::Cheio => 100,
        }
    }
    fn nome(self) -> &'static str {
        match self {
            Cenario::Vazio => "tanque vazio",
            Cenario::Cheio => "tanque cheio",
        }
    }
}

/// A cadeia de programas que roda a partir de `inicio`, até a
/// centrifugação encerrar o ciclo.
fn cadeia(inicio: Programa) -> Vec<Programa> {
    let mut v = vec![inicio];
    let mut p = inicio;
    // Limite pelo número de programas: nunca gira sem fim.
    for _ in 0..Programa::QUANTIDADE {
        if p.fim_de_ciclo() == programa::FimDeCiclo::Desliga {
            break;
        }
        p = p.proximo();
        v.push(p);
    }
    v
}

/// Etapas que a máquina deve percorrer, na ordem, em toda a cadeia.
fn etapas_esperadas(inicio: Programa, extra: bool, cenario: Cenario) -> Vec<(Programa, usize)> {
    let mut esperadas = Vec::new();

    for (pos, p) in cadeia(inicio).into_iter().enumerate() {
        // O ENXAGUAR começa com um bloco de drenagem que só roda se
        // houver água — igual ao PASSO 0 do `enxaguar` original ("drena
        // se houver água"). Isso só acontece quando ele é o primeiro da
        // cadeia E o tanque já estava cheio; vindo do programa anterior,
        // a bomba acabou de esvaziar o tanque.
        let primeira_etapa = if p == Programa::Enxaguar
            && !(pos == 0 && cenario == Cenario::Cheio)
        {
            5
        } else {
            0
        };

        for (i, et) in p.etapas().iter().enumerate() {
            if i >= primeira_etapa && (!et.so_enxague_extra || extra) {
                esperadas.push((p, i));
            }
        }
    }

    esperadas
}

fn verifica(inicio: Programa, extra: bool, cenario: Cenario) -> Vec<String> {
    let mut falhas = Vec::new();
    let r = simula(inicio, extra, cenario.nivel());
    let esperadas = etapas_esperadas(inicio, extra, cenario);

    let nomes_cadeia: Vec<String> = cadeia(inicio).iter().map(|p| format!("{p:?}")).collect();
    println!(
        "
=== começando em {:?} — enxágue extra {} — {} — {} min simulados",
        inicio,
        if extra { "LIGADO" } else { "desligado" },
        cenario.nome(),
        r.segundos / 60
    );
    println!("  cadeia esperada: {}", nomes_cadeia.join(" → "));

    // Imprime a sequência agrupada por programa, para caber na tela.
    let mut linha = String::new();
    let mut prog_atual: Option<Programa> = None;
    for &(p, i) in &r.etapas_vistas {
        if prog_atual != Some(p) {
            if prog_atual.is_some() {
                linha.push_str("
              ");
            }
            linha.push_str(&format!("{p:?}: "));
            prog_atual = Some(p);
        }
        let nome = match p.etapas().get(i) {
            Some(et) => format!("{:?}", et.acao),
            None => "??".to_string(),
        };
        linha.push_str(&format!("{i}:{nome} "));
    }
    println!("  percorreu:    {linha}");

    if !r.terminou {
        falhas.push(format!(
            "não terminou em {} min simulados (travou numa etapa?)",
            r.segundos / 60
        ));
    }

    // 1. Todas as etapas da cadeia, na ordem, sem pular e sem repetir.
    if r.etapas_vistas != esperadas {
        let faltando: Vec<String> = esperadas
            .iter()
            .filter(|e| !r.etapas_vistas.contains(e))
            .map(|(p, i)| format!("{p:?}[{i}]"))
            .collect();
        if faltando.is_empty() {
            falhas.push(format!(
                "ordem ou repetição errada: {} etapas percorridas, {} esperadas",
                r.etapas_vistas.len(),
                esperadas.len()
            ));
        } else {
            falhas.push(format!("etapas que nunca rodaram: {}", faltando.join(", ")));
        }
    }

    // 2. A cadeia tem de passar pela centrifugação.
    let centrifugou = r
        .etapas_vistas
        .iter()
        .any(|&(p, _)| p == Programa::Centrifugar);
    if !centrifugou {
        falhas.push("a cadeia nunca chegou na centrifugação".to_string());
    }

    // 3. Nada de triac ligado depois do fim.
    if r.triacs_ligados {
        falhas.push("terminou com algum triac ainda ligado".to_string());
    }

    // 4. O fim da cadeia desliga e zera tudo, como no original.
    if !r.desligada {
        falhas.push("fim da centrifugação não deixou a máquina DESLIGADA".to_string());
    }
    if r.programa_final != Programa::Pesada {
        falhas.push(format!(
            "fim da centrifugação deixou o programa em {:?}, deveria voltar para Pesada",
            r.programa_final
        ));
    }
    if r.opcoes_finais != Opcoes::padrao() {
        falhas.push("fim da centrifugação não zerou as opções".to_string());
    }

    falhas
}

// ═════════════════════════════════════════════════════════════════════════════
// CÓDIGOS DE ERRO PISCADOS NO PAINEL
// ═════════════════════════════════════════════════════════════════════════════
//
// Cada erro pisca o painel inteiro um número próprio de vezes, pausa e
// repete. Conferir isso na placa custaria 40 min de tempo limite por erro,
// então a contagem é verificada aqui: força-se a falha, lê-se o painel
// tique a tique e conta-se o que um técnico contaria olhando a máquina.

/// Como forçar cada erro.
#[derive(Clone, Copy)]
enum Falha {
    /// Válvula aberta e o nível nunca sobe → tempo limite de 40 min.
    NaoEnche,
    /// Bomba ligada e o tanque nunca esvazia → tempo limite de 4 min 20 s.
    NaoDrena,
    /// Agitando com água e o nível cai sozinho → 5 leituras.
    NivelCai,
}

const FALHAS: [Falha; 3] = [Falha::NaoEnche, Falha::NaoDrena, Falha::NivelCai];

impl Falha {
    fn nome(self) -> &'static str {
        match self {
            Falha::NaoEnche => "não encheu (válvula/torneira/sensor)",
            Falha::NaoDrena => "não esvaziou (bomba/mangueira/filtro)",
            Falha::NivelCai => "nível caiu sozinho (vazamento/sensor)",
        }
    }

    fn esperado(self) -> maquina::Erro {
        match self {
            Falha::NaoEnche => maquina::Erro::Encher,
            Falha::NaoDrena => maquina::Erro::Drenar,
            Falha::NivelCai => maquina::Erro::Nivel,
        }
    }

    /// Programa que começa na etapa certa para a falha acontecer.
    fn programa(self) -> Programa {
        match self {
            // Etapa 0 = EncherSabao.
            Falha::NaoEnche | Falha::NivelCai => Programa::Pesada,
            // Etapa 0 = Drenar.
            Falha::NaoDrena => Programa::Centrifugar,
        }
    }

    /// Sensores no instante de energizar: `(médio, alto)`.
    fn nivel_inicial(self) -> (bool, bool) {
        match self {
            Falha::NaoEnche => (false, false), // tanque vazio, e nunca enche
            Falha::NaoDrena => (true, true),   // tanque cheio, e nunca esvazia
            Falha::NivelCai => (true, false),  // já no nível: a agitação começa
        }
    }
}

/// Teto da busca pelo erro: 50 min simulados, folga sobre os 40 min do
/// tempo limite de enchimento, que é o mais longo.
const LIMITE_BUSCA_S: u32 = 50 * 60;

/// Força a falha e devolve `(erro, traço do painel)`.
///
/// O traço tem uma amostra por tique de 50 ms, e cada amostra é
/// **quantos** dos 10 LEDs estão acesos — assim o traço também prova que
/// eles piscam juntos, e não um de cada vez.
///
/// A captura começa no tique seguinte ao instante em que o erro apareceu,
/// que é exatamente onde o contador do pisca foi zerado — então o traço
/// está em fase com o início da contagem, igual ao que se vê na placa.
fn captura_codigo(falha: Falha) -> Option<(maquina::Erro, Vec<usize>)> {
    let painel = Painel::nova();
    let at = Atuadores::novos();
    let entradas = Entradas::novas();

    // A máquina lê os sensores uma vez no construtor: o nível tem de estar
    // posto antes.
    let (medio, alto) = falha.nivel_inicial();
    entradas.nivel_medio.set(medio);
    entradas.nivel_alto.set(alto);

    let mut m = maquina::Maquina::nova(&painel, &at, &entradas);
    m.restaura(EstadoSalvo {
        programa: falha.programa(),
        etapa: 0,
        opcoes: Opcoes {
            nivel_alto: false,
            enxague_extra: false,
            turbo: false,
            amaciante: false,
        },
        ligado: false,
    });

    // Deixa a animação de inicialização terminar e liga a máquina.
    for _ in 0..(10 * TIQUES_POR_SEGUNDO) {
        m.trata(Evento::Tique);
        if m.fotografia().desligada {
            break;
        }
    }
    m.trata(Evento::BotaoPressionado(Botao::OnOff));
    m.trata(Evento::BotaoSolto(Botao::OnOff));

    // Roda com os sensores TRAVADOS — é isso que estoura o tempo limite.
    let mut erro = None;
    for s in 0..LIMITE_BUSCA_S {
        for _ in 0..TIQUES_POR_SEGUNDO {
            m.trata(Evento::Tique);
        }
        // O vazamento: 10 s depois de ligar, já dentro da agitação e
        // passada a carência de 3 s do sensor.
        if matches!(falha, Falha::NivelCai) && s == 10 {
            m.trata(Evento::NivelMedio { atingido: false });
        }
        m.trata(Evento::Segundo);
        if let Some(e) = m.fotografia().erro {
            erro = Some(e);
            break;
        }
    }
    let erro = erro?;

    // Três repetições completas do código.
    let ciclo = (erro.piscadas() * maquina::TIQUES_ERRO_PISCADA
        + maquina::TIQUES_ERRO_PAUSA) as usize;
    let mut traco = Vec::with_capacity(ciclo * 3);
    for _ in 0..(ciclo * 3) {
        m.trata(Evento::Tique);
        traco.push(painel.todos().iter().filter(|l| l.esta_ligada()).count());
    }

    Some((erro, traco))
}

/// Comprimentos dos trechos acesos e dos trechos apagados, em tiques.
fn trechos(traco: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut acesos = Vec::new();
    let mut apagados = Vec::new();
    let mut i = 0;
    while i < traco.len() {
        let ligado = traco[i] > 0;
        let inicio = i;
        while i < traco.len() && (traco[i] > 0) == ligado {
            i += 1;
        }
        if ligado {
            acesos.push(i - inicio);
        } else {
            apagados.push(i - inicio);
        }
    }
    (acesos, apagados)
}

fn verifica_codigo(falha: Falha) -> Vec<String> {
    let mut problemas = Vec::new();

    let Some((erro, traco)) = captura_codigo(falha) else {
        return vec![format!(
            "a falha \"{}\" não levou a máquina a erro nenhum em {} min simulados",
            falha.nome(),
            LIMITE_BUSCA_S / 60
        )];
    };

    // 1. É o erro certo?
    if erro != falha.esperado() {
        problemas.push(format!(
            "entrou em {:?}, esperado {:?}",
            erro,
            falha.esperado()
        ));
    }

    let piscadas = erro.piscadas() as usize;
    println!(
        "=== {} → {:?}, {} piscadas",
        falha.nome(),
        erro,
        piscadas
    );

    // 2. Os 10 LEDs andam juntos — nunca um subconjunto aceso.
    if let Some(&n) = traco.iter().find(|&&n| n != 0 && n != 10) {
        problemas.push(format!(
            "o painel mostrou {n} LED(s) acesos; o código pisca os 10 juntos"
        ));
    }

    let (acesos, apagados) = trechos(&traco);

    // 3. Três repetições → 3 × N piscadas, nenhuma a mais, nenhuma a menos.
    if acesos.len() != piscadas * 3 {
        problemas.push(format!(
            "contei {} piscadas em 3 repetições, esperado {} (3 × {piscadas})",
            acesos.len(),
            piscadas * 3
        ));
    }

    // 4. Toda piscada tem a mesma duração.
    if acesos
        .iter()
        .any(|&t| t != maquina::TIQUES_ERRO_ACESO as usize)
    {
        problemas.push(format!(
            "piscadas com durações diferentes: {acesos:?} tiques (esperado {})",
            maquina::TIQUES_ERRO_ACESO
        ));
    }

    // 5. Existem só dois tamanhos de escuro: o intervalo entre piscadas e a
    //    pausa. E a pausa tem de ser bem maior, senão não se sabe onde a
    //    contagem começa.
    let curto = maquina::TIQUES_ERRO_APAGADO as usize;
    let longo = (maquina::TIQUES_ERRO_APAGADO + maquina::TIQUES_ERRO_PAUSA) as usize;
    if let Some(&t) = apagados.iter().find(|&&t| t != curto && t != longo) {
        problemas.push(format!(
            "escuro de {t} tiques fora de padrão (esperado {curto} entre piscadas ou {longo} na pausa)"
        ));
    }
    if longo < curto * 3 {
        problemas.push(format!(
            "a pausa ({longo} tiques) não é bem maior que o intervalo entre piscadas ({curto}); a contagem fica ambígua"
        ));
    }

    // 6. Desenha o que o técnico vê, para conferir de olho.
    let desenho: String = traco
        .iter()
        .take(traco.len() / 3)
        .map(|&n| if n > 0 { '▀' } else { '_' })
        .collect();
    println!(
        "  {desenho}  ({:.1} s por repetição)",
        traco.len() as f32 / 3.0 * 0.05
    );

    problemas
}

/// Dois erros com o mesmo número de piscadas seriam indistinguíveis.
fn verifica_codigos_distintos() -> Vec<String> {
    let mut problemas = Vec::new();
    for (i, a) in FALHAS.iter().enumerate() {
        for b in FALHAS.iter().skip(i + 1) {
            if a.esperado().piscadas() == b.esperado().piscadas() {
                problemas.push(format!(
                    "{:?} e {:?} piscam as duas {} vezes — indistinguíveis",
                    a.esperado(),
                    b.esperado(),
                    a.esperado().piscadas()
                ));
            }
        }
    }
    // Uma piscada só se confundiria com o pisca de tampa aberta.
    for f in FALHAS {
        if f.esperado().piscadas() < 2 {
            problemas.push(format!(
                "{:?} pisca {} vez; o código tem de começar em 2",
                f.esperado(),
                f.esperado().piscadas()
            ));
        }
    }
    problemas
}

const TODOS: [Programa; 5] = [
    Programa::Pesada,
    Programa::Normal,
    Programa::Rapido,
    Programa::Enxaguar,
    Programa::Centrifugar,
];

#[cfg(test)]
mod testes {
    use super::*;

    fn tiques(m: &mut maquina::Maquina<'_>, n: usize) {
        for _ in 0..n {
            m.trata(Evento::Tique);
        }
    }

    #[test]
    fn todas_as_opcoes_completam_os_ciclos() {
        for p in TODOS {
            for bits in 0..16 {
                let opcoes = Opcoes::de_byte(bits);
                for cenario in [Cenario::Vazio, Cenario::Cheio] {
                    let r = simula_com_opcoes(p, opcoes, cenario.nivel());
                    assert!(r.terminou && r.desligada && !r.triacs_ligados,
                        "programa={p:?}, opções={bits}, tanque={}", cenario.nome());
                    assert_eq!(r.etapas_vistas, etapas_esperadas(p, opcoes.enxague_extra, cenario));
                    assert_eq!(r.programa_final, Programa::Pesada);
                    assert!(r.opcoes_finais == Opcoes::padrao());
                }
            }
        }
    }

    #[test]
    fn queda_de_nivel_no_molho_enxague_desliga_saidas() {
        for amaciante in [false, true] {
            for nivel_alto in [false, true] {
                let painel = Painel::nova();
                let at = Atuadores::novos();
                let entradas = Entradas::novas();
                entradas.nivel_medio.set(true);
                entradas.nivel_alto.set(nivel_alto);
                let mut m = maquina::Maquina::nova(&painel, &at, &entradas);
                m.restaura(EstadoSalvo {
                    programa: Programa::Enxaguar, etapa: 7, ligado: true,
                    opcoes: Opcoes { amaciante, nivel_alto, ..Opcoes::padrao() },
                });
                tiques(&mut m, 90);
                assert!(m.fotografia().executando);
                // Com amaciante, exercita também a fase que agita.
                if amaciante {
                    for _ in 0..301 {
                        tiques(&mut m, 20);
                        m.trata(Evento::Segundo);
                    }
                    let mut agitou = false;
                    for _ in 0..20 {
                        tiques(&mut m, 1);
                        agitou |= at.motor_a.esta_ligada() || at.motor_c.esta_ligada();
                    }
                    assert!(agitou);
                }
                m.trata(if nivel_alto { Evento::NivelAlto { atingido: false } }
                    else { Evento::NivelMedio { atingido: false } });
                for _ in 0..8 {
                    tiques(&mut m, 20);
                    m.trata(Evento::Segundo);
                }
                assert_eq!(m.fotografia().erro, Some(maquina::Erro::Nivel));
                assert!(!at.algum_ligado());
            }
        }
    }

    #[test]
    fn tampa_pausa_e_retoma_agitacao_com_espera() {
        let painel = Painel::nova();
        let at = Atuadores::novos();
        let entradas = Entradas::novas();
        entradas.nivel_medio.set(true);
        let mut m = maquina::Maquina::nova(&painel, &at, &entradas);
        m.restaura(EstadoSalvo { programa: Programa::Rapido, etapa: 1,
            opcoes: Opcoes::padrao(), ligado: true });
        tiques(&mut m, 91);
        assert!(at.motor_a.esta_ligada() || at.motor_c.esta_ligada());
        m.trata(Evento::Tampa { aberta: true });
        let antes = m.fotografia();
        for _ in 0..5 {
            tiques(&mut m, 20);
            m.trata(Evento::Segundo);
            assert!(!at.algum_ligado());
        }
        assert_eq!(m.fotografia().segundos_etapa, antes.segundos_etapa);
        m.trata(Evento::Tampa { aberta: false });
        tiques(&mut m, 39);
        assert!(!at.algum_ligado());
        tiques(&mut m, 2);
        assert!(m.fotografia().executando);
        assert_eq!(m.fotografia().etapa, antes.etapa);
        assert!(at.motor_a.esta_ligada() || at.motor_c.esta_ligada());
    }

    #[test]
    fn boot_restaura_etapa_e_respeita_estado_ligado() {
        for ligado in [false, true] {
            let painel = Painel::nova();
            let at = Atuadores::novos();
            let entradas = Entradas::novas();
            let mut m = maquina::Maquina::nova(&painel, &at, &entradas);
            m.restaura(EstadoSalvo { programa: Programa::Centrifugar, etapa: 3,
                opcoes: Opcoes::padrao(), ligado });
            tiques(&mut m, 89);
            assert!(!at.algum_ligado());
            tiques(&mut m, 1);
            assert_eq!(m.fotografia().executando, ligado);
            assert_eq!(m.fotografia().desligada, !ligado);
            assert_eq!(m.fotografia().etapa, 3);
            if ligado {
                assert!(at.eletrobomba.esta_ligada());
                assert!(!at.motor_c.esta_ligada());
                tiques(&mut m, 199);
                assert!(!at.motor_c.esta_ligada());
                tiques(&mut m, 1);
                assert!(at.motor_c.esta_ligada());
            }
        }
    }
}

fn main() {
    let mut problemas = 0;

    let mut rodados = 0;
    for p in TODOS {
        for extra in [false, true] {
            for cenario in [Cenario::Vazio, Cenario::Cheio] {
            rodados += 1;
            let falhas = verifica(p, extra, cenario);
            if falhas.is_empty() {
                println!("  ✔ sem problemas");
            } else {
                problemas += falhas.len();
                for f in falhas {
                    println!("  ✘ {f}");
                }
            }
            }
        }
    }

    // ── Os códigos de erro piscados no painel ────────────────────────────────
    println!();
    for f in verifica_codigos_distintos() {
        problemas += 1;
        println!("  ✘ {f}");
    }
    for falha in FALHAS {
        let falhas = verifica_codigo(falha);
        if falhas.is_empty() {
            println!("  ✔ sem problemas");
        } else {
            problemas += falhas.len();
            for f in falhas {
                println!("  ✘ {f}");
            }
        }
    }

    println!();
    if problemas == 0 {
        println!(
            "Tudo certo: {rodados} ciclos simulados de ponta a ponta \
             e {} códigos de erro conferidos.",
            FALHAS.len()
        );
    } else {
        println!("{problemas} problema(s) encontrado(s).");
        std::process::exit(1);
    }
}
