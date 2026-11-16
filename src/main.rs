//! =============================================================
//! Empresa  : Alpe Placas Eletrônicas — Divisão Metalúrgica Alado
//! Autor    : Luis Fernando Moreira
//! Projeto  : CONSUL MULT — Firmware Máquina de Lavar
//! Linguagem: Rust
//! RTOS     : lilos (executor async cooperativo)
//! Chip     : GD32C231K8T6 (Arm Cortex-M23, 64 KB flash, 12 KB RAM)
//! Placa    : a mesma PCI do firmware C++ de fábrica — o GD32C231K8T6
//!            (LQFP32) é pino a pino igual ao STM32C031K6T6 (LQFP32).
//!
//! ─── Como o firmware está organizado ─────────────────────────
//!
//!   mcu/       periféricos do GD32C231 via PAC (clock, GPIO, ADC,
//!              watchdog, flash)
//!   placa      nomes dos pinos da placa CONSUL MULT
//!   modelo     a chave SW1..SW4 que diz qual lavadora é
//!   encoder    o seletor de 16 programas
//!   botoes     os três botões do painel, lidos por ADC
//!   relogio    o contador de segundos do ciclo (pode ser parado)
//!   agita      o vai-e-vem do motor
//!   tabelas    os tempos de cada programa, por modelo
//!   leituras   o que a amostragem viu (sensores, seletor, botões)
//!   evento     o sinal que acorda a máquina de estados
//!   maquina    as 7 etapas do ciclo (lê, decide, age)
//!   estado_salvo  grava o ciclo na flash para retomar após queda de luz
//!   tarefas    as 7 tarefas do lilos
//!
//! Fluxo:
//!
//!   [chave, encoder, botões, sensores] → tarefas::amostragem ─┐
//!   [21 ms agitação]                   → tarefas::agitacao  ──┼→ SINAL
//!   [21 ms tampa]                      → tarefas::tampa       │
//!   [1 s]                              → tarefas::tempo     ──┘
//!                                                              │
//!                                                              ▼
//!                                                     tarefas::maquina
//!                                                              │
//!                                               [triacs e LEDs do painel]
//! =============================================================

#![no_std]
#![no_main]
// No build de bancada (`--features diag-botoes`) a maquina de estados
// nao e instanciada de proposito: o diagnostico dos botoes entra no
// lugar dela. Todo o ciclo fica sem uso, e isso nao e aviso util.
#![cfg_attr(feature = "diag-botoes", allow(dead_code))]

use panic_halt as _; // panic → trava o núcleo; o FWDGT reinicia em 2,8 s

use core::pin::pin;
use cortex_m_rt::entry;

mod agita;
mod botoes;
mod diag;
mod encoder;
mod estado_salvo;
mod evento;
mod leituras;
mod maquina;
mod mcu;
mod modelo;
mod placa;
mod relogio;
mod tabelas;
mod tarefas;

#[entry]
fn main() -> ! {
    // ─── 1. Periféricos do chip (singleton do PAC) ───────────────────────────
    let p = mcu::pac::Peripherals::take().unwrap();

    // ─── 2. Núcleo a 48 MHz ──────────────────────────────────────────────────
    // O firmware C++ roda a 24 MHz, mas todos os tempos do ciclo são em
    // milissegundos e segundos, então a frequência não muda o
    // comportamento — só dá folga para o executor.
    mcu::clock::configura_48mhz(&p.fmc, &p.rcu);

    // ─── 3. Base de tempo do lilos (SysTick de 1 ms) ─────────────────────────
    let mut cp = cortex_m::Peripherals::take().unwrap();
    lilos::time::initialize_sys_tick(&mut cp.SYST, mcu::clock::FREQUENCIA_HZ);

    // ─── 4. Pinos da placa — triacs e LEDs já saem daqui desligados ──────────
    let (saidas, entradas) = placa::inicializa(&p.rcu);

    // ─── 5. ADC dos botões do painel (PA6) ───────────────────────────────────
    mcu::adc::inicializa(&p.rcu, &p.adc);

    // ─── 6. Primeira leitura dos sensores ────────────────────────────────────
    // A máquina lê o estado pelo módulo `leituras`, que as tarefas de
    // amostragem alimentam. Semeamos aqui para a partida não decidir
    // em cima de zeros.
    leituras::set_posicao_encoder(encoder::posicao(entradas.trilhas_encoder()));
    leituras::set_tampas(
        entradas.tampa_aberta(),
        entradas.tampa_mecanica_aberta(),
    );
    // Pressostato em nível baixo = cheio.
    leituras::set_nivel_agua(!entradas.pressostato());

    // ─── 7. Relógio do ciclo ligado ──────────────────────────────────────────
    // Equivale ao `HAL_TIM_Base_Start_IT(&htim14)` do `main()` do C++:
    // a contagem começa no boot e só para quando a tampa abre, a
    // máquina é pausada ou o ciclo termina.
    relogio::liga(true);

    // ─── 8. As sete tarefas ──────────────────────────────────────────────────
    // `pin!` fixa cada future na pilha do `main`, que nunca retorna —
    // por isso as tarefas podem emprestar `saidas` e `entradas`.
    let t_tempo = pin!(tarefas::tempo());
    let t_agitacao = pin!(tarefas::agitacao());
    let t_tampa = pin!(tarefas::tampa(&entradas));
    let t_amostragem = pin!(tarefas::amostragem(&entradas, &p.adc));
    // Build de bancada (`--features diag-botoes`): o diagnostico dos
    // botoes entra no lugar da maquina de estados. Veja src/diag.rs.
    #[cfg(not(feature = "diag-botoes"))]
    let t_maquina = pin!(tarefas::maquina(&saidas, &entradas));
    #[cfg(feature = "diag-botoes")]
    let t_maquina = pin!(tarefas::diagnostico_botoes(&saidas, &p.adc));
    let t_gravacao = pin!(tarefas::gravacao(&p.fmc));
    // No build de bancada o watchdog fica de fora: assim o `probe-rs`
    // pode parar o nucleo para ler memoria sem a placa resetar em 2,8 s.
    #[cfg(not(feature = "diag-botoes"))]
    let t_watchdog = pin!(tarefas::watchdog(&p.fwdgt, &p.rcu));

    // ─── 9. Entrega o controle ao executor (nunca retorna) ───────────────────
    lilos::exec::run_tasks(
        &mut [
            t_tempo,
            t_agitacao,
            t_tampa,
            t_amostragem,
            t_maquina,
            t_gravacao,
            #[cfg(not(feature = "diag-botoes"))]
            t_watchdog,
        ],
        lilos::exec::ALL_TASKS,
    )
}
