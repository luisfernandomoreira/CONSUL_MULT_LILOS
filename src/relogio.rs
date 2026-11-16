//! =============================================================
//! Relógio do ciclo — o TIM14 do firmware C++
//!
//! Um contador de segundos que pode ser **parado**: é com ele que o
//! ciclo mede a duração de cada etapa, e pará-lo é exatamente o que
//! acontece quando a tampa abre ou a máquina entra em pausa. Ao
//! fechar a tampa o ciclo continua de onde estava.
//!
//! Três contadores saem da mesma base de 1 s:
//!
//!   `total_segundos`  tempo decorrido na etapa; a etapa decide quando
//!                     zerar
//!   `delay_5`         satura em 5 — atraso curto de estabilização
//!                     elétrica no começo de cada etapa
//!   `delay_25`        satura em 25 — atraso longo, usado na drenagem
//!
//! Por que fica num `static` em vez de dentro da `Maquina`: a tarefa
//! de 1 s precisa continuar contando mesmo enquanto a máquina de
//! estados está ocupada (por exemplo piscando LEDs de erro). É o mesmo
//! papel das variáveis `volatile static` da classe `Tempo` do C++,
//! que a interrupção do TIM14 atualiza.
//! =============================================================

use core::cell::Cell;
use critical_section::Mutex;

/// Atraso de estabilização que as etapas exigem antes de agir.
pub const MIN_DELAY_5: u16 = 3;
/// Atraso longo usado na drenagem e na centrifugação.
pub const MIN_DELAY_25: u16 = 18;

const TETO_DELAY_5: u16 = 5;
const TETO_DELAY_25: u16 = 25;

#[derive(Clone, Copy, Default)]
struct Contadores {
    ligado: bool,
    total_segundos: u16,
    delay_5: u16,
    delay_25: u16,
}

static RELOGIO: Mutex<Cell<Contadores>> = Mutex::new(Cell::new(Contadores {
    ligado: false,
    total_segundos: 0,
    delay_5: 0,
    delay_25: 0,
}));

fn com<R>(f: impl FnOnce(&mut Contadores) -> R) -> R {
    critical_section::with(|cs| {
        let celula = RELOGIO.borrow(cs);
        let mut c = celula.get();
        let r = f(&mut c);
        celula.set(c);
        r
    })
}

/// Chamada pela tarefa de 1 s. Só conta com o relógio ligado.
pub fn atualiza() {
    com(|c| {
        if !c.ligado {
            return;
        }
        c.total_segundos = c.total_segundos.saturating_add(1);
        if c.delay_5 < TETO_DELAY_5 {
            c.delay_5 += 1;
        }
        if c.delay_25 < TETO_DELAY_25 {
            c.delay_25 += 1;
        }
    });
}

/// Liga ou para a contagem (o `controlaTimer14` do C++).
pub fn liga(ligado: bool) {
    com(|c| c.ligado = ligado);
}

pub fn total_segundos() -> u16 {
    com(|c| c.total_segundos)
}

pub fn delay_5() -> u16 {
    com(|c| c.delay_5)
}

pub fn delay_25() -> u16 {
    com(|c| c.delay_25)
}

pub fn set_total_segundos(valor: u16) {
    com(|c| c.total_segundos = valor);
}

/// Usado para pular o atraso de estabilização quando a espera já foi
/// cumprida de outra forma (os 2 s de tampa ou de encoder).
pub fn set_delay_5(valor: u16) {
    com(|c| c.delay_5 = valor);
}

/// Zera os três contadores — o `resetHalltempo` do C++.
pub fn reset_tudo() {
    com(|c| {
        c.total_segundos = 0;
        c.delay_5 = 0;
        c.delay_25 = 0;
    });
}

/// Zera apenas os atrasos — o `resetAtrasos` do C++.
pub fn reset_atrasos() {
    com(|c| {
        c.delay_5 = 0;
        c.delay_25 = 0;
    });
}
