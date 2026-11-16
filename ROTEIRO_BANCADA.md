# Roteiro de bancada — primeira gravação do GD32C231K8T6

Para a primeira placa com o GD32 no lugar do STM32. A ordem importa:
cada passo só faz sentido se o anterior passou.

> **Segurança.** Esta placa chaveia 220 V em triacs que acionam motor,
> bomba e válvula de água. Nos passos 1 a 4 **não ligue a carga de
> potência**. Quando ligar, tenha a mangueira em balde ou ralo, e nunca
> ponha a mão no cesto com o motor energizado. Não burle o sensor de
> tampa em nenhum momento — é ele que corta os triacs.

---

## 0. Antes de energizar

- [ ] MCU na orientação certa (pino 1 / chanfro conferindo com a serigrafia)
- [ ] Nenhum curto entre pinos adjacentes do LQFP32 (continuidade, principalmente PA13/PA14 e os pinos de potência)
- [ ] Alimentação: 3,3 V em VDD, 0 V em VSS
- [ ] SWD ligado: **PA13 = SWDIO**, **PA14 = SWCLK**, GND comum com o gravador
- [ ] Chave de modelo numa combinação **válida** — para o primeiro teste use a do **auto-teste**:

```text
  SW1=0  SW2=1  SW3=1  SW4=0     → modelo 7 (auto-teste de linha)
```

Se a chave estiver numa combinação inválida, o firmware entra em erro
de modelo logo no boot: os três LEDs piscam de 700 em 700 ms para
sempre e nada liga. É comportamento correto, não defeito.

---

## 1. Conferir as tabelas (sem placa, antes de gravar)

```bash
python -I ferramentas/confere_contra_cpp.py
```

Tem de terminar em `Tudo certo: 45 checagens, nenhuma divergencia.`

---

## 2. Gravar

Da **raiz do projeto** (o caminho do descritor no `.cargo/config.toml`
é relativo):

```bash
cargo run --release
```

Esperado: o probe-rs reconhece o `GD32C231K8`, apaga, grava e verifica.

**Se o probe-rs não achar o chip:**

```bash
probe-rs list
```

**Se gravar mas não conseguir reconectar depois**, é o watchdog: com o
firmware travado o FWDGT reseta o núcleo a cada 2,8 s e a sessão de
debug cai. Conecte com reset:

```bash
probe-rs attach --chip GD32C231K8 --chip-description-path target-probe/GD32C231K8.yaml --connect-under-reset
```

---

## 3. Boot — os primeiros 3 segundos

Com a chave no **modelo 7**, a sequência é:

| Momento | O que acontece |
|---------|----------------|
| 0 → 300 ms | nada; a chave e o encoder assentam |
| 300 ms | lê a chave de modelo; vê o 7 e entra em auto-teste |
| 300 ms → 2,3 s | os três LEDs invertem 4× a cada 500 ms (**2 piscadas**) |
| 2,3 s → 2,6 s | pausa |
| 2,6 s em diante | padrão de 8 passos do auto-teste, trocando a cada 301 ms |

Já dá para separar três coisas só pelo ritmo dos LEDs:

| Ritmo dos três LEDs | Significa |
|---------------------|-----------|
| 2 piscadas de 500 ms e para | boot normal |
| pisca **lento** (700 ms), sem parar | **modelo inválido** — confira SW1..SW4 |
| pisca **rápido** (250 ms), sem parar | tempo de etapa estourou: não encheu em 50 min ou não drenou em 6 min |
| padrão irregular de 301 ms | auto-teste rodando |

Os dois piscas de erro são de velocidade bem diferente de propósito —
é o que separa "placa montada com a chave errada" de "não está
entrando ou saindo água".

- [ ] Os três LEDs piscam 2× no boot
- [ ] Não entra no pisca de 700 ms

**Se nenhum LED acender:** meça PA1, PC14 e PB9. Saída é **lógica
invertida** — nível BAIXO acende. Se os três ficarem em 3,3 V fixos, o
firmware não chegou a rodar (confira o clock e a gravação). Se ficarem
em 0 V fixos, é a parte de LED do painel, não o MCU.

