//! =============================================================
//! Diagnóstico — espiar o ADC com a placa rodando
//!
//! Esta placa não tem UART nem display, então não há como imprimir
//! nada. O jeito de ver o que o ADC está lendo é deixar os valores em
//! endereços de RAM com **símbolo fixo** (`#[no_mangle]`) e lê-los por
//! SWD com o `probe-rs`, sem parar o núcleo.
//!
//! Nada aqui é lido pela lógica do firmware: são só quatro contadores
//! de observação. Custam 8 bytes de RAM e algumas instruções a cada
//! 81 ms.
//!
//! ─── Como usar ───────────────────────────────────────────────
//!
//! Levante os endereços no ELF (eles mudam a cada compilação):
//!
//! ```bash
//! arm-none-eabi-nm target/thumbv6m-none-eabi/release/consul_mult_lilos \
//!     | grep -i DIAG_
//! ```
//!
//! Com a placa rodando, leia os quatro de uma vez:
//!
//! ```bash
//! probe-rs read b16 0x2000XXXX 4 \
//!     --chip GD32C231K8 \
//!     --chip-description-path target-probe/GD32C231K8.yaml
//! ```
//!
//! ─── Como interpretar ────────────────────────────────────────
//!
//! | O que você vê | O que significa |
//! |---|---|
//! | `FALHA` subindo e `OK` em 0 | o ADC não converte — canal, clock ou habilitação |
//! | `OK` subindo e `AD` sempre 0 | converte, mas o resultado vem zero — entrada ou canal errado |
//! | `MIN`/`MAX` em repouso fora de 7..60 | a faixa de repouso não bate; **nenhum botão funciona** |
//! | `MIN`/`MAX` em repouso dentro de 7..60 | o ADC está bom; o problema é outro |
//!
//! Para descobrir o valor de um botão: segure-o e leia `AD` algumas
//! vezes. Para a faixa de repouso: religue a placa, não toque em nada
//! e leia `MIN` e `MAX`.
//! =============================================================

use core::ptr::addr_of_mut;

/// Última leitura crua do ADC dos botões.
#[no_mangle]
pub static mut DIAG_AD: u16 = 0;

/// Menor leitura vista desde a partida.
#[no_mangle]
pub static mut DIAG_AD_MIN: u16 = u16::MAX;

/// Maior leitura vista desde a partida.
#[no_mangle]
pub static mut DIAG_AD_MAX: u16 = 0;

/// Quantas conversões terminaram bem.
#[no_mangle]
pub static mut DIAG_ADC_OK: u16 = 0;

/// Quantas conversões estouraram o tempo limite.
#[no_mangle]
pub static mut DIAG_ADC_FALHA: u16 = 0;

/// Registra uma conversão bem-sucedida.
///
/// Chamada só pela tarefa de amostragem, que é a única escritora — e,
/// num executor cooperativo de um núcleo, não há outra tarefa no meio
/// da escrita.
pub fn registra_leitura(valor: u16) {
    // SAFETY: escrita volátil num `static mut` que ninguém mais
    // escreve. O `probe-rs` lê por SWD, fora do núcleo.
    unsafe {
        core::ptr::write_volatile(addr_of_mut!(DIAG_AD), valor);

        let min = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MIN));
        if valor < min {
            core::ptr::write_volatile(addr_of_mut!(DIAG_AD_MIN), valor);
        }

        let max = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MAX));
        if valor > max {
            core::ptr::write_volatile(addr_of_mut!(DIAG_AD_MAX), valor);
        }

        let ok = core::ptr::read_volatile(addr_of_mut!(DIAG_ADC_OK));
        core::ptr::write_volatile(addr_of_mut!(DIAG_ADC_OK), ok.wrapping_add(1));
    }
}

