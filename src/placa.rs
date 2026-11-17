//! =============================================================
//! Placa CONSUL MULT — mapa de pinos e agrupamentos de hardware
//!
//! É AQUI (e só aqui) que se mexe quando a placa muda. O resto do
//! firmware fala em nomes — `s.bomba(true)`, `e.tampa_aberta()` — e
//! nunca em "PA9" ou "PB2".
//!
//! O mapa abaixo é o do firmware C++ de fábrica
//! (`controle_hardware.hpp` + `CONSUL_MULT_V1.ioc`). O GD32C231K8T6
//! (LQFP32) é pino a pino igual ao STM32C031K6T6 (LQFP32), então os
//! nomes de pino são os mesmos.
//!
//! ─── Lógica elétrica ─────────────────────────────────────────
//!   LEDs e triacs     : INVERTIDOS → nível BAIXO liga, ALTO desliga
//!                       (no C++, ControlarPino(.., true) escreve no
//!                        BRR, ou seja, leva o pino a nível baixo)
//!   Chave SW1..SW4    : pull-up interno
//!   Encoder           : pull-down interno
//!   Pressostato (PA5) : sem pull; nível BAIXO = cheio
//!   Tampa (PA4)       : pull-up; nível ALTO = ABERTA
//!   Tampa mecânica    : PA7, sem pull; nível ALTO = ABERTA
//!   Botões do painel  : PA6, analógico (divisor resistivo, ver `botoes`)
//!
//! ─── Dois pinos com dois nomes ───────────────────────────────
//! O firmware C++ dá dois nomes a dois pinos, e o porte mantém isso:
//!
//!   PA9 → `motor_c` e `motor_pulse`   (centrifugação / pulso)
//!   PA3 → `atuador` e `valvula_amaciante`
//!
//! Não é erro de digitação: é o mesmo triac acionado por dois
//! caminhos lógicos do ciclo. Mexer num mexe no outro.
//! =============================================================

use crate::mcu::clock;
use crate::mcu::gpio::{Analogico, Entrada, Pino, Porta, Pull, Saida};
use crate::mcu::pac::Rcu;

// ─── Níveis de uma saída invertida ───────────────────────────────────────────
const DESLIGADO: bool = true; // nível alto
const LIGADO: bool = false; // nível baixo

#[inline(always)]
fn nivel(ligar: bool) -> bool {
    if ligar {
        LIGADO
    } else {
        DESLIGADO
    }
}

// ─── Mapa de pinos ───────────────────────────────────────────────────────────
// Trocou a placa de lugar? Mude só estas constantes.
mod pino {
    use super::{Pino, Porta};

    // Saídas de potência (triacs)
    pub const BOMBA: Pino = Pino::novo(Porta::B, 2);
    /// Também é a válvula de amaciante — mesmo triac.
    pub const ATUADOR: Pino = Pino::novo(Porta::A, 3);
    /// Também é o pulso de motor (`motor_pulse`) — mesmo triac.
    pub const MOTOR_C: Pino = Pino::novo(Porta::A, 9);
    pub const MOTOR_A: Pino = Pino::novo(Porta::C, 6);
    pub const VALVULA_SABAO: Pino = Pino::novo(Porta::A, 2);

    // LEDs do painel
    pub const LED_PAUSE: Pino = Pino::novo(Porta::A, 1); // ON/OFF
    pub const LED_MAIS_SECAS: Pino = Pino::novo(Porta::C, 14);
    pub const LED_ENXAGUE_DUPLO: Pino = Pino::novo(Porta::B, 9);

    // Sensores
    pub const PRESSOSTATO: Pino = Pino::novo(Porta::A, 5);
    pub const TAMPA: Pino = Pino::novo(Porta::A, 4);
    pub const TAMPA_MECANICA: Pino = Pino::novo(Porta::A, 7);

    // Chave de modelo da lavadora
    pub const SW1: Pino = Pino::novo(Porta::A, 12);
    pub const SW2: Pino = Pino::novo(Porta::A, 11);
    pub const SW3: Pino = Pino::novo(Porta::B, 8);
    pub const SW4: Pino = Pino::novo(Porta::B, 7);

    // Encoder de programa (4 trilhas). Os nomes 1/3/4/5 vêm do C++.
    pub const ENC_1: Pino = Pino::novo(Porta::B, 4);
    pub const ENC_3: Pino = Pino::novo(Porta::B, 3);
    pub const ENC_4: Pino = Pino::novo(Porta::B, 5);
    pub const ENC_5: Pino = Pino::novo(Porta::B, 6);

