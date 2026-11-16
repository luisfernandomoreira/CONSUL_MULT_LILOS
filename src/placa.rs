//! =============================================================
//! Placa CONSUL MULT — mapa de pinos e agrupamentos de hardware
//!
//! É AQUI (e só aqui) que se mexe quando a placa muda.
//! O resto do firmware fala em nomes — `painel.pesada`, `at.motor_a`,
//! `entradas.tampa` — e nunca em "PA7" ou "PB2".
//!
//! O GD32C231K8T6 (LQFP32) é pino a pino idêntico ao STM32C031K6T6
//! (LQFP32) usado na versão anterior, então o mapa abaixo é exatamente
//! o mesmo do firmware Embassy original.
//!
//! ─── Lógica elétrica ─────────────────────────────────────────
//!   LEDs e triacs  : INVERTIDOS → nível BAIXO liga, ALTO desliga
//!   Botões         : pull-up interno → nível BAIXO = pressionado
//!   Sensores nível : pull-down interno → nível ALTO = nível atingido
//!   Tampa (PB1)    : nível ALTO = tampa ABERTA
//! =============================================================

use crate::mcu::clock;
use crate::mcu::gpio::{Entrada, Pino, Porta, Pull, Saida};
use crate::mcu::pac::Rcu;
use crate::programa::Programa;

// ─── Nível que DESLIGA uma saída invertida ───────────────────────────────────
const DESLIGADO: bool = true; // nível alto
const LIGADO: bool = false; // nível baixo

// ─── Mapa de pinos da placa ──────────────────────────────────────────────────
// Trocou a placa de lugar? Mude só estas constantes.
mod pino {
    use super::{Pino, Porta};

    // LEDs dos 5 programas
    pub const LED_PESADA: Pino = Pino::novo(Porta::A, 7);
    pub const LED_NORMAL: Pino = Pino::novo(Porta::A, 6);
    pub const LED_RAPIDO: Pino = Pino::novo(Porta::A, 4);
    pub const LED_ENXAGUAR: Pino = Pino::novo(Porta::A, 1);
    pub const LED_CENTRIFUGAR: Pino = Pino::novo(Porta::A, 0);

    // LEDs das opções
    pub const LED_ENXAGUE_EXTRA: Pino = Pino::novo(Porta::A, 11);
    pub const LED_TURBO: Pino = Pino::novo(Porta::A, 15);
    pub const LED_AMACIANTE: Pino = Pino::novo(Porta::C, 6);
    pub const LED_NIVEL_MEDIO: Pino = Pino::novo(Porta::A, 3);
    pub const LED_NIVEL_ALTO: Pino = Pino::novo(Porta::A, 2);

    // Triacs da placa de potência
    pub const MOTOR_C: Pino = Pino::novo(Porta::B, 2); // centrifugação
    pub const MOTOR_A: Pino = Pino::novo(Porta::A, 8); // agitação
    pub const ELETROBOMBA: Pino = Pino::novo(Porta::A, 9); // drenagem
    pub const VALVULA_SABAO: Pino = Pino::novo(Porta::B, 6);
    pub const VALVULA_AMACIANTE: Pino = Pino::novo(Porta::B, 0);

    // Botões do painel
    pub const BT_ON_OFF: Pino = Pino::novo(Porta::C, 14);
    pub const BT_SELETOR: Pino = Pino::novo(Porta::A, 5);
    pub const BT_NIVEL: Pino = Pino::novo(Porta::C, 15);
    pub const BT_ENXAGUE_EXTRA: Pino = Pino::novo(Porta::A, 12);
    pub const BT_TURBO: Pino = Pino::novo(Porta::B, 9);
    pub const BT_AMACIANTE: Pino = Pino::novo(Porta::A, 10);

    // Sensores
    pub const SENSOR_NIVEL_MEDIO: Pino = Pino::novo(Porta::B, 7);
    pub const SENSOR_NIVEL_ALTO: Pino = Pino::novo(Porta::B, 8);
    pub const TAMPA: Pino = Pino::novo(Porta::B, 1);
}