---

## 4. Triacs desligados no boot — **antes de ligar a carga**

Com a placa energizada mas **sem potência nos triacs**, meça as cinco
saídas. Todas têm de estar em **nível ALTO** (desligado):

| Saída | Pino | Esperado |
|-------|------|----------|
| Válvula de sabão | PA2 | 3,3 V |
| Atuador / válvula de amaciante | PA3 | 3,3 V |
| Motor C / pulso | PA9 | 3,3 V |
| Motor A | PC6 | 3,3 V |
| Bomba | PB2 | 3,3 V |

O firmware aplica o nível **antes** de comutar o pino para saída
(`Saida::nova` em [mcu/gpio.rs](src/mcu/gpio.rs)), justamente para
nenhum triac piscar no boot. Este passo verifica isso.

- [ ] As cinco saídas em 3,3 V no boot

> Só ligue a potência depois que este passo passar.

---

## 5. Auto-teste de linha — uma saída por vez

A chave no modelo 7 roda o auto-teste do firmware. **Cada giro do
encoder avança um passo.** Cada passo desliga as saídas do anterior
antes de ligar a sua, então nunca há duas acionadas juntas.

| Passo | O que liga | Pino | Observação |
|-------|-----------|------|------------|
| 0 | nada | — | zera o relógio; estado inicial |
| 1 | válvula de sabão | PA2 | entra água |
| 2 | agitação padrão | PC6 / PA9 | vai-e-vem (tempos abaixo) |
| 3 | atuador, e bomba após 3 s | PA3, PB2 | drena |
| 4 | pulso de motor após 15 s | PA9 | centrifuga |
| 5 | nada | — | fim; o próximo giro volta ao passo 0 |

Há um atraso de 2 s entre passos (os motores e a válvula ficam
desligados nesse intervalo) — é o `delay_5` de estabilização.

### Passo 2 — conferir a agitação com osciloscópio ou multímetro

O padrão **Padrão** ([agita.rs](src/agita.rs)) sobre a base de 21 ms:

```text
  motor A ligado   ~0,29 s   (contagens 0..13)
  tudo parado      ~0,44 s   (14..34)
  motor C ligado   ~0,29 s   (35..48)
  tudo parado      ~0,44 s   (49..69)
  ciclo completo   ~1,49 s
```

- [ ] Passo 1: só PA2 ativo
- [ ] Passo 2: PC6 e PA9 alternam, **nunca juntos**, ciclo ~1,5 s
- [ ] Passo 3: PA3 ativa, PB2 entra 3 s depois
- [ ] Passo 4: PA9 ativa 15 s depois do início do passo
- [ ] Passo 5: tudo desliga
- [ ] Cada giro do encoder avança exatamente um passo

**Se o giro do encoder não avançar o passo**, é o encoder: veja o
passo 7.

---

## 6. Os três botões — valida o ADC

Este é o passo que fecha os **pontos abertos 1, 2 e 3** do README (canal
do ADC, resolução e clock). Não precisa de instrumento: cada botão tem
um efeito visível próprio.

Ponha a chave num **modelo normal** (ex.: `SW1=0 SW2=0 SW3=0 SW4=1` →
modelo 1) e reenergize.

| Botão | Efeito esperado |
|-------|-----------------|
| Roupas Mais Secas | acende/apaga o LED de PC14 a cada aperto |
| Enxágue Duplo | acende/apaga o LED de PB9 a cada aperto |
| ON/OFF (pausa) | pausa o ciclo; aperte de novo e ele continua |

Segurar o botão **não** deve repetir a ação — a ação sai no aperto e
rearma quando o painel volta ao repouso.

- [ ] Cada botão mexe no seu LED
- [ ] Botão segurado não repete
- [ ] Nenhum botão mexe no LED do outro

### Se der errado, o sintoma diz onde