    // Botões do painel, por divisor resistivo
    pub const BOTOES_AD: Pino = Pino::novo(Porta::A, 6);
}

// ═════════════════════════════════════════════════════════════════════════════
// SAÍDAS — triacs e LEDs
// ═════════════════════════════════════════════════════════════════════════════

pub struct Saidas {
    bomba: Saida,
    atuador: Saida,
    motor_c: Saida,
    motor_a: Saida,
    valvula_sabao: Saida,
    led_pause: Saida,
    led_mais_secas: Saida,
    led_enxague_duplo: Saida,
}

impl Saidas {
    // ─── Triacs ──────────────────────────────────────────────────────────────

    pub fn bomba(&self, ligar: bool) {
        self.bomba.escreve(nivel(ligar));
    }

    /// Atuador da transmissão. **Mesmo pino da válvula de amaciante.**
    pub fn atuador(&self, ligar: bool) {
        self.atuador.escreve(nivel(ligar));
    }

    /// Válvula de amaciante. **Mesmo pino do atuador.**
    pub fn valvula_amaciante(&self, abrir: bool) {
        self.atuador.escreve(nivel(abrir));
    }

    pub fn valvula_sabao(&self, abrir: bool) {
        self.valvula_sabao.escreve(nivel(abrir));
    }

    /// Motor no sentido A. Desliga o sentido C antes.
    ///
    /// ─── A trava de sentido ──────────────────────────────────────
    ///
    /// A e C sao os dois sentidos do MESMO motor. Ligar os dois ao
    /// mesmo tempo e uma reversao com os dois triacs conduzindo, e e
    /// por isso que o desligamento do outro sentido mora **aqui**, e
    /// nao em quem chama.
    ///
    /// Nao e paranoia: `motor_pulse` e chamado direto pelas fases de
    /// drenagem e de centrifugacao, longe do `agita`, que tem a sua
    /// propria troca de sentido. Hoje nenhum caminho chega nessas
    /// fases com o sentido A ligado — mas isso depende de cada etapa
    /// lembrar de cortar antes. Com a trava aqui, "os dois ligados"
    /// deixa de ser representavel.
    pub fn motor_a(&self, ligar: bool) {
        if ligar {
            self.motor_c.escreve(DESLIGADO);
        }
        self.motor_a.escreve(nivel(ligar));
    }

    /// Motor em centrifugação. Desliga o sentido A antes.
    /// **Mesmo pino do `motor_pulse`.**
    pub fn motor_c(&self, ligar: bool) {
        if ligar {
            self.motor_a.escreve(DESLIGADO);
        }
        self.motor_c.escreve(nivel(ligar));
    }

    /// Pulso de motor usado na drenagem e na centrifugação.
    /// **Mesmo pino do `motor_c`**, e com a mesma trava de sentido.
    pub fn motor_pulse(&self, ligar: bool) {
        self.motor_c(ligar);
    }

    /// Desliga todos os triacs. Chamada no boot, ao abrir a tampa, ao
    /// desligar e em qualquer condição de erro.
    ///
    /// A válvula de amaciante NÃO aparece aqui porque divide o pino com
    /// o atuador, que já é desligado na linha acima — é exatamente o
    /// que o `desligaTodosTriacs()` do C++ faz.
    pub fn desliga_todos_triacs(&self) {
        self.motor_a(false);
        self.motor_c(false);
        self.bomba(false);
        self.atuador(false);
        self.valvula_sabao(false);
    }

    // ─── LEDs ────────────────────────────────────────────────────────────────

    pub fn led_pause(&self, aceso: bool) {
        self.led_pause.escreve(nivel(aceso));
    }

    pub fn led_mais_secas(&self, aceso: bool) {
        self.led_mais_secas.escreve(nivel(aceso));
    }

    pub fn led_enxague_duplo(&self, aceso: bool) {
        self.led_enxague_duplo.escreve(nivel(aceso));
    }

    /// Inverte o LED de ON/OFF — é o pisca de tampa aberta.
    pub fn pisca_led(&self) {
        self.led_pause.inverte();
    }

    /// Inverte os três LEDs ao mesmo tempo — animação de boot e de erro.
    pub fn pisca_led_interface(&self) {
        self.led_pause.inverte();
        self.led_mais_secas.inverte();
        self.led_enxague_duplo.inverte();
    }