/// Registra uma conversão que não terminou.
pub fn registra_falha() {
    // SAFETY: idem `registra_leitura`.
    unsafe {
        let n = core::ptr::read_volatile(addr_of_mut!(DIAG_ADC_FALHA));
        core::ptr::write_volatile(addr_of_mut!(DIAG_ADC_FALHA), n.wrapping_add(1));
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Leitura pelo próprio firmware (modo de diagnóstico dos botões)
// ═════════════════════════════════════════════════════════════════════════════

/// Quantas conversões já terminaram bem.
///
/// O modo de diagnóstico usa isto para saber se o ADC está vivo: se o
/// contador não anda, não é faixa errada — é conversão não acontecendo.
#[cfg(feature = "diag-botoes")]
pub fn conversoes_ok() -> u16 {
    // SAFETY: leitura volátil de um `static mut` que só a tarefa de
    // amostragem escreve.
    unsafe { core::ptr::read_volatile(addr_of_mut!(DIAG_ADC_OK)) }
}

/// Diferenca entre a maior e a menor leitura desde a partida.
///
/// Se isto fica perto de zero depois de alguem apertar os botoes, o
/// ADC nao esta vendo o divisor resistivo: pino, canal ou modo
/// analogico.
#[cfg(feature = "diag-botoes")]
pub fn amplitude() -> u16 {
    // SAFETY: leituras volateis de `static mut` que so a tarefa de
    // amostragem escreve.
    unsafe {
        let min = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MIN));
        let max = core::ptr::read_volatile(addr_of_mut!(DIAG_AD_MAX));
        max.saturating_sub(min)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// VARREDURA DE CANAIS (build de bancada)
// ═════════════════════════════════════════════════════════════════════════════

/// Quantos canais a varredura cobre.
pub const NUM_CANAIS: usize = 16;

/// Última leitura de cada canal do ADC.
#[no_mangle]
pub static mut DIAG_CANAIS: [u16; NUM_CANAIS] = [0; NUM_CANAIS];

/// Menor leitura vista em cada canal.
#[no_mangle]
pub static mut DIAG_CANAIS_MIN: [u16; NUM_CANAIS] = [u16::MAX; NUM_CANAIS];

/// Maior leitura vista em cada canal.
#[no_mangle]
pub static mut DIAG_CANAIS_MAX: [u16; NUM_CANAIS] = [0; NUM_CANAIS];

/// Guarda a leitura de um canal da varredura.
#[cfg(feature = "diag-botoes")]
pub fn registra_canal(canal: usize, valor: u16) {
    if canal >= NUM_CANAIS {
        return;
    }
    // SAFETY: escrita volátil num `static mut` que só a tarefa de
    // diagnóstico escreve, em índice conferido acima.
    unsafe {
        let base = addr_of_mut!(DIAG_CANAIS) as *mut u16;
        core::ptr::write_volatile(base.add(canal), valor);

        let base_min = addr_of_mut!(DIAG_CANAIS_MIN) as *mut u16;
        if valor < core::ptr::read_volatile(base_min.add(canal)) {
            core::ptr::write_volatile(base_min.add(canal), valor);
        }

        let base_max = addr_of_mut!(DIAG_CANAIS_MAX) as *mut u16;
        if valor > core::ptr::read_volatile(base_max.add(canal)) {
            core::ptr::write_volatile(base_max.add(canal), valor);
        }
    }
}

/// O canal que mais variou desde a partida, se algum variou o bastante.
///
/// É assim que se descobre em que canal o PA6 realmente está: aperte um
/// botão do painel e o divisor resistivo mexe a tensão **de um canal
/// só** — esse é o certo. Os demais ficam parados no seu valor de
/// repouso.
#[cfg(feature = "diag-botoes")]
pub fn canal_mais_ativo(variacao_minima: u16) -> Option<usize> {
    let mut melhor: Option<(usize, u16)> = None;

    for canal in 0..NUM_CANAIS {
        // SAFETY: leituras voláteis em índice dentro do vetor.
        let amplitude = unsafe {
            let min = core::ptr::read_volatile((addr_of_mut!(DIAG_CANAIS_MIN) as *mut u16).add(canal));
            let max = core::ptr::read_volatile((addr_of_mut!(DIAG_CANAIS_MAX) as *mut u16).add(canal));
            max.saturating_sub(min)
        };

        if amplitude < variacao_minima {
            continue;
        }
        if melhor.is_none_or(|(_, a)| amplitude > a) {
            melhor = Some((canal, amplitude));
        }
    }

    melhor.map(|(canal, _)| canal)
}

// ═════════════════════════════════════════════════════════════════════════════
// LOG CIRCULAR DAS LEITURAS (build de bancada)
// ═════════════════════════════════════════════════════════════════════════════

/// Quantas leituras o log guarda.
pub const TAM_LOG: usize = 64;

/// As ultimas leituras do canal dos botoes, em ordem circular.
///
/// Aperte um botao e despeje isto: a sequencia mostra o valor antes,
/// durante e depois do aperto. E o "log" que falta numa placa sem
/// UART e sem display.
#[no_mangle]
pub static mut DIAG_LOG: [u16; TAM_LOG] = [0; TAM_LOG];

/// Onde a proxima leitura vai entrar no log (0..TAM_LOG-1).
#[no_mangle]
pub static mut DIAG_LOG_POS: u16 = 0;

/// Maior leitura vista FORA da faixa de repouso.
///
/// Com o painel em repouso o valor fica na faixa baixa; qualquer coisa
/// acima dela e aperto de botao (ou ruido). Este e o numero que o
/// diagnostico pisca nos LEDs.
#[no_mangle]
pub static mut DIAG_PICO: u16 = 0;

/// Guarda uma leitura no log circular.
#[cfg(feature = "diag-botoes")]
pub fn registra_log(valor: u16) {
    // SAFETY: escritas volateis em `static mut` que so a tarefa de
    // diagnostico escreve, com indice sempre dentro do vetor.
    unsafe {
        let pos = core::ptr::read_volatile(addr_of_mut!(DIAG_LOG_POS)) as usize % TAM_LOG;
        let base = addr_of_mut!(DIAG_LOG) as *mut u16;
        core::ptr::write_volatile(base.add(pos), valor);
        core::ptr::write_volatile(addr_of_mut!(DIAG_LOG_POS), ((pos + 1) % TAM_LOG) as u16);

        if !crate::botoes::em_repouso(valor) {
            let pico = core::ptr::read_volatile(addr_of_mut!(DIAG_PICO));
            if valor > pico {
                core::ptr::write_volatile(addr_of_mut!(DIAG_PICO), valor);
            }
        }
    }
}

/// O maior valor visto fora da faixa de repouso.
#[cfg(feature = "diag-botoes")]
pub fn pico() -> u16 {
    // SAFETY: leitura volatil de um `static mut`.
    unsafe { core::ptr::read_volatile(addr_of_mut!(DIAG_PICO)) }
}

// ═════════════════════════════════════════════════════════════════════════════
// PATAMARES (build de bancada)
// ═════════════════════════════════════════════════════════════════════════════

/// Quantos patamares distintos o diagnostico guarda.
pub const NUM_PLATOS: usize = 6;

/// Valores que ficaram ESTAVEIS por algumas amostras seguidas.
///
/// Cada botao do painel produz um patamar proprio. Aperte os tres, um
/// por vez, e isto fica com os tres valores — e so copia-los para as
/// faixas de `botoes.rs`.
#[no_mangle]
pub static mut DIAG_PLATOS: [u16; NUM_PLATOS] = [0; NUM_PLATOS];

/// Quantos patamares distintos ja foram vistos.
#[no_mangle]
pub static mut DIAG_PLATOS_N: u16 = 0;

/// Diferenca minima para dois patamares serem considerados distintos.
#[cfg(feature = "diag-botoes")]
const DISTANCIA_PLATO: u16 = 30;

/// Guarda um patamar, se ainda nao houver um parecido.
#[cfg(feature = "diag-botoes")]
pub fn registra_plato(valor: u16) {
    // SAFETY: escritas volateis em `static mut` que so a tarefa de
    // diagnostico escreve, com indice conferido.
    unsafe {
        let base = addr_of_mut!(DIAG_PLATOS) as *mut u16;
        let n = core::ptr::read_volatile(addr_of_mut!(DIAG_PLATOS_N)) as usize;

        for i in 0..n.min(NUM_PLATOS) {
            let ja = core::ptr::read_volatile(base.add(i));
            if valor.abs_diff(ja) <= DISTANCIA_PLATO {
                return; // ja temos este patamar
            }
        }

        if n < NUM_PLATOS {
            core::ptr::write_volatile(base.add(n), valor);
            core::ptr::write_volatile(addr_of_mut!(DIAG_PLATOS_N), (n + 1) as u16);
        }
    }
}

/// Ultima leitura da referencia interna de tensao (canal 14).
///
/// E `VREFINT / VDDA`: sobe quando a fonte de 3,3 V cai. Comparando
/// esta leitura entre placas descobre-se se as fontes diferem, e
/// comparando com o patamar dos botoes descobre-se se o divisor
/// acompanha a fonte. Ver `mcu::adc::converte_vrefint`.
#[no_mangle]
pub static mut DIAG_VREFINT: u16 = 0;

/// Guarda a leitura da referencia interna.
pub fn registra_vrefint(valor: u16) {
    // SAFETY: escrita volatil num `static mut` que so a tarefa de
    // diagnostico escreve.
    unsafe { core::ptr::write_volatile(addr_of_mut!(DIAG_VREFINT), valor) }
}