| Sintoma | Causa provável | Onde mexer |
|---------|----------------|------------|
| **Nenhum** botão funciona | a leitura de repouso caiu fora de 7..60, então a ação nunca rearma | `REPOUSO` em [botoes.rs](src/botoes.rs) |
| Botão **errado** responde | escala deslocada — a resolução não é 10 bits | `DRES_10_BITS` em [mcu/adc.rs](src/mcu/adc.rs) |
| Resposta **errática** | clock do ADC ou tempo de amostragem | `SPT_MAIS_LONGO` e `RCU_CFG1.ADCSEL/ADCPSC` |
| Nada, nem errático | canal errado | `CANAL_BOTOES` em [mcu/adc.rs](src/mcu/adc.rs) |

### Modo de diagnóstico dos botões

Se o teste acima falhar, existe um build de bancada que mostra nos
próprios LEDs o que o ADC está lendo — sem osciloscópio, sem debugger:

```bash
cargo run --release --features diag-botoes
```

Ele troca a máquina de estados por um diagnóstico. **Os triacs ficam
desligados o tempo todo**, então pode rodar sem água e sem carga.

| Padrão dos LEDs | Significa |
|-----------------|-----------|
| todos apagados | repouso (7..60) — o ADC está bom |
| só o LED de ON/OFF aceso | botão **Pausa** reconhecido |
| só o LED de mais secas aceso | botão **Mais Secas** reconhecido |
| só o LED de enxágue duplo aceso | botão **Enxágue Duplo** reconhecido |
| os três piscando juntos | valor **fora de todas as faixas** |
| o LED de ON/OFF piscando sozinho | o ADC **não está convertendo** |

Os dois últimos são os casos de defeito, e são bem diferentes:

* **Fora de todas as faixas** — o ADC lê, mas o número não cai em
  nenhuma janela de [botoes.rs](src/botoes.rs). É resolução (12 bits em
  vez de 10) ou o divisor resistivo.
* **Não está convertendo** — a conversão nem termina. É canal, clock ou
  habilitação do periférico em [mcu/adc.rs](src/mcu/adc.rs).

Com o diagnóstico rodando, aperte um botão de cada vez e veja em que
caso cai. Depois volte ao build normal:

```bash
cargo run --release
```

### Ver o número exato pelo SWD

Para o valor cru, o firmware deixa cinco contadores em RAM com símbolo
fixo: `DIAG_AD`, `DIAG_AD_MIN`, `DIAG_AD_MAX`, `DIAG_ADC_OK` e
`DIAG_ADC_FALHA`. O perfil release apaga os símbolos, então levante os
endereços no build de depuração, que tem a mesma otimização:

```bash
cargo build
```

```bash
arm-none-eabi-nm target/thumbv6m-none-eabi/debug/consul_mult_lilos | grep DIAG_
```

Com a placa rodando, leia o endereço que o `nm` deu:

```bash
probe-rs read b16 0x20000000 1 --chip GD32C231K8 --chip-description-path target-probe/GD32C231K8.yaml
```

Segure um botão e leia `DIAG_AD` algumas vezes para ter o valor dele.
Para a faixa de repouso, religue a placa, não toque em nada e leia
`DIAG_AD_MIN` e `DIAG_AD_MAX`.

> O FWDGT estoura em 2,8 s. Se o `probe-rs` parar o núcleo para ler, a
> placa reseta — leia rápido, ou use o modo de diagnóstico acima, que
> não depende do debugger.

---

## 7. Encoder — as 16 posições

Com a chave num modelo normal, gire o seletor posição por posição.

Cada giro **recomeça o ciclo** no programa novo:

| Modelo | O que acontece ao girar |
|--------|-------------------------|
| 1–5 e 7 | 2 s com tudo desligado e começa sozinho |
| **6** | fica em pausa, pisca o LED de ON/OFF 4× e espera o ON/OFF |

- [ ] Todas as 16 posições são reconhecidas (nenhuma "morta")
- [ ] Uma posição não pula duas
- [ ] No modelo 6, o pisca de confirmação aparece

