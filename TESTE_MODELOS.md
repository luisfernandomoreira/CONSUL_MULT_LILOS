# Folha de teste dos modelos — na máquina

Complemento do [ROTEIRO_BANCADA.md](ROTEIRO_BANCADA.md). O roteiro testa
os periféricos uma vez; esta folha testa **o que muda de modelo para
modelo**, que é o que a giga de teste não pega.

O modelo é relido a **cada etapa** (`obter_modelo_atual`), não só no
boot. Então mudar a chave com o ciclo rodando tem efeito — é teste, não
descuido.

> **Segurança.** Mangueira em balde ou ralo. Não burle o sensor de
> tampa: é ele que corta os triacs.

---

## A chave SW1..SW4

| SW1 SW2 SW3 SW4 | Modelo | Lavadoras |
|---|---|---|
| 0 1 1 0 | 7 | auto-teste de fábrica |
| 0 0 0 1 | 1 | CWH12AB V1 e V2 |
| 0 0 1 0 | 2 | CWH12AB V3, V4 e V5 |
| 0 0 1 1 | 3 | CWE10AB / CW11AB / CWG12AB |
| 0 1 0 0 | 4 | CWS11AB |
| 1 0 0 0 | 5 | CWE08AB / CWE09AB |
| 1 1 1 1 | 6 | CWH12BB / CWB09BB |
| qualquer outra | 8 | inválido — 3 LEDs a 700 ms, triacs off |

---

## Os testes que distinguem um modelo do outro

Se um destes falhar, o modelo está sendo lido ou tratado errado —
mesmo que o ciclo "pareça" rodar.

| Teste | O que esperar |
|---|---|
| **Centrifugação sem "mais secas"** | 5 min nos modelos 1, 2, 4 e 6 · 6 min nos modelos 3 e 5 |
| **Centrifugação com "mais secas"** | 10 min nos modelos 1, 2, 4 e 6 · **12 min** no modelo 3 (não corta) · **6 min** no modelo 5 (ignora o botão) |
| **Botão "mais secas" no modelo 5** | Não mexe na centrifugação: comanda o **enxágue duplo** |
| **Tampa aberta e fechada** | Modelos 1–5 e 7: retoma sozinha 2 s depois de fechar · **Modelo 6: fica em pausa e só retoma no ON/OFF** |
| **Girar o seletor com o ciclo rodando** | Modelos ≠ 6: espera 2 s e começa o programa novo · Modelo 6: entra em pausa |
| **Sensor de tampa** | Modelos 1–4 e 7 leem o **eletrônico (PA4)** · Modelos **5 e 6** leem o **mecânico (PA7)** |
| **Chave para combinação inválida no meio do ciclo** | Cai em erro de modelo na etapa seguinte: LEDs a 700 ms, triacs off |

---

## Posições de uma etapa só

Cada modelo põe "só enxaguar" e "só centrifugar" em posições
diferentes do encoder. Errar isto é o tipo de bug que passa batido.

| Modelo | Só enxaguar | Só centrifugar | Recomeça se o ciclo já terminou? |
|---|---|---|---|
| 1, 2, 4, 6 | 14 | 15 | não |
| 3 | 7 | 8 | **sim**, no "só enxaguar" |
| 5 | **15** | **0** | **sim**, no "só enxaguar" |

- [ ] Em cada modelo, a posição de "só enxaguar" vai direto ao enxágue (não enche)
- [ ] Em cada modelo, a posição de "só centrifugar" vai direto à centrifugação
- [ ] Modelos 3 e 5: "só enxaguar" recomeça com o ciclo já terminado
- [ ] Modelos 1, 2, 4 e 6: nessa mesma condição, **não** recomeça

---

## O programa mais longo de cada modelo

Serve para cronometrar a etapa AGITAR e confirmar que a tabela certa
está em uso. É o jeito mais rápido de separar modelos parecidos.

| Modelo | Posição | Tempo de AGITAR |
|---|---|---|
| 1 | 1 | 400 s |
| 2 | 1 | 400 s |
| 3 | 2 | 480 s |
| 4 | 9 | 480 s |
| 5 | 10 | 480 s |
| 6 | 1 | 400 s |

