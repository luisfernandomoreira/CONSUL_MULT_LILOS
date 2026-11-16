//! Confere as tabelas de `src/programa.rs` no PC, sem precisar de placa.
//!
//! Roda assim, na raiz do projeto:
//!
//! ```text
//! rustc -O ferramentas/conferir_programas.rs -o conferir && ./conferir
//! ```
//!
//! Não precisa de `cargo` nem de dependência nenhuma: o arquivo lê
//! `src/programa.rs` como módulo. Depois de mexer em qualquer ciclo de
//! lavagem, rode isto antes de gravar na placa.
//!
//! O que é verificado, em todos os programas, com e sem "enxágue extra":
//!
//!   1. nunca enche com o tanque já cheio;
//!   2. nunca agita nem deixa de molho com o tanque vazio;
//!   3. nunca centrifuga com o tanque cheio;
//!   4. todo `Centrifugar` tem um `Pulsos` antes (roupa distribuída);
//!   5. `DrenarExtra` e `BombaPos` só depois de `Drenar`;
//!   6. toda etapa de tempo fixo tem duração maior que zero;
//!   7. o ciclo termina numa etapa estável.

// A ferramenta usa só uma parte de `programa.rs`; o resto é do firmware.
#![allow(dead_code)]

#[path = "../src/programa.rs"]
mod programa;

use programa::{Acao, Programa};

const TODOS: [Programa; 5] = [
    Programa::Pesada,
    Programa::Normal,
    Programa::Rapido,
    Programa::Enxaguar,
    Programa::Centrifugar,
];

fn main() {
    let mut problemas = 0;

    for p in TODOS {
        for extra in [false, true] {
            mostra_ciclo(p, extra);
            let falhas = verifica(p, extra);
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

    println!();
    if problemas == 0 {
        println!("Tudo certo: {} ciclos conferidos.", TODOS.len() * 2);
    } else {
        println!("{problemas} problema(s) encontrado(s).");
        std::process::exit(1);
    }
}

/// Imprime a sequência de etapas e o tempo fixo total do ciclo.
fn mostra_ciclo(p: Programa, extra: bool) {
    let etapas: Vec<_> = p
        .etapas()
        .iter()
        .filter(|et| !et.so_enxague_extra || extra)
        .collect();

    let fixo: u32 = etapas.iter().map(|et| et.segundos as u32).sum();

    println!(
        "\n=== {:?} — enxágue extra {} — {} etapas, {} min de tempo fixo",
        p,
        if extra { "LIGADO" } else { "desligado" },
        etapas.len(),
        fixo / 60
    );

    let linha: Vec<String> = etapas
        .iter()
        .map(|et| {
            if et.segundos == 0 {
                format!("{:?}(sensor)", et.acao)
            } else {
                format!("{:?}({}s)", et.acao, et.segundos)
            }
        })
        .collect();
    println!("  {}", linha.join(" → "));
}

/// Simula o ciclo acompanhando se o tanque está cheio e se a roupa já foi
/// distribuída, e devolve a lista de violações encontradas.
fn verifica(p: Programa, extra: bool) -> Vec<String> {
    let mut falhas = Vec::new();
    let mut cheio = false;
    let mut distribuiu = false;
    let mut ultima = None;

    for (i, et) in p.etapas().iter().enumerate() {
        if et.so_enxague_extra && !extra {
            continue;
        }
        let a = et.acao;

        match a {
            Acao::EncherSabao | Acao::EncherEnxague | Acao::EncherAmaciante => {
                if cheio {
                    falhas.push(format!("etapa {i}: {a:?} com o tanque já cheio"));
                }
                cheio = true;
                distribuiu = false;
            }
            Acao::Agitar | Acao::Molho | Acao::MolhoEnxague => {
                if !cheio {
                    falhas.push(format!("etapa {i}: {a:?} com o tanque vazio"));
                }
                if et.segundos == 0 {
                    falhas.push(format!("etapa {i}: {a:?} sem duração"));
                }
            }
            Acao::Drenar => {
                cheio = false;
                distribuiu = false;
            }
            Acao::DrenarExtra | Acao::BombaPos => {
                if cheio {
                    falhas.push(format!("etapa {i}: {a:?} antes de Drenar"));
                }
                if et.segundos == 0 {
                    falhas.push(format!("etapa {i}: {a:?} sem duração"));
                }
            }
            Acao::PausaFinal => {
                if cheio || et.segundos == 0 {
                    falhas.push(format!("etapa {i}: pausa final inválida"));
                }
            }
            Acao::Pulsos => {
                if cheio {
                    falhas.push(format!("etapa {i}: Pulsos com o tanque cheio"));
                }
                distribuiu = true;
            }
            Acao::Centrifugar => {
                if cheio {
                    falhas.push(format!("etapa {i}: Centrifugar com o tanque cheio"));
                }
                if !distribuiu {
                    falhas.push(format!("etapa {i}: Centrifugar sem Pulsos antes"));
                }
                if et.segundos == 0 {
                    falhas.push(format!("etapa {i}: Centrifugar sem duração"));
                }
            }
        }
        ultima = Some(a);
    }

    match ultima {
        // BombaPos: tanque vazio. Molho: fim do programa Enxaguar, que por
        // projeto deixa a água para o Centrifugar que o usuário escolher.
        Some(Acao::BombaPos | Acao::Molho | Acao::MolhoEnxague | Acao::PausaFinal) => {}
        Some(outra) => falhas.push(format!("o ciclo termina em {outra:?}")),
        None => falhas.push("programa sem nenhuma etapa".to_string()),
    }

    falhas
}