**Posição morta** quer dizer trilha não chegando: meça PB4, PB3, PB5 e
PB6 em cada posição e compare com a tabela de [encoder.rs](src/encoder.rs).
Lembre que as trilhas têm **pull-down** interno.

---

## 8. Tampa — a proteção principal

Com a máquina rodando (motor ou bomba ativos), **abra a tampa**.

| Esperado | |
|---|---|
| Triacs cortam | imediatamente |
| LED de ON/OFF | passa a piscar (301 ms) |
| Relógio do ciclo | para onde estava |
| Ao fechar | retoma 2 s depois (modelo 6: só com o ON/OFF) |

Modelos 5 e 6 leem a tampa **mecânica** (PA7); os outros, a
**eletrônica** (PA4).

- [ ] Abrir a tampa corta os triacs
- [ ] LED de ON/OFF pisca com a tampa aberta
- [ ] Fechar retoma do ponto onde parou, 2 s depois
- [ ] Abrir e fechar rápido várias vezes não deixa triac ligado

Este é o **ponto aberto 4** do README: o debounce virou 2 leituras de
21 ms (~42 ms), contra ~10 ms do laço livre do C++. Se a resposta
parecer lenta ou se a tampa "tremer" entre aberta e fechada, ajuste
`LEITURAS_ESTAVEIS_TAMPA` em [tarefas.rs](src/tarefas.rs).

---

## 9. Pressostato — nível de água

Com a válvula enchendo, acompanhe até o nível fechar o pressostato.

- [ ] A válvula fecha quando o nível é atingido
- [ ] O ciclo passa de ENCHER para AGITAR
- [ ] Não fica oscilando entre as duas etapas

A oscilação da água é tratada por média de 10 amostras a cada 81 ms
(~0,8 s de janela). Se ainda pular de etapa, o filtro é a
`FiltroPressostato` em [tarefas.rs](src/tarefas.rs).

Nível BAIXO no pino = **cheio**.

---

## 10. Retomada após queda de luz

Com um ciclo rodando no meio (de preferência em AGITAR ou MOLHO):

1. Corte a energia
2. Religue
3. A máquina tem de voltar **na mesma etapa**, com as mesmas opções

- [ ] Volta na mesma etapa
- [ ] "Mais secas" e "enxágue duplo" voltam como estavam
- [ ] Se estava em pausa, volta em pausa

O estado fica na última página da flash (`0x0800_FC00`). **Placa nova
começa limpa** — a flash virgem é lida como "não há estado salvo", e é
de propósito (diferença 4 do README; no C++ ela acendia as três opções
sozinhas).

Se a retomada não funcionar, confira que o `.hex` termina antes de
`0x0800FC00` — o gravador não pode sobrescrever essa página.

---

## 11. Watchdog

O FWDGT estoura em **2,8 s**. Para confirmar que está armado, pare o
núcleo com o debugger e deixe parado: a placa tem de resetar (os LEDs
voltam a piscar as 2 piscadas de boot).

- [ ] Núcleo parado → reset em ~3 s

---

## 12. Ciclo completo

Por último, um ciclo inteiro num programa curto, com carga de potência
e água, acompanhando a sequência:

```text
  ENCHER → AGITAR → MOLHO → ENXAGUAR → CENTRIFUGAR → DESLIGAR
```

- [ ] Um ciclo completo sem travar e sem erro de interface
- [ ] Mesmo programa, mesmos tempos da máquina de referência com o firmware C++

Vale cronometrar as etapas e comparar com uma máquina rodando o C++ —
é a conferência final de que os tempos portados estão certos na prática,
e não só na tabela.

---

## Se algo não fechar

Anote **em qual passo** parou e o que os LEDs estavam fazendo. Com
isso dá para ir direto ao módulo: o ritmo dos LEDs separa erro de
modelo, tempo estourado, auto-teste e operação normal; e os passos 6,
7, 8 e 9 isolam ADC, encoder, tampa e pressostato um a um.