// ═════════════════════════════════════════════════════════════════════════════
// PAINEL — os 10 LEDs
// ═════════════════════════════════════════════════════════════════════════════

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
    /// Apaga os 5 LEDs de programa.
    pub fn apaga_programas(&self) {
        self.pesada.escreve(DESLIGADO);
        self.normal.escreve(DESLIGADO);
        self.rapido.escreve(DESLIGADO);
        self.enxaguar.escreve(DESLIGADO);
        self.centrifugar.escreve(DESLIGADO);
    }

    /// Acende apenas o LED do programa indicado; apaga os outros.
    pub fn mostra_programa(&self, p: Programa) {
        self.apaga_programas();
        self.led_do_programa(p).escreve(LIGADO);
    }

    fn led_do_programa(&self, p: Programa) -> &Saida {
        match p {
            Programa::Pesada => &self.pesada,
            Programa::Normal => &self.normal,
            Programa::Rapido => &self.rapido,
            Programa::Enxaguar => &self.enxaguar,
            Programa::Centrifugar => &self.centrifugar,
        }
    }

    /// Reflete as opções escolhidas nos LEDs correspondentes.
    ///
    /// Com a máquina desligada todos ficam apagados.
    pub fn mostra_opcoes(&self, o: &crate::opcoes::Opcoes, maquina_ligada: bool) {
        let ligado = |v: bool| {
            if maquina_ligada && v {
                LIGADO
            } else {
                DESLIGADO
            }
        };
        self.enxague_extra.escreve(ligado(o.enxague_extra));
        self.turbo.escreve(ligado(o.turbo));
        self.amaciante.escreve(ligado(o.amaciante));
        self.nivel_medio.escreve(ligado(!o.nivel_alto));
        self.nivel_alto.escreve(ligado(o.nivel_alto));
    }

    /// Acende os 10 LEDs do painel.
    pub fn acende_tudo(&self) {
        for led in self.todos() {
            led.escreve(LIGADO);
        }
    }

    /// Apaga tudo — painel completamente escuro.
    pub fn apaga_tudo(&self) {
        self.apaga_programas();
        self.enxague_extra.escreve(DESLIGADO);
        self.turbo.escreve(DESLIGADO);
        self.amaciante.escreve(DESLIGADO);
        self.nivel_medio.escreve(DESLIGADO);
        self.nivel_alto.escreve(DESLIGADO);
    }

    /// Lista de todos os LEDs — usada pela animação de inicialização.
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
}

// ═════════════════════════════════════════════════════════════════════════════
// ATUADORES — os 5 triacs
// ═════════════════════════════════════════════════════════════════════════════

pub struct Atuadores {
    pub motor_c: Saida,
    pub motor_a: Saida,
    pub eletrobomba: Saida,
    pub valvula_sabao: Saida,
    pub valvula_amaciante: Saida,
}

impl Atuadores {
    /// Desliga TODOS os triacs. Chamada no boot, ao abrir a tampa,
    /// ao desligar e em qualquer condição de erro.
    pub fn desliga_todos(&self) {
        self.motor_c.escreve(DESLIGADO);
        self.motor_a.escreve(DESLIGADO);
        self.eletrobomba.escreve(DESLIGADO);
        self.valvula_sabao.escreve(DESLIGADO);
        self.valvula_amaciante.escreve(DESLIGADO);
    }

    pub fn motor_a(&self, ligar: bool) {
        self.motor_a.escreve(if ligar { LIGADO } else { DESLIGADO });
    }

    pub fn motor_c(&self, ligar: bool) {
        self.motor_c.escreve(if ligar { LIGADO } else { DESLIGADO });
    }

    pub fn eletrobomba(&self, ligar: bool) {
        self.eletrobomba
            .escreve(if ligar { LIGADO } else { DESLIGADO });
    }

    pub fn valvula_sabao(&self, abrir: bool) {
        self.valvula_sabao
            .escreve(if abrir { LIGADO } else { DESLIGADO });
    }

