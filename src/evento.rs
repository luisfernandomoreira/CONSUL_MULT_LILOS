//! =============================================================
//! Núcleo do firmware: EVENTOS
//!
//! Todo o firmware conversa por aqui. Quem lê hardware **publica**
//! eventos; a máquina de estados **consome** eventos. Ninguém mais
//! precisa conhecer ninguém.
//!
//!   tarefa_entradas ─┐
//!   tarefa_tempo  ───┼─▶ [ FILA ] ─▶ tarefa_maquina ─▶ atuadores/painel
//!                    │
//!   (futuro: UART,   ┘
//!    EXTI, sensor…)
//!
//! Para adicionar uma funcionalidade nova:
//!   1. acrescente uma variante em `Evento`;
//!   2. publique-a de onde ela nasce;
//!   3. trate-a no `match` da máquina de estados.
//!
//! Nada mais muda.
//!
//! =============================================================

use core::cell::RefCell;
use critical_section::Mutex;
use lilos::exec::Notify;

/// Botões do painel frontal.
///
/// Os números são o índice do botão nos vetores da tarefa de entradas e
/// o bit dele no espelho de botões da máquina de estados.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Botao {
    /// Liga/desliga geral (PC14).
    OnOff = 0,
    /// Seletor de programa (PA5) — também cancela o ciclo em andamento.
    Seletor = 1,
    /// Nível de água: médio ↔ alto (PC15).
    Nivel = 2,
    /// Enxágue extra (PA12).
    EnxagueExtra = 3,
    /// Agitação turbo (PB9).
    Turbo = 4,
    /// Amaciante (PA10).
    Amaciante = 5,
}

impl Botao {
    /// Todos os botões, na ordem dos índices.
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

    /// Bit deste botão numa máscara de 6 bits.
    pub const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

/// Tudo o que pode acontecer na máquina.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Evento {
    /// Botão pressionado — já filtrado por debounce de 50 ms.
    BotaoPressionado(Botao),
    /// Botão solto — mesmo debounce.
    BotaoSolto(Botao),
    /// Seletor e Nível pressionados ao mesmo tempo: é o gesto que abre o
    /// modo de auto-teste (a máquina só aceita nos primeiros 15 s, ver
    /// `maquina.rs`). Publicado uma única vez por combinação.
    ComboAutoTeste,
    /// Estado da tampa mudou. `true` = aberta. Debounce de 200 ms.
    Tampa { aberta: bool },
    /// Sensor de nível médio mudou. `true` = nível atingido.
    NivelMedio { atingido: bool },
    /// Sensor de nível alto mudou. `true` = nível atingido.
    NivelAlto { atingido: bool },
    /// Base de tempo rápida — 50 ms. Move a agitação, os piscas e a
    /// animação de inicialização.
    Tique,
    /// Base de tempo lenta — 1 s. Conta a duração das etapas.
    Segundo,
}

// ═════════════════════════════════════════════════════════════════════════════
// FILA
// ═════════════════════════════════════════════════════════════════════════════

/// Quantos eventos cabem na fila antes de começar a descartar.
///
/// A máquina consome um evento por vez e nunca bloqueia, então na prática
/// a fila fica quase sempre vazia. 16 é folga confortável.
const CAPACIDADE: usize = 16;

struct Buffer {
    itens: [Evento; CAPACIDADE],
    inicio: usize,
    quantidade: usize,
    /// Conta eventos descartados por fila cheia. Se isto sair de zero,
    /// alguma tarefa está demorando demais — bom ponto de diagnóstico.
    perdidos: u16,
}

/// Fila circular de eventos, segura para uso entre tarefas e interrupções.
pub struct FilaEventos {
    buffer: Mutex<RefCell<Buffer>>,
    aviso: Notify,
}

impl FilaEventos {
    const fn nova() -> Self {
        Self {
            buffer: Mutex::new(RefCell::new(Buffer {
                itens: [Evento::Tique; CAPACIDADE],
                inicio: 0,
                quantidade: 0,
                perdidos: 0,
            })),
            aviso: Notify::new(),
        }
    }

    /// Coloca um evento na fila e acorda quem estiver esperando.
    ///
    /// Pode ser chamada de qualquer tarefa e também de uma interrupção.
    /// Se a fila estiver cheia o evento é descartado (e contado).
    pub fn publica(&self, evento: Evento) {
        critical_section::with(|cs| {
            let mut b = self.buffer.borrow_ref_mut(cs);
            if b.quantidade == CAPACIDADE {
                b.perdidos = b.perdidos.saturating_add(1);
                return;
            }
            let fim = (b.inicio + b.quantidade) % CAPACIDADE;
            b.itens[fim] = evento;
            b.quantidade += 1;
        });
        self.aviso.notify();
    }

    /// Espera até haver um evento e o retira da fila.
    pub async fn proximo(&self) -> Evento {
        // `until_racy` (em vez de `until`) assina o aviso ANTES de testar a
        // fila. Custa alguns ciclos a mais, mas garante que um evento
        // publicado por uma interrupção nunca seja perdido — margem de
        // segurança para quando alguém acrescentar uma ISR aqui no futuro.
        self.aviso.until_racy(|| self.retira()).await
    }

    fn retira(&self) -> Option<Evento> {
        critical_section::with(|cs| {
            let mut b = self.buffer.borrow_ref_mut(cs);
            if b.quantidade == 0 {
                return None;
            }
            let evento = b.itens[b.inicio];
            b.inicio = (b.inicio + 1) % CAPACIDADE;
            b.quantidade -= 1;
            Some(evento)
        })
    }

    /// Quantos eventos já foram descartados por fila cheia (diagnóstico).
    #[allow(dead_code)]
    pub fn perdidos(&self) -> u16 {
        critical_section::with(|cs| self.buffer.borrow_ref(cs).perdidos)
    }
}

/// A fila única do firmware.
pub static FILA: FilaEventos = FilaEventos::nova();

/// Atalho: publica um evento na fila global.
#[inline]
pub fn publica(evento: Evento) {
    FILA.publica(evento);
}

/// Atalho: espera o próximo evento da fila global.
#[inline]
pub async fn proximo() -> Evento {
    FILA.proximo().await
}
