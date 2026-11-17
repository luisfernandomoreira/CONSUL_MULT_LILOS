//! Roda no PC os testes dos modulos de logica pura do firmware.
//!
//! `cargo test` NAO funciona neste projeto: o alvo padrao e
//! `thumbv6m-none-eabi`, que nao tem a crate `test`, e compilar para o
//! host falha porque o `build.rs` do lilos rejeita alvos que nao sao
//! Cortex-M. Entao os testes rodam por fora, com o rustc direto — o
//! mesmo padrao das outras ferramentas desta pasta.
//!
//! ```bash
//! rustc --test --edition 2021 -O ferramentas/testa_logica.rs -o target/testes.exe
//! ./target/testes.exe
//! ```
//!
//! Sao incluidos os cinco modulos que nao dependem de hardware:
//! `encoder`, `erro`, `botoes`, `modelo` e `tabelas`. Eles concentram o que foi
//! transcrito a mao do firmware C++, que e justamente o que mais vale
//! ter sob teste.

#![allow(dead_code)]

#[path = "../src/botoes.rs"]
mod botoes;
#[path = "../src/encoder.rs"]
mod encoder;
#[path = "../src/erro.rs"]
mod erro;
#[path = "../src/modelo.rs"]
mod modelo;
#[path = "../src/tabelas.rs"]
mod tabelas;