    pub fn valvula_amaciante(&self, abrir: bool) {
        self.valvula_amaciante
            .escreve(if abrir { LIGADO } else { DESLIGADO });
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// ENTRADAS — botões e sensores
// ═════════════════════════════════════════════════════════════════════════════

pub struct Entradas {
    pub bt_on_off: Entrada,
    pub bt_seletor: Entrada,
    pub bt_nivel: Entrada,
    pub bt_enxague_extra: Entrada,
    pub bt_turbo: Entrada,
    pub bt_amaciante: Entrada,
    pub sensor_nivel_medio: Entrada,
    pub sensor_nivel_alto: Entrada,
    pub tampa: Entrada,
}

impl Entradas {
    /// Botão pressionado = nível BAIXO (pull-up interno).
    pub fn botao_pressionado(&self, b: crate::evento::Botao) -> bool {
        use crate::evento::Botao::*;
        match b {
            OnOff => self.bt_on_off.esta_baixa(),
            Seletor => self.bt_seletor.esta_baixa(),
            Nivel => self.bt_nivel.esta_baixa(),
            EnxagueExtra => self.bt_enxague_extra.esta_baixa(),
            Turbo => self.bt_turbo.esta_baixa(),
            Amaciante => self.bt_amaciante.esta_baixa(),
        }
    }

    /// Sensor de nível médio atingido (nível ALTO no pino).
    pub fn nivel_medio_atingido(&self) -> bool {
        self.sensor_nivel_medio.esta_alta()
    }

    /// Sensor de nível alto atingido.
    pub fn nivel_alto_atingido(&self) -> bool {
        self.sensor_nivel_alto.esta_alta()
    }

    /// Tampa aberta = nível ALTO no pino.
    pub fn tampa_aberta(&self) -> bool {
        self.tampa.esta_alta()
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// INICIALIZAÇÃO
// ═════════════════════════════════════════════════════════════════════════════

/// Liga o clock das portas, configura todos os pinos e devolve os três
/// agrupamentos. Os triacs já saem daqui desligados.
pub fn inicializa(rcu: &Rcu) -> (Painel, Atuadores, Entradas) {
    clock::habilita_gpio(rcu);

    let painel = Painel {
        pesada: Saida::nova(pino::LED_PESADA, DESLIGADO),
        normal: Saida::nova(pino::LED_NORMAL, DESLIGADO),
        rapido: Saida::nova(pino::LED_RAPIDO, DESLIGADO),
        enxaguar: Saida::nova(pino::LED_ENXAGUAR, DESLIGADO),
        centrifugar: Saida::nova(pino::LED_CENTRIFUGAR, DESLIGADO),
        enxague_extra: Saida::nova(pino::LED_ENXAGUE_EXTRA, DESLIGADO),
        turbo: Saida::nova(pino::LED_TURBO, DESLIGADO),
        amaciante: Saida::nova(pino::LED_AMACIANTE, DESLIGADO),
        nivel_medio: Saida::nova(pino::LED_NIVEL_MEDIO, DESLIGADO),
        nivel_alto: Saida::nova(pino::LED_NIVEL_ALTO, DESLIGADO),
    };

    let atuadores = Atuadores {
        motor_c: Saida::nova(pino::MOTOR_C, DESLIGADO),
        motor_a: Saida::nova(pino::MOTOR_A, DESLIGADO),
        eletrobomba: Saida::nova(pino::ELETROBOMBA, DESLIGADO),
        valvula_sabao: Saida::nova(pino::VALVULA_SABAO, DESLIGADO),
        valvula_amaciante: Saida::nova(pino::VALVULA_AMACIANTE, DESLIGADO),
    };
    atuadores.desliga_todos();

    let entradas = Entradas {
        bt_on_off: Entrada::nova(pino::BT_ON_OFF, Pull::Cima),
        bt_seletor: Entrada::nova(pino::BT_SELETOR, Pull::Cima),
        bt_nivel: Entrada::nova(pino::BT_NIVEL, Pull::Cima),
        bt_enxague_extra: Entrada::nova(pino::BT_ENXAGUE_EXTRA, Pull::Cima),
        bt_turbo: Entrada::nova(pino::BT_TURBO, Pull::Cima),
        bt_amaciante: Entrada::nova(pino::BT_AMACIANTE, Pull::Cima),
        sensor_nivel_medio: Entrada::nova(pino::SENSOR_NIVEL_MEDIO, Pull::Baixo),
        sensor_nivel_alto: Entrada::nova(pino::SENSOR_NIVEL_ALTO, Pull::Baixo),
        tampa: Entrada::nova(pino::TAMPA, Pull::Nenhum),
    };

    (painel, atuadores, entradas)
}