    pub fn apaga_leds(&self) {
        self.led_pause(false);
        self.led_mais_secas(false);
        self.led_enxague_duplo(false);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// ENTRADAS — sensores, chave de modelo e encoder
// ═════════════════════════════════════════════════════════════════════════════

pub struct Entradas {
    pressostato: Entrada,
    tampa: Entrada,
    tampa_mecanica: Entrada,
    sw1: Entrada,
    sw2: Entrada,
    sw3: Entrada,
    sw4: Entrada,
    enc_1: Entrada,
    enc_3: Entrada,
    enc_4: Entrada,
    enc_5: Entrada,
    _botoes_ad: Analogico,
}

impl Entradas {
    /// Leitura crua do pressostato. Nível ALTO = ainda não cheio.
    ///
    /// Quem usa isto é o filtro de 10 amostras em `tarefas::amostragem`,
    /// que é também quem inverte o sinal para virar "nível de água
    /// atingido" — igual ao `FiltroDebouncePressostato` do C++.
    pub fn pressostato(&self) -> bool {
        self.pressostato.esta_alta()
    }

    /// Tampa eletrônica (modelos 1–4 e 7). ALTO = aberta.
    pub fn tampa_aberta(&self) -> bool {
        self.tampa.esta_alta()
    }

    /// Tampa mecânica (modelos 5 e 6). ALTO = aberta.
    pub fn tampa_mecanica_aberta(&self) -> bool {
        self.tampa_mecanica.esta_alta()
    }

    /// Os quatro bits da chave de modelo, na ordem SW1..SW4.
    pub fn chave_modelo(&self) -> [bool; 4] {
        [
            self.sw1.esta_alta(),
            self.sw2.esta_alta(),
            self.sw3.esta_alta(),
            self.sw4.esta_alta(),
        ]
    }

    /// As quatro trilhas do encoder, na ordem ENC_1, ENC_3, ENC_4, ENC_5
    /// — a mesma ordem em que o C++ as passa para `setterEncoder`.
    pub fn trilhas_encoder(&self) -> [bool; 4] {
        [
            self.enc_1.esta_alta(),
            self.enc_3.esta_alta(),
            self.enc_4.esta_alta(),
            self.enc_5.esta_alta(),
        ]
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// INICIALIZAÇÃO
// ═════════════════════════════════════════════════════════════════════════════

/// Liga o clock das portas, configura todos os pinos e devolve os dois
/// agrupamentos. Os triacs e os LEDs já saem daqui desligados.
pub fn inicializa(rcu: &Rcu) -> (Saidas, Entradas) {
    clock::habilita_gpio(rcu);

    let saidas = Saidas {
        bomba: Saida::nova(pino::BOMBA, DESLIGADO),
        atuador: Saida::nova(pino::ATUADOR, DESLIGADO),
        motor_c: Saida::nova(pino::MOTOR_C, DESLIGADO),
        motor_a: Saida::nova(pino::MOTOR_A, DESLIGADO),
        valvula_sabao: Saida::nova(pino::VALVULA_SABAO, DESLIGADO),
        led_pause: Saida::nova(pino::LED_PAUSE, DESLIGADO),
        led_mais_secas: Saida::nova(pino::LED_MAIS_SECAS, DESLIGADO),
        led_enxague_duplo: Saida::nova(pino::LED_ENXAGUE_DUPLO, DESLIGADO),
    };
    saidas.desliga_todos_triacs();

    let entradas = Entradas {
        pressostato: Entrada::nova(pino::PRESSOSTATO, Pull::Nenhum),
        tampa: Entrada::nova(pino::TAMPA, Pull::Cima),
        tampa_mecanica: Entrada::nova(pino::TAMPA_MECANICA, Pull::Nenhum),
        sw1: Entrada::nova(pino::SW1, Pull::Cima),
        sw2: Entrada::nova(pino::SW2, Pull::Cima),
        sw3: Entrada::nova(pino::SW3, Pull::Cima),
        sw4: Entrada::nova(pino::SW4, Pull::Cima),
        enc_1: Entrada::nova(pino::ENC_1, Pull::Baixo),
        enc_3: Entrada::nova(pino::ENC_3, Pull::Baixo),
        enc_4: Entrada::nova(pino::ENC_4, Pull::Baixo),
        enc_5: Entrada::nova(pino::ENC_5, Pull::Baixo),
        _botoes_ad: Analogico::novo(pino::BOTOES_AD),
    };

    (saidas, entradas)
}