// ═════════════════════════════════════════════════════════════════════════════
// ENCODER
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod encoder_tests {
    use super::encoder;

    /// As 16 combinacoes de 4 trilhas tem de dar as 16 posicoes, sem
    /// repetir nenhuma. Uma posicao duplicada seria um programa
    /// inalcancavel e outro acionado por duas posicoes do disco.
    #[test]
    fn as_16_combinacoes_dao_16_posicoes_distintas() {
        let mut vistas = [false; 16];
        for padrao in 0u8..16 {
            let trilhas = [
                padrao & 0b1000 != 0,
                padrao & 0b0100 != 0,
                padrao & 0b0010 != 0,
                padrao & 0b0001 != 0,
            ];
            let pos = encoder::posicao(trilhas) as usize;
            assert!(pos < 16, "posicao {} fora de 0..15", pos);
            assert!(!vistas[pos], "posicao {} apareceu duas vezes", pos);
            vistas[pos] = true;
        }
        assert!(vistas.iter().all(|&v| v), "alguma posicao nunca aparece");
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// BOTOES — as faixas do ADC
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod botoes_tests {
    use super::botoes::{self, Botao};

    /// Valores medidos (Pausa) e inferidos do deslocamento de -31 na
    /// placa com o GD32. Se alguem mexer nas faixas, isto acusa.
    const MEDIDOS: [(u16, Botao); 3] = [
        (789, Botao::Pause),
        (851, Botao::MaisSecas),
        (931, Botao::EnxagueDuplo),
    ];

    #[test]
    fn repouso_inclui_zero() {
        // Esta placa le zero em repouso. Com o limite inferior em 7,
        // como no C++, o ferrolho nunca armava e nenhum botao
        // funcionava.
        assert!(botoes::em_repouso(0));
        assert!(botoes::em_repouso(30));
        assert!(botoes::em_repouso(60));
        assert!(!botoes::em_repouso(61));
    }

    #[test]
    fn cada_valor_medido_cai_na_sua_faixa() {
        for (valor, esperado) in MEDIDOS {
            assert_eq!(
                botoes::identifica(valor),
                Some(esperado),
                "valor {} nao caiu na faixa esperada",
                valor
            );
        }
    }

    #[test]
    fn valor_de_repouso_nao_e_botao() {
        assert_eq!(botoes::identifica(0), None);
        assert_eq!(botoes::identifica(30), None);
    }

    /// Nenhum valor pode cair em duas faixas ao mesmo tempo, e o
    /// repouso nao pode encostar em faixa de botao.
    ///
    /// Foi exatamente isso que causou o defeito do "Mais Secas
    /// desligava a maquina": a faixa de Pausa, alargada, alcancava a
    /// subida de tensao do Mais Secas.
    #[test]
    fn faixas_nao_se_sobrepoem() {
        for valor in 0u16..=1023 {
            let botao = botoes::identifica(valor);
            if botoes::em_repouso(valor) {
                assert_eq!(
                    botao, None,
                    "valor {} e repouso e botao ao mesmo tempo",
                    valor
                );
            }
        }
    }

    /// Entre as faixas tem de haver folga: a tensao sobe gradualmente
    /// quando o contato fecha, e a travessia precisa cair no vazio.
    #[test]
    fn ha_folga_entre_as_faixas_de_botao() {
        let mut transicoes = 0;
        let mut anterior = botoes::identifica(0);
        for valor in 1u16..=1023 {
            let atual = botoes::identifica(valor);
            if atual != anterior {
                // Toda troca tem de passar por None (a folga).
                assert!(
                    atual.is_none() || anterior.is_none(),
                    "faixas de {:?} e {:?} se tocam em {}",
                    anterior,
                    atual,
                    valor
                );
                transicoes += 1;
            }
            anterior = atual;
        }
        // Tres faixas: entra e sai de cada uma.
        assert_eq!(transicoes, 6, "esperava 3 faixas separadas por folga");
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// MODELO — a chave SW1..SW4
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod modelo_tests {
    use super::modelo;

    /// Tabela de `modeloMaquina.cpp`, bit a bit.
    const COMBINACOES: [([bool; 4], u8); 7] = [
        ([false, false, false, true], 1),
        ([false, false, true, false], 2),
        ([false, false, true, true], 3),
        ([false, true, false, false], 4),
        ([true, false, false, false], 5),
        ([true, true, true, true], 6),
        ([false, true, true, false], 7),
    ];

    #[test]
    fn as_sete_combinacoes_validas() {
        for (chave, esperado) in COMBINACOES {
            assert_eq!(modelo::do_chave(chave), esperado, "chave {:?}", chave);
            assert!(modelo::valido(esperado));
        }
    }

    #[test]
    fn qualquer_outra_combinacao_e_invalida() {
        let validas: [[bool; 4]; 7] = COMBINACOES.map(|(c, _)| c);
        for bits in 0u8..16 {
            let chave = [
                bits & 0b0001 != 0,
                bits & 0b0010 != 0,
                bits & 0b0100 != 0,
                bits & 0b1000 != 0,
            ];
            if validas.contains(&chave) {
                continue;
            }
            let m = modelo::do_chave(chave);
            assert_eq!(m, 8, "chave {:?} devia ser invalida", chave);
            assert!(!modelo::valido(m));
        }
    }

    #[test]
    fn so_modelos_5_e_6_usam_tampa_mecanica() {
        for m in 1u8..=8 {
            assert_eq!(
                modelo::usa_tampa_mecanica(m),
                m == 5 || m == 6,
                "modelo {}",
                m
            );
        }
    }

    #[test]
    fn modelo_7_e_o_auto_teste() {
        assert_eq!(modelo::AUTO_TESTE, 7);
        assert!(modelo::valido(modelo::AUTO_TESTE));
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TABELAS — os tempos de cada programa
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tabelas_tests {
    use super::tabelas;

    #[test]
    fn modelo_fora_de_1_6_cai_no_padrao_do_cpp() {
        // `determinarTempoDeAgitar` devolve 120 quando o modelo nao tem
        // tabela — e o que acontece nos modelos 7 e 8.
        for m in [0u8, 7, 8, 99] {
            assert_eq!(tabelas::tempo_agitar(m, 0), 120, "modelo {}", m);
        }
    }

    #[test]
    fn modelo_sem_tabela_nao_tem_molho() {
        // No C++ o array vem vazio e a etapa MOLHO sai sem fazer nada.
        for m in [0u8, 7, 8, 99] {
            assert!(tabelas::config_molho(m, 0).is_none(), "modelo {}", m);
        }
    }

    #[test]
    fn tipo_acima_do_limite_cai_no_ultimo_programa() {
        for m in 1u8..=6 {
            let ultimo = tabelas::config_molho(m, 15);
            assert_eq!(tabelas::config_molho(m, 99), ultimo, "modelo {}", m);
        }
    }

    #[test]
    fn programas_de_uma_etapa_so_nao_tem_molho() {
        // Posicoes 14 e 15 dos modelos 1, 2, 4 e 6 sao "so enxaguar" e
        // "so centrifugar": nao passam pelo molho.
        for m in [1u8, 2, 4, 6] {
            for tipo in [14u8, 15] {
                let cfg = tabelas::config_molho(m, tipo).unwrap();
                assert_eq!(cfg.ciclos, 0, "modelo {} tipo {}", m, tipo);
                assert_eq!(tabelas::tempo_agitar(m, tipo), 0);
            }
        }
    }

    #[test]
    fn ciclo_turbo_so_vale_dentro_dos_modelos_com_tabela() {
        for m in [0u8, 7, 8, 99] {
            for ciclo in 0u16..40 {
                assert!(!tabelas::ciclo_e_turbo(m, 0, ciclo), "modelo {}", m);
            }
        }
    }

    #[test]
    fn lavagem_economica_e_um_programa_por_modelo() {
        // Cada modelo tem exatamente um programa economico (tabela
        // `eco` do C++).
        let esperado = [(1u8, 10u8), (2, 10), (3, 3), (4, 10), (5, 11), (6, 10)];
        for (m, tipo_eco) in esperado {
            let quantos = (0u8..16)
                .filter(|&t| tabelas::reutiliza_agua(m, t))
                .collect::<Vec<_>>();
            assert_eq!(quantos, vec![tipo_eco], "modelo {}", m);
        }
    }

    #[test]
    fn modelo_sem_tabela_nao_reutiliza_agua() {
        for m in [0u8, 7, 8, 99] {
            for t in 0u8..16 {
                assert!(!tabelas::reutiliza_agua(m, t), "modelo {}", m);
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// ERRO — os codigos piscados
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod erro_tests {
    use super::erro::Erro;

    const TODOS: [Erro; 3] = [Erro::Nivel, Erro::Encher, Erro::Drenar];

    /// Dois erros com o mesmo numero de piscadas seriam
    /// indistinguiveis em campo — o tecnico conta as piscadas e nao tem
    /// como saber qual dos dois aconteceu.
    #[test]
    fn nenhum_erro_repete_o_numero_de_piscadas() {
        for (i, a) in TODOS.iter().enumerate() {
            for b in &TODOS[i + 1..] {
                assert_ne!(
                    a.piscadas(),
                    b.piscadas(),
                    "{:?} e {:?} piscam o mesmo numero",
                    a,
                    b
                );
            }
        }
    }

    /// A contagem comeca em 2: uma piscada so se confundiria com o
    /// pisca de tampa aberta, que e o LED de ON/OFF sozinho.
    #[test]
    fn contagem_comeca_em_dois() {
        for e in TODOS {
            assert!(e.piscadas() >= 2, "{:?} pisca {} vez", e, e.piscadas());
        }
    }

    /// Mais de umas poucas piscadas ninguem conta certo de longe.
    #[test]
    fn contagem_nao_passa_de_seis() {
        for e in TODOS {
            assert!(e.piscadas() <= 6, "{:?} pisca {} vezes", e, e.piscadas());
        }
    }

    /// Os numeros sao os mesmos das outras placas da linha.
    #[test]
    fn codigos_batem_com_as_outras_placas() {
        assert_eq!(Erro::Nivel.piscadas(), 2);
        assert_eq!(Erro::Encher.piscadas(), 3);
        assert_eq!(Erro::Drenar.piscadas(), 4);
    }

    #[test]
    fn todo_erro_tem_nome() {
        for e in TODOS {
            assert!(!e.nome().is_empty(), "{:?} sem nome", e);
        }
    }
}
