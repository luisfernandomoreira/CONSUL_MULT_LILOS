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

    /// Os tres valores MEDIDOS na placa com o GD32, pelo build
    /// `--features diag-botoes`. Se alguem mexer nas faixas, isto
    /// acusa.
    const MEDIDOS: [(u16, Botao); 3] = [
        (789, Botao::Pause),
        (849, Botao::MaisSecas),
        (931, Botao::EnxagueDuplo),
    ];

    #[test]
    fn repouso_inclui_zero() {
        // Esta placa le zero em repouso. Com o limite inferior em 7,
        // como no C++, o ferrolho nunca armava e nenhum botao
        // funcionava.
        assert!(botoes::em_repouso(0)); // valor medido na placa
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

// ═════════════════════════════════════════════════════════════════════════════
// BOTOES — normalizacao pela referencia interna
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod normaliza_tests {
    use super::botoes::{self, CORRECAO_MAXIMA_MILESIMOS, VREFINT_REFERENCIA};

    /// Sem valor de calibracao nao ha conta a fazer: a leitura passa
    /// intocada. E o estado em que o firmware sai de fabrica, ate
    /// alguem medir o VREFINT da placa de referencia.
    #[test]
    fn inerte_sem_calibracao() {
        if VREFINT_REFERENCIA != 0 {
            return; // ja calibrado; este teste nao se aplica
        }
        for ad in [0u16, 30, 789, 851, 931, 1023] {
            for vref in [0u16, 300, 372, 400] {
                assert_eq!(botoes::normaliza(ad, vref), ad, "ad={} vref={}", ad, vref);
            }
        }
    }

    /// VREFINT ausente (leitura falhou) nunca deve zerar nem distorcer
    /// a leitura do botao.
    #[test]
    fn inerte_sem_leitura_de_vrefint() {
        for ad in [0u16, 789, 1023] {
            assert_eq!(botoes::normaliza(ad, 0), ad);
        }
    }

    /// Com o VREFINT igual ao da calibracao, a correcao e 1:1.
    #[test]
    fn nao_mexe_quando_a_fonte_e_a_mesma() {
        if VREFINT_REFERENCIA == 0 {
            return;
        }
        for ad in [30u16, 789, 851, 931] {
            assert_eq!(botoes::normaliza(ad, VREFINT_REFERENCIA), ad, "ad={}", ad);
        }
    }

    /// O caso que mais preocupa: se o divisor for ratiometrico, a
    /// normalizacao esta ERRADA — a leitura ja estava certa. O limite
    /// de correcao existe para que, nesse caso, o erro introduzido
    /// fique DENTRO da faixa do botao.
    ///
    /// Simula uma fonte 5% diferente (bem acima da tolerancia de um
    /// regulador) e confere que todo botao continua sendo reconhecido.
    #[test]
    fn mesmo_errada_a_normalizacao_nao_estoura_a_faixa() {
        let referencia = if VREFINT_REFERENCIA == 0 { 372 } else { VREFINT_REFERENCIA };

        for (centro, esperado) in [
            (789u16, botoes::Botao::Pause),
            (849, botoes::Botao::MaisSecas),
            (931, botoes::Botao::EnxagueDuplo),
        ] {
            for desvio_pct in [-5i32, -3, -1, 0, 1, 3, 5] {
                // Divisor ratiometrico: a leitura NAO muda com a fonte.
                let ad = centro;
                // Mas o VREFINT muda, e e o que a normalizacao enxerga.
                let vref = ((referencia as i32) * (100 + desvio_pct) / 100) as u16;

                let corrigido = normaliza_com(ad, vref, referencia);
                let erro = (corrigido as i32 - ad as i32).abs();
                let limite = (ad as i32) * (CORRECAO_MAXIMA_MILESIMOS as i32) / 1000;
                assert!(
                    erro <= limite + 1,
                    "centro {} desvio {}%: erro {} passou do limite {}",
                    centro, desvio_pct, erro, limite
                );
                assert_eq!(
                    botoes::identifica(corrigido),
                    Some(esperado),
                    "centro {} desvio {}% deu {} e saiu da faixa",
                    centro, desvio_pct, corrigido
                );
            }
        }
    }

    /// A trava precisa valer nos dois sentidos, nao so para cima.
    #[test]
    fn a_trava_vale_nos_dois_sentidos() {
        let referencia = 372u16;
        for ad in [100u16, 789, 1000] {
            let limite = (ad as u32) * CORRECAO_MAXIMA_MILESIMOS / 1000;
            // VREFINT muito maior -> correcao para baixo
            let baixo = normaliza_com(ad, referencia * 4, referencia);
            // VREFINT muito menor -> correcao para cima
            let alto = normaliza_com(ad, referencia / 4, referencia);
            assert!(
                (ad as u32).saturating_sub(baixo as u32) <= limite,
                "ad={} caiu demais: {}", ad, baixo
            );
            assert!(
                (alto as u32).saturating_sub(ad as u32) <= limite,
                "ad={} subiu demais: {}", ad, alto
            );
        }
    }

    /// Nunca pode passar do fundo de escala de 10 bits.
    #[test]
    fn nunca_passa_de_1023() {
        let referencia = 372u16;
        for ad in [1000u16, 1020, 1023] {
            assert!(normaliza_com(ad, 1, referencia) <= 1023);
        }
    }

    /// A mesma conta de `botoes::normaliza`, com a referencia passada
    /// por fora — permite testar o comportamento calibrado sem mexer
    /// na constante do firmware.
    fn normaliza_com(ad: u16, vrefint: u16, referencia: u16) -> u16 {
        if referencia == 0 || vrefint == 0 {
            return ad;
        }
        let ad32 = ad as u32;
        let corrigido = ad32 * referencia as u32 / vrefint as u32;
        let limite = ad32 * CORRECAO_MAXIMA_MILESIMOS / 1_000;
        corrigido
            .clamp(ad32.saturating_sub(limite), ad32 + limite)
            .min(1023) as u16
    }
}