**Para separar 1, 2 e 6**, que compartilham quase toda a tabela, use a
**posição 11** — é o único tempo em que os três divergem:

| Modelo | Posição 11 |
|---|---|
| 1 | 125 s |
| 2 | 110 s |
| 6 | 120 s |

E o modelo 6 tem a posição 0 em **320 s**, contra 340 s nos modelos 1 e 2.

---

## Por modelo

### Modelo 7 — auto-teste (comece por aqui)
- [ ] Passo 5 do roteiro: uma saída por vez, nenhuma simultânea
- [ ] Passo 5.2: agitação conferida no osciloscópio ou multímetro

### Modelo 1 — CWH12AB V1 e V2
- [ ] Tampa pelo sensor eletrônico (PA4)
- [ ] Posição 1: AGITAR ≈ 400 s · Posição 11: ≈ 125 s
- [ ] Posição 12 agita em TURBO · posição 13 em DELICADO
- [ ] Centrifugação: 5 min sem "mais secas", 10 min com
- [ ] Posições 14 e 15 como etapa única
- [ ] Ciclo completo sem travar

### Modelo 2 — CWH12AB V3, V4 e V5
- [ ] Tampa eletrônica (PA4)
- [ ] **Posição 11: ≈ 110 s** (é o que o separa do modelo 1)
- [ ] Centrifugação: 5 min sem "mais secas", 10 min com
- [ ] Ciclo completo sem travar

### Modelo 3 — CWE10AB / CW11AB / CWG12AB
- [ ] Tampa eletrônica (PA4)
- [ ] Posição 2: AGITAR ≈ 480 s, em DELICADO · posição 6 em TURBO
- [ ] **Só enxaguar na 7, só centrifugar na 8**
- [ ] Centrifugação: 6 min sem "mais secas", **12 min** com (não corta)
- [ ] Posições 14 e 15 são programas normais aqui (enchem)
- [ ] Ciclo completo sem travar

### Modelo 4 — CWS11AB
- [ ] Tampa eletrônica (PA4)
- [ ] Posição 9: AGITAR ≈ 480 s
- [ ] Centrifugação: 5 min sem "mais secas", 10 min com
- [ ] Ciclo completo sem travar

### Modelo 5 — CWE08AB / CWE09AB
- [ ] **Tampa pelo sensor mecânico (PA7)** — o eletrônico é ignorado
- [ ] Posição 10: AGITAR ≈ 480 s
- [ ] **Só centrifugar na posição 0, só enxaguar na 15**
- [ ] **"Mais secas" faz enxágue duplo**, e não encurta a centrifugação
- [ ] **Centrifugação sempre 6 min**, com ou sem "mais secas"
- [ ] Posições 10 e 13 agitam em DELICADO
- [ ] Ciclo completo sem travar

### Modelo 6 — CWH12BB / CWB09BB
- [ ] **Tampa pelo sensor mecânico (PA7)**
- [ ] **Tampa aberta → pausa; retoma só com o ON/OFF depois de fechar**
- [ ] **Girar o seletor deixa em pausa** (não recomeça sozinho)
- [ ] Posição 0: ≈ 320 s · posição 11: ≈ 120 s
- [ ] Posições 0 e 12 em TURBO · 5, 6, 9 e 13 em DELICADO
- [ ] Centrifugação: 5 min sem "mais secas", 10 min com
- [ ] Os três botões respondem (é o modelo que revelou o bug do ADC)
- [ ] Ciclo completo sem travar

### Modelo 8 — combinação inválida
- [ ] Três LEDs piscando de 700 em 700 ms, nenhum triac liga
- [ ] Voltando a chave para uma combinação válida, o firmware sai do erro

---

## Se algo não fechar

Anote **o modelo, a posição do encoder e o que os LEDs faziam**. Com
isso dá para ir direto ao ponto: a tabela de tempos está em
`src/tabelas.rs`, o padrão de agitação em `src/agita.rs`, e as regras
de tampa, pausa e centrifugação em `src/maquina.rs`.
