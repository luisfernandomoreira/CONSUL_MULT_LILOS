# CONSUL MULT — Firmware Máquina de Lavar · GD32C231K8T6 + Rust + lilos

Porte do firmware C++ de fábrica (projeto CubeIDE `CONSUL_MULT_V1`,
STM32C031K6T6 + HAL/LL) para o **GD32C231K8T6** em **Rust** com o RTOS
**lilos**.

A placa é a mesma. Só mudou o microcontrolador e a linguagem.

> **Vai gravar a primeira placa?** Siga o
> **[Roteiro de bancada](ROTEIRO_BANCADA.md)** — 12 passos, do teste de
> continuidade ao ciclo completo, com o que medir em cada pino e o que
> cada ritmo de pisca dos LEDs quer dizer.

---

## Índice

| | |
|---|---|
| **Entender** | [1. Por que a placa não muda](#1-por-que-a-placa-não-muda) · [2. Uma placa, oito lavadoras](#2-uma-placa-oito-lavadoras) · [3. O painel](#3-o-painel) |
| **Como funciona** | [4. O modelo de execução](#4-o-modelo-de-execução) · [5. A lógica do ciclo](#5-a-lógica-do-ciclo) · [6. O que para o ciclo](#6-o-que-para-o-ciclo) |
| **Em campo** | [7. Proteções e códigos de erro](#7-proteções-e-códigos-de-erro) |
| **Referência** | [8. Mapa dos arquivos](#8-mapa-dos-arquivos) · [9. O que mudou em relação ao C++](#9-o-que-mudou-em-relação-ao-firmware-c) · [10. Dois pinos com dois nomes](#10-dois-pinos-com-dois-nomes) · [11. Mapa de pinos](#11-mapa-de-pinos) · [12. Estado salvo na flash](#12-estado-salvo-na-flash) |
| **Trabalhar** | [13. Compilar e gravar](#13-compilar-e-gravar) · [14. Alvo de compilação](#14-alvo-de-compilação--por-que-thumbv6m-e-não-thumbv8mbase) · [15. O PAC](#15-o-pac) · [16. Como mexer no firmware](#16-como-mexer-no-firmware) |

---

## 1. Por que a placa não muda

O GD32C231K8T6 (LQFP32) é **pino a pino** igual ao STM32C031K6T6
(LQFP32) que a PCI usava. Mesma pinagem, mesmo encapsulamento, mesmo
layout — nenhum traço de cobre mudou.

| | STM32C031K6T6 | GD32C231K8T6 |
|---|---|---|
| Núcleo | Cortex-M0+ | Cortex-M23 |
| Flash | 32 KB | 64 KB |
| SRAM | 12 KB | 12 KB |
| Clock usado | 24 MHz | 48 MHz |
| Encapsulamento | LQFP32 | LQFP32 |

O clock mais alto não muda comportamento nenhum: todos os tempos do
ciclo são contados em segundos e milissegundos, não em ciclos de
máquina.

---

## 2. Uma placa, oito lavadoras

A mesma PCI atende várias lavadoras Consul. Quem decide qual é a
máquina é a **chave SW1..SW4** da placa:

| Modelo | Lavadoras | SW1..SW4 |
|--------|-----------|----------|
| 1 | CWH12AB V1 e V2 | `0 0 0 1` |
| 2 | CWH12AB V3, V4 e V5 | `0 0 1 0` |
| 3 | CWE10AB / CW11AB / CWG12AB | `0 0 1 1` |
| 4 | CWS11AB | `0 1 0 0` |
| 5 | CWE08AB / CWE09AB | `1 0 0 0` |
| 6 | CWH12BB / CWB09BB | `1 1 1 1` |
| 7 | **auto-teste de linha** | `0 1 1 0` |
| 8 | combinação inválida | qualquer outra |

Cada modelo tem seus próprios tempos de agitação, de molho e de
centrifugação — são as tabelas de [tabelas.rs](src/tabelas.rs).

O modelo não é lido só no boot: **toda etapa relê a chave a cada
passada**. É de propósito. Se a chave estiver num contato ruim e
oscilar para uma combinação inválida no meio de um ciclo, a máquina
desliga os triacs na hora em vez de continuar rodando o programa de
outra lavadora.

Modelo 8 (inválido) desliga todos os triacs e pisca os três LEDs do
painel de 700 em 700 ms, para sempre. É a proteção contra placa montada
com a chave errada.

---

## 3. O painel

### Seletor de programa — encoder de 16 posições

Um encoder absoluto de 4 trilhas (PB4, PB3, PB5, PB6). A combinação das
trilhas dá a posição 0..15, que é o **tipo de lavagem**. A tabela não é
binária direta: é a sequência física das pistas do disco, em
[encoder.rs](src/encoder.rs).

Girar o seletor **recomeça o ciclo** no programa novo. Nos modelos 1–5
e 7 a máquina espera 2 s com tudo desligado e começa sozinha; no
**modelo 6** ela fica em pausa, pisca o LED de ON/OFF 4× e espera a
confirmação no botão.

Algumas posições do encoder são programas de uma etapa só:

| Modelo | Só enxaguar | Só centrifugar |
|--------|-------------|----------------|
| 1, 2, 4, 6 | posição 14 | posição 15 |
| 3 | posição 7 | posição 8 |
| 5 | posição 15 | posição 0 |

### Três botões num pino só

Os botões **Pausa (ON/OFF)**, **Roupas Mais Secas** e **Enxágue Duplo**
estão num divisor resistivo ligado a **PA6**, lido pelo ADC. O firmware
descobre qual foi apertado pela faixa de tensão:

```text
    0 ..  60   nenhum botão apertado   (medido:   0)
  764 .. 814   Pausa / ON-OFF          (medido: 789)
  824 .. 874   Roupas Mais Secas       (medido: 849)
  906 .. 956   Enxágue Duplo           (medido: 931)
```

Os quatro valores foram **medidos nesta placa**, não herdados do C++ —
veja [a seção do ADC](#9-o-que-mudou-em-relação-ao-firmware-c).

Leitura de **10 bits** (0..1023) — é o que fixa a resolução do ADC em
[mcu/adc.rs](src/mcu/adc.rs). As faixas estão em
[botoes.rs](src/botoes.rs).

A ação sai **no aperto, uma vez só**. O mecanismo é um ferrolho: a
faixa de repouso **arma** a ação, e o aperto a **consome**. Enquanto o
botão estiver segurado, a leitura continua na faixa dele e nada mais
acontece; só quando o dedo sai e o painel volta ao repouso é que a
próxima ação é armada. Por isso a faixa de repouso é tão importante
quanto as dos botões: se a leitura em repouso cair fora de 7..60,
nenhum botão funciona.

### Três LEDs

| LED | Pino | O que mostra |
|-----|------|--------------|
| ON/OFF (pause) | PA1 | aceso: rodando · piscando: tampa aberta ou pausa |
| Roupas mais secas | PC14 | opção ligada |
| Enxágue duplo | PB9 | opção ligada |

Os três piscando juntos = erro. O **ritmo** diz qual:

| Ritmo dos três LEDs | Significa |
|---------------------|-----------|
| 2 piscadas de 500 ms e para | boot normal |
| **N piscadas, pausa escura, repete** | proteção disparou — conte o N na [seção 7](#7-proteções-e-códigos-de-erro) |
| pisca lento (700 ms), sem parar | chave de modelo inválida |
| pisca rápido (250 ms), sem parar | falha ao gravar na flash |
| padrão irregular de 301 ms | auto-teste rodando |

Os três primeiros não se confundem: o código de erro tem **pausa
escura** entre as repetições, e os outros dois são pisca contínuo, de
velocidades bem diferentes.

---

## 4. O modelo de execução

### O laço que virou tarefas

O firmware C++ é um laço infinito. Ele gira o mais rápido que o núcleo
permite, e em cada volta faz duas coisas:

```c
while (1) {
    maquina.selecionaProgramasDeLavagem();  // lê encoder, botões, pressostato
    maquina.ExecutarEtapa();                // executa a etapa atual
    LL_IWDG_ReloadCounter(IWDG);            // alimenta o watchdog
}
```

Quem conta o tempo são quatro timers por interrupção: TIM14 (1 s),
TIM16 (21 ms), TIM3 (81 ms) e TIM17 (301 ms).

Aqui o laço virou **sete tarefas** do lilos:

| Tarefa | Período | O que faz | No C++ era |
|--------|---------|-----------|------------|
| `tempo` | 1 s | conta os segundos do ciclo | ISR do TIM14 |
| `agitacao` | 21 ms | move o contador do vai-e-vem | ISR do TIM16 |
| `tampa` | 21 ms | filtra os dois sensores de tampa | `getLeituraTampa` |
| `amostragem` | 81 ms | encoder, botões (ADC), pressostato | TIM3 |
| `maquina` | por sinal | executa a etapa atual | `while(1)` |
| `gravacao` | por pedido | grava o estado na flash | `gravaDadosFlash` |
| `watchdog` | 1 s | alimenta o FWDGT (2,8 s) | `ReloadCounter` |

As tarefas estão em [tarefas.rs](src/tarefas.rs) e nascem todas no
[main.rs](src/main.rs).

### Por que async e não interrupção

O lilos é um executor **cooperativo de uma pilha só**: não há
preempção, não há pilha por tarefa, não há disputa por `mutex`. Cada
tarefa roda até encontrar um `.await`, e aí devolve o controle. O
executor dorme num `WFI` quando ninguém tem nada a fazer, e o SysTick
o acorda.

Para esta máquina isso cai como uma luva:

* **Não há disputa de dado.** Duas tarefas nunca estão no meio de uma
  escrita ao mesmo tempo, porque só uma roda por vez. O estado
  compartilhado existe (ver abaixo), mas o acesso é curto e a seção
  crítica está ali mais pela forma do que por necessidade real de
  exclusão.
* **Esperar fica legível.** No C++, "esperar 700 ms piscando o LED" é
  `LL_mDelay(700)` — que **trava o núcleo**: o laço para, e só os
  timers continuam. Aqui é `sleep_for(Millis(700)).await`, e as outras
  seis tarefas seguem rodando, inclusive o watchdog.
* **O custo é a disciplina.** Uma tarefa que nunca dê `.await` trava
  todo o firmware. É por isso que a máquina de estados **nunca** espera
  dentro de uma etapa: ela decide pelo estado e sai (ver seção 5).

A única interrupção ativa é o **SysTick**, que é do núcleo. Nenhuma
interrupção de periférico é usada — ver a
[seção 14](#14-alvo-de-compilação--por-que-thumbv6m-e-não-thumbv8mbase).

### A cadência de 21 ms

A tarefa mais rápida é a `agitacao`, de 21 ms, e é ela que dá o ritmo
da máquina de estados: **a etapa atual é reavaliada a cada 21 ms.**

Esse número não foi escolhido aqui — é o período do TIM16 do firmware
C++ (24 MHz ÷ 24000 ÷ 21), que move o contador do vai-e-vem do motor.
Como a agitação é a coisa mais rápida que o ciclo precisa resolver,
avaliar a máquina na mesma cadência basta para todo o resto.

### Por que não há fila de eventos

Seria natural pôr uma fila de eventos entre as tarefas de leitura e a
máquina. Não há, e é deliberado.

O que a máquina lê — [relogio.rs](src/relogio.rs),
[agita.rs](src/agita.rs) e [leituras.rs](src/leituras.rs) — é
**estado**, não mensagem. Numa fila, um tique perdido por fila cheia
seria tempo de ciclo perdido; com estado, uma passada perdida é só uma
passada, e a próxima vem 21 ms depois já com o valor certo.

Então [evento.rs](src/evento.rs) não é fila: é um **sinal de
acordar**. Qualquer tarefa chama `evento::acorda()`, e vários sinais
entre duas passadas valem um só.

```text
   tarefas::tempo      (1 s)  ──┐
   tarefas::agitacao  (21 ms)  ─┼──→  SINAL  ──→  tarefas::maquina
   tarefas::amostragem (81 ms) ─┘                      │
                                                       ▼
                                             [triacs e LEDs]
```

Há uma exceção: `encoder_mudou`, em `leituras`, é **pegajoso**. Fica
marcado até a máquina consumi-lo, porque girar o seletor é um evento
de verdade — não dá para perder a troca de programa só porque a
máquina estava ocupada piscando um LED.

### Estado compartilhado

Três módulos guardam estado em `static` porque quem escreve e quem lê
são tarefas diferentes. É o mesmo papel das variáveis `volatile
static` que as ISRs do C++ atualizam, e o motivo de existirem é o
mesmo: **a contagem de tempo não pode parar porque a máquina de
estados está ocupada.**

| Módulo | Guarda | Quem escreve | Quem lê |
|--------|--------|--------------|---------|
| [relogio.rs](src/relogio.rs) | `total_segundos`, `delay_5`, `delay_25`, ligado/parado | `tarefas::tempo` | máquina |
| [agita.rs](src/agita.rs) | contador do vai-e-vem, timer ligado | `tarefas::agitacao` | máquina |
| [leituras.rs](src/leituras.rs) | ADC, nível de água, tampas, posição do encoder | `amostragem` e `tampa` | máquina |

Dois valores de `leituras` já chegam tratados:

* **`nivel_agua`** vem invertido e filtrado. O pressostato em nível
  baixo significa cheio, e a média de 10 amostras a cada 81 ms
  (~0,8 s de janela) evita que a água batendo no tubo faça o ciclo
  pular de etapa.
* **`tampa`** vem com debounce de 2 leituras de 21 ms. Os **dois**
  sensores são filtrados sempre; quem escolhe qual vale é a máquina,
  pelo modelo (mecânico no 5 e 6, eletrônico nos outros).

### O caminho de ponta a ponta

Uma troca de programa, do dedo ao triac:

```text
 1. dedo gira o seletor
 2. tarefas::amostragem (81 ms) lê PB4/PB3/PB5/PB6
 3. encoder::DetectorMudanca vê que mudou
 4. leituras::marca_encoder(nova_posicao)      ← fica marcado
 5. evento::acorda()
 6. tarefas::maquina desperta
 7. maquina::passo() consome a marca
 8. maquina::selecionar_nova_lavagem():
      desliga triacs · zera relógio · zera molho
      decide a etapa inicial pelo modelo e pela posição
      pede a gravação na flash
 9. modelo 6: pisca o ON/OFF 4× e fica em pausa
    outros : espera 2 s com tudo desligado e começa
10. tarefas::gravacao grava o estado, fora do caminho da decisão
```

---

## 5. A lógica do ciclo

### As sete etapas

```text
  ENCHER ──→ AGITAR ──→ MOLHO ──→ ENXAGUAR ──→ CENTRIFUGAR ──→ DESLIGAR
    ↑          │          │           │
    └──────────┴──────────┘           │   (nível de água caiu:
       sem água, volta a encher       │    volta a encher)
                                      │
  TESTE: entra só quando a chave de modelo está na posição 7
```

O número de cada etapa é o que vai para a flash, então a ordem do
`enum Etapa` não pode mudar sem invalidar o estado gravado nas placas
que já estão em campo.

### Como uma etapa anda

Esta é a ideia central do firmware, e vale entender antes de ler
qualquer etapa.

Uma etapa **não é um procedimento que roda do começo ao fim**. É uma
função chamada de novo a cada 21 ms que, em cada chamada, olha o
relógio e os sensores e decide **o que as saídas devem estar fazendo
agora**. Ela nunca espera, nunca faz laço, nunca bloqueia.

Quem anda é o relógio; a etapa só lê.

```rust
// o esqueleto de toda etapa
let total = relogio::total_segundos();   // em que ponto da etapa estamos
if <condição de saída>  { transicionar_para(<próxima>); return; }
if <janela de tempo A>  { <liga estas saídas>;          return; }
if <janela de tempo B>  { <liga aquelas>;               return; }
```

Duas consequências práticas:

* **Ligar uma saída é idempotente.** Escrever "bomba ligada" 50 vezes
  no mesmo segundo é inofensivo, e é o que acontece.
* **Parar o relógio congela a etapa.** Não é preciso guardar "onde eu
  estava": o ponto da etapa *é* o valor do relógio. É assim que a
  tampa aberta e a pausa funcionam — param o relógio e desligam os
  triacs; quando o relógio volta a andar, a etapa continua exatamente
  de onde estava.

### O prólogo que toda etapa executa

Antes da lógica própria, toda etapa passa pela mesma peneira, nesta
ordem:

| # | Pergunta | Se a resposta for ruim |
|---|----------|------------------------|
| 1 | Qual é o modelo? É válido? | inválido: desliga tudo, pisca 700 ms, zera o relógio e sai |
| 2 | A tampa está aberta, ou a máquina está em pausa? | desliga tudo, para o relógio, pisca o ON/OFF e sai |
| 3 | A tampa acabou de fechar? Já passaram os 2 s? | não passaram: desliga tudo e sai |
| 4 | Já passou o atraso de estabilização (`delay_5` ≥ 3 s)? | não: desliga tudo e sai |
| 5 | O nível de água está como esta etapa precisa? | não: volta para ENCHER |

Só depois disso a etapa faz o que é dela. ENCHER tem um passo antes de
todos — a checagem de tempo estourado —, e ENXAGUAR usa uma variante
do item 2 que também para o ciclo quando o programa é de lavagem
econômica (ver adiante).

O item 4 é a razão de os triacs nunca chavearem junto: depois de
qualquer parada, a etapa fica 3 s desligando tudo antes de voltar a
acionar.

### Os três atrasos, e para que serve cada um

| Atraso | Onde | Para quê |
|--------|------|----------|
| `delay_5` (≥ 3 s) | prólogo de toda etapa | estabilização elétrica depois de qualquer parada; satura em 5 |
| `delay_25` (≥ 18 s) | drenagem e centrifugação | dar tempo de a água começar a sair antes de pulsar o motor; satura em 25 |
| 2 s (tempo absoluto) | tampa fechada e troca de programa | espera com tudo desligado; não usa o relógio do ciclo, que está parado nesse momento |

Os dois primeiros andam com o relógio do ciclo (param quando ele
para). O terceiro é medido em tempo absoluto (`TickTime`), justamente
porque precisa correr com o relógio parado.

Quando um dos 2 s termina, o firmware escreve `delay_5 = 3` para que o
prólogo não cobre outros 3 s em cima — a espera já foi cumprida com os
triacs desligados.

### ENCHER

Abre a válvula de sabão até o pressostato acusar nível.

```text
  nível atingido?  → AGITAR
  senão            → válvula de sabão ligada
  tempo > 50 min   → erro de tempo (pisca 250 ms)
```

O modelo 1, nos programas 12 e 13, faz mais: mantém o atuador ligado e,
depois de 13 s, pulsa o motor enquanto a água entra, para a roupa
assentar. O pulso é contado por **viradas de segundo**, não pelo
relógio direto — o contador é zerado abaixo de 8 s e reiniciado em 179
quando passa de 193, o que dá um pulso periódico.

### AGITAR

```text
  tempo ≥ tempo_agitar(modelo, programa)  → MOLHO
  sem água                                → ENCHER
  senão                                   → agita conforme modelo e programa
```

O tempo vem de `TEMPO_AGITAR` em [tabelas.rs](src/tabelas.rs). Zero
significa "este programa não existe neste modelo".

No **modelo 5** a agitação turbo é amarrada ao botão de enxágue duplo:
`bt_modo_agita = bt_enxague_duplo`. É a lavadora que usa o mesmo botão
para duas coisas.

### MOLHO

A etapa mais elaborada. Alterna duas fases, N vezes:

```text
   ┌─ MOLHO:  motores parados, por tempo_molho segundos
   │            ↓ ciclo++
   └─ AGITAR: agita, por tempo_agitar segundos
                ↓
      ciclo ≥ ciclos?  → ENXAGUAR
```

Os três números — `ciclos`, `tempo_molho`, `tempo_agitar` — vêm de
`CONFIG_MOLHO`, por modelo e programa. Variam muito: o modelo 1 no
programa 0 faz **36 ciclos de 92 s de molho com 33 s de agitação**; no
programa 5, **3 ciclos de 180 s com 170 s de agitação**.

Duas coisas valem notar:

* **O contador de agitação só liga na borda.** Trocar de fase liga ou
  para o contador do vai-e-vem uma única vez, não a cada passada —
  senão ele seria zerado sem parar e o motor nunca completaria um
  vai-e-vem.
* **O último ciclo sai em turbo.** `CICLO_TURBO` diz em qual ciclo de
  cada programa a agitação é mais forte, para soltar a roupa antes do
  enxágue. `-1` = nunca.

Ao sair para o enxágue é aqui que se decide se o programa é de
**lavagem econômica** (`REUTILIZA_AGUA`): se for, o ciclo vai pausar na
entrada do enxágue esperando a tampa abrir, para o usuário aproveitar a
água.

### ENXAGUAR

Três subetapas que se repetem:

```text
  1. DRENAR       tira a água da lavagem
  2. ENTRAR ÁGUA  enche de água limpa
  3. AGITAR       agita e deixa de molho
       └→ com enxágue duplo, volta para 1
       └→ senão, CENTRIFUGAR
```

**Drenar** é a subetapa com mais detalhe. Depois dos 18 s de
`delay_25`, a sequência por tempo é:

| Tempo | O que faz |
|-------|-----------|
| 0–120 s | atuador + bomba drenando — com uma janela em **70–85 s** em que a válvula abre e a bomba para (lava o tubo do pressostato) |
| 121–406 s | 14 pulsos curtos de motor, depois centrifugação até 396 s |
| fim | vai para ENTRAR ÁGUA |

Os pulsos estão em `FASES_DRENAGEM`, em [maquina.rs](src/maquina.rs).
Nos modelos 3 e 4 só a bomba drena (sem atuador).

**Agitar** (subetapa 3) tem quatro janelas fixas:

| Tempo | O que faz |
|-------|-----------|
| 0–200 s | agita (padrão conforme modelo/programa) |
| 201–251 s | molho de 50 s |
| 252–312 s | agita forte |
| 313–333 s | molho final de 20 s |
| 334–337 s | se há enxágue duplo pendente, volta a DRENAR |
| depois | CENTRIFUGAR |

#### Como o enxágue duplo se conta

Um contador, `num_enxagues_executado`, vale 1, 2 ou 3:

```text
  1 → primeiro enxágue em andamento
  2 → primeiro terminou; o segundo pode começar
  3 → segundo em andamento; depois dele, centrifugar
```

No primeiro enxágue dos modelos 1, 2, 5 e 6 a drenagem é **encurtada**:
aos 250 s o relógio salta para 397 s, pulando a centrifugação longa —
não faz sentido centrifugar entre dois enxágues. Nos modelos 3 e 4 o
encurtamento não existe; o contador avança no fim das fases.

E o **amaciante entra só no último enxágue**: nos modelos 3 e 4, a
subetapa de entrada de água abre a válvula de amaciante junto com a de
sabão quando o contador já não é 2.

Note ainda que, no **modelo 5**, quem manda no enxágue duplo é o botão
*"roupas mais secas"*, não o de enxágue duplo. Outra troca de papel de
botão específica daquela lavadora.

### CENTRIFUGAR

```text
  ainda tem água  → atuador + bomba, até 6 min (depois, erro de tempo)
  água saiu       → sequência de fases
```

Depois de drenar, as fases de `FASES_CENTRIFUGA`:

| Tempo | O que faz |
|-------|-----------|
| 0–90 s | bomba (e atuador, fora dos modelos 3 e 4) |
| 91–237 s | 13 pulsos curtos de motor, para desgrudar a roupa |
| 238–953 s | **centrifugação longa** — encurtada conforme a tabela abaixo |
| 954–972 s | motor desligado, desacelerando |
| 973–991 s | desliga bomba, depois atuador |
| fim | DESLIGAR |

O tempo de centrifugação é o único lugar em que o botão "roupas mais
secas" muda tempo de ciclo:

| Modelo | Normal | Mais secas |
|--------|--------|------------|
| 1, 2, 4, 6 | 5 min | 10 min |
| 3 | 6 min | 12 min |
| 5 | 6 min | 6 min (ignora o botão) |

O encurtamento é feito escrevendo `954` no relógio, o que empurra a
etapa para a fase de desaceleração. Como a etapa já leu o relógio no
começo da passada, o corte só vale na passada seguinte — o mesmo
comportamento do C++.

### DESLIGAR

Apaga os três LEDs, desliga todos os triacs, para os dois relógios e o
pisca, e zera o estado do ciclo (subetapa de enxágue, contador de
enxágues, flags de tampa e de pausa). Fica nesse estado até alguém
apertar um botão ou girar o seletor.

### TESTE — o auto-teste de linha

Com a chave no modelo 7, a máquina entra no auto-teste em vez do ciclo.
**Cada giro do encoder avança um passo**, e cada passo desliga as
saídas do anterior antes de ligar a sua:

| Passo | O que liga |
|-------|-----------|
| 0 | nada (zera o relógio) |
| 1 | válvula de sabão |
| 2 | agitação padrão |
| 3 | atuador, e bomba 3 s depois |
| 4 | pulso de motor 15 s depois |
| 5 | nada; o próximo giro volta ao passo 0 |

Os três LEDs piscam num padrão de 8 passos a cada 301 ms, para se
distinguir de qualquer outro estado. É o teste usado em bancada — ver o
[Roteiro de bancada](ROTEIRO_BANCADA.md).

> **O auto-teste é a única etapa que não checa a tampa**, e isso vem do
> firmware C++. É coerente com o propósito: o teste existe para
> observar cada atuador, o que em geral exige a tampa aberta. Mas o
> passo 4 gira o motor nessas condições — é modo de fábrica, não de
> campo.

### A agitação

O agitador alterna entre dois sentidos numa cadência que define a
força da lavagem:

```text
  contador:  0 ....... tempo_a ..... c_inicio ... c_fim ... reset
  motor:     [--- A ---]  (parado)   [---- C ----] (parado)   ↺
```

| Padrão | A | C início | C fim | reset | ciclo |
|--------|---|----------|-------|-------|-------|
| Padrão | 13 | 35 | 48 | 70 | ~1,49 s |
| Delicado | 12 | 74 | 86 | 147 | ~3,11 s |
| Turbo | 16 | 27 | 43 | 54 | ~1,16 s |
| Turbo delicado | 12 | 42 | 54 | 84 | ~1,79 s |

Os números são contagens do contador de 21 ms. O padrão **Padrão**, por
exemplo, dá motor A por ~0,29 s, pausa de ~0,44 s, motor C por ~0,29 s
e pausa de ~0,44 s.

Os dois sentidos **nunca** ficam ligados juntos, e a trava mora em
[placa.rs](src/placa.rs), não em quem chama: `motor_a` desliga o C
antes de ligar, e `motor_c` desliga o A.

Isso importa porque `motor_pulse` — os pulsos da drenagem e da
centrifugação — é chamado direto pelas fases, longe do `agita`. Com a
trava no driver, "os dois sentidos ligados" deixa de ser representável,
em vez de depender de cada etapa lembrar de cortar antes.

Qual padrão sai depende do modelo, do programa e da etapa — são quatro
funções de escolha em `agita.rs`, uma por contexto (`etapa_agitar`,
`etapa_molho`, `enxague_inicio`, `enxague_final`).

### Os dois relógios

| Relógio | Período | Para quê | No C++ |
|---------|---------|----------|--------|
| [relogio.rs](src/relogio.rs) | 1 s | duração das etapas; **pode ser parado** | TIM14 |
| [agita.rs](src/agita.rs) | 21 ms | cadência do vai-e-vem do motor | TIM16 |

---

## 6. O que para o ciclo

| Condição | Efeito | Como volta |
|----------|--------|------------|
| **Tampa aberta** | triacs desligados, relógio parado, ON/OFF piscando | 2 s depois de fechar (modelo 6: só com o ON/OFF) |
| **Pausa (ON/OFF)** | idem | apertar o ON/OFF de novo |
| **Seletor girado** | recomeça no programa novo | 2 s (modelo 6: ON/OFF) |
| **Modelo inválido** | LEDs piscando 700 ms, nada liga | corrigir a chave |
| **Proteção disparou** | tudo desligado, painel piscando o código ([seção 7](#7-proteções-e-códigos-de-erro)) | apertar ON/OFF ou girar o seletor |
| **Lavagem econômica** | pausa na entrada do enxágue | abrir a tampa |

Dois detalhes que não são óbvios:

* **Abrir a tampa cancela a lavagem econômica.** Faz sentido: abrir a
  tampa é exatamente o gesto de quem tirou a água e quer que o ciclo
  siga.
* **O modelo 6 exige confirmação.** Nos outros modelos, fechar a tampa
  retoma o ciclo sozinho 2 s depois. No 6, fechar não basta: o LED de
  ON/OFF fica piscando até alguém apertar o botão. É uma exigência
  daquela lavadora, não uma inconsistência.

---

## 7. Proteções e códigos de erro

Esta placa não tem display. Quando uma proteção dispara, a máquina
**para com tudo desligado** e o painel anuncia o que aconteceu pelo
**número de piscadas** dos três LEDs juntos.

O técnico conta as piscadas em campo, sem gravador, sem notebook e sem
abrir nada. Os números são os mesmos das outras placas da linha — quem
aprendeu numa, leu todas.

### As três proteções

| Proteção | O que vigia | Dispara quando |
|----------|-------------|----------------|
| `Encher` | o pressostato durante o enchimento | passam **50 min** e o nível não chegou — vale para ENCHER e para a entrada de água do enxágue |
| `Drenar` | o pressostato durante a drenagem | passam **6 min** com água ainda no tanque — vale para o enxágue e para a centrifugação |
| `Nivel` | o pressostato nas etapas que retêm água | o nível cai sozinho pela **3ª vez na mesma lavagem** — em AGITAR, MOLHO ou no molho do enxágue |

A de `Nivel` merece explicação. Perder o nível **uma vez** é normal:
espuma assentando solta o pressostato, e o ciclo simplesmente volta a
encher — é o que o firmware C++ faz, sempre. O problema é fazer isso
para sempre: com um vazamento, a máquina encheria e esvaziaria a noite
toda sem ninguém saber. Então a terceira recuperação na mesma lavagem
vira erro. O limite está em `LIMITE_RECUPERACOES_NIVEL`, em
[maquina.rs](src/maquina.rs).

### Os códigos piscados

Cada piscada é **250 ms acesa e 250 ms apagada**. Terminada a contagem,
o painel fica **1,5 s escuro** e tudo recomeça — para sempre, até
alguém intervir.

| Piscadas | Erro | O que suspeitar, nesta ordem |
|----------|------|------------------------------|
| **2** | `Nivel` | vazamento no tanque ou na mangueira · mangueira de saída solta ou caída · pressostato com mau contato |
| **3** | `Encher` | torneira fechada ou entupida · válvula de sabão (PA2) · pressostato preso em "cheio" |
| **4** | `Drenar` | bomba (PB2) · mangueira de saída dobrada ou alta demais · filtro entupido · pressostato preso em "cheio" |

```text
2 piscadas — Nivel

  ▀▀__▀▀__________________▀▀__▀▀__________________▀▀__▀▀_____ ...
  └ 1 s ┘└──── 1,5 s ────┘
  └──────── 2,5 s ───────┘

3 piscadas — Encher

  ▀▀__▀▀__▀▀______________▀▀__▀▀__▀▀______________▀▀__▀▀__▀▀_ ...
  └─ 1,5 s ─┘└─── 1,5 s ─┘
  └────────── 3 s ───────┘

4 piscadas — Drenar

  ▀▀__▀▀__▀▀__▀▀__________▀▀__▀▀__▀▀__▀▀__________▀▀__▀▀__▀▀_ ...
  └─── 2 s ────┘└─ 1,5 s ┘
  └─────────── 3,5 s ────┘
```

Duas escolhas de projeto que facilitam a contagem:

* **Começa em 2.** Uma piscada só se confundiria com o pisca de tampa
  aberta, que é o LED de ON/OFF sozinho.
* **A pausa escura é 3× o intervalo entre piscadas.** Se fosse do mesmo
  tamanho, não dava para saber onde a contagem começa.

### Como sair do erro

| Gesto | Efeito |
|-------|--------|
| **ON/OFF** | desliga a máquina; o ciclo recomeça do zero no próximo toque |
| **Girar o seletor** | sai do erro já no programa novo |
| Abrir a tampa | **não** sai — o erro continua piscando |
| Cortar a energia | **não** volta para o erro ao religar |

Essa última merece atenção: **o erro não vai para a flash**. Ao
religar, a máquina retoma a etapa que estava rodando. Se a falha for
real, a proteção dispara de novo em minutos; se tiver sido um susto, o
ciclo continua. É de propósito — uma placa que volta travada da queda
de luz é pior que uma que tenta mais uma vez.

Não se retoma um ciclo de cima de uma proteção que disparou: sair pelo
ON/OFF desliga a máquina, e a lavagem começa do início.

### Os dois avisos que não são código

Estes piscam **continuamente**, sem contagem e sem pausa escura, e
**se curam sozinhos** quando a causa sai. Por isso não travam a
máquina e não têm número:

| Aviso | Ritmo | O que é | Por que não trava |
|-------|-------|---------|-------------------|
| Chave de modelo inválida | pisca lento, **700 ms** | SW1..SW4 numa combinação que não existe | o técnico corrige a chave e o aviso sai sozinho; travar obrigaria a apertar ON/OFF depois de consertar |
| Falha ao gravar na flash | pisca rápido, **250 ms** | a página de estado não aceitou a gravação | a tarefa de gravação fica tentando; se a próxima der certo, o ciclo segue |

```text
Modelo inválido (700 ms)

  ▀▀▀▀▀▀_______▀▀▀▀▀▀_______▀▀▀▀▀▀_______▀▀▀▀▀▀_______ ...

Falha de flash (250 ms)

  ▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__▀▀__ ...
```

Nenhum dos dois se confunde com um código de erro: **código tem pausa
escura**, aviso contínuo não tem.

### Tudo que o painel pode estar fazendo

Uma tabela só, para quando a pergunta é "o que essa placa está
tentando me dizer?":

| O que você vê | Significa |
|---------------|-----------|
| 2 piscadas dos três LEDs e para | boot normal, logo após ligar |
| **N piscadas, pausa escura, repete** | proteção disparou — conte o N na tabela acima |
| pisca lento (700 ms) sem parar | chave de modelo inválida |
| pisca rápido (250 ms) sem parar | falha ao gravar na flash |
| só o LED de **ON/OFF** piscando | tampa aberta, ou máquina em pausa |
| LED de ON/OFF aceso fixo | ciclo rodando |
| LEDs de opção acesos | "mais secas" e/ou "enxágue duplo" ligados |
| padrão irregular a cada 301 ms | auto-teste de linha (chave no modelo 7) |
| tudo apagado | máquina desligada |

### Onde isso vive no código

O enum e os números estão em [erro.rs](src/erro.rs), um módulo **sem
nenhuma dependência de hardware** — de propósito, para os testes
poderem rodar no PC:

```rust
pub const fn piscadas(self) -> u64 {
    match self {
        Erro::Nivel => 2,
        Erro::Encher => 3,
        Erro::Drenar => 4,
    }
}
```

Para acrescentar uma proteção: dê a ela um número ainda não usado,
chame `entra_em_erro(Erro::NovoErro)` de onde a condição é detectada, e
acrescente a linha na tabela acima.

Os testes de `ferramentas/testa_logica.rs` reprovam se **dois erros
piscarem a mesma quantidade** — seriam indistinguíveis em campo —, se a
contagem começar abaixo de 2 ou passar de 6, e se algum erro ficar sem
nome. Rode-os com a receita da [seção 13](#13-compilar-e-gravar).

Para provocar cada proteção na bancada, veja o passo 12 do
[Roteiro de bancada](ROTEIRO_BANCADA.md).

---

## 8. Mapa dos arquivos

```text
src/
├── main.rs            partida do chip e as 7 tarefas
├── mcu/
│   ├── mod.rs          reexporta o PAC
│   ├── clock.rs        48 MHz e clocks de periférico
│   ├── gpio.rs         entrada, saída e modo analógico
│   ├── adc.rs          ADC de 10 bits para os botões (PA6)
│   ├── flash.rs        FMC — apagar e gravar página
│   └── wdt.rs          FWDGT de 2,8 s
├── placa.rs           mapa de pinos + Saidas / Entradas
├── modelo.rs          chave SW1..SW4 → modelo 1..8
├── encoder.rs         4 trilhas → programa 0..15
├── erro.rs            as proteções e o código piscado de cada uma
├── botoes.rs          faixas de ADC → qual botão
├── diag.rs            diagnóstico do ADC (build de bancada)
├── relogio.rs         contador de segundos do ciclo
├── agita.rs           padrões de vai-e-vem do motor
├── tabelas.rs         tempos de cada programa, por modelo
├── leituras.rs        estado dos sensores, entre tarefas
├── evento.rs          o sinal que acorda a máquina
├── maquina.rs         as 7 etapas do ciclo
├── estado_salvo.rs    retomada após queda de luz
└── tarefas.rs         as 7 tarefas do lilos

ferramentas/
├── confere_contra_cpp.py   compara as tabelas com o firmware C++
├── gera_hex.py             gera e confere o .hex de producao
├── le_adc.py               le os canais do ADC da placa por SWD
└── testa_logica.rs         testes da logica pura, rodam no PC

crates/gd32c2x1-pac/    PAC gerado por svd2rust (não editar)
svd/                    SVD oficial da GigaDevice
target-probe/           descritor do chip para o probe-rs
```

A camada `mcu/` é a **única** parte que toca em periférico. Tudo acima
dela fala em nomes (`s.bomba(true)`, `e.tampa_aberta()`) e é código
portável.

---

## 9. O que mudou em relação ao firmware C++

O comportamento é o mesmo. Onde o porte não é literal, é por um destes
motivos:

| # | C++ | Aqui | Por quê |
|---|-----|------|---------|
| 1 | `LL_mDelay()` trava o núcleo | `sleep_for().await` | nos piscas de erro e de seleção, o relógio do ciclo e o watchdog continuam andando |
| 2 | `gravaDadosFlash()` grava na hora | pedido para a tarefa `gravacao` | apagar a página trava o núcleo alguns ms; aqui isso acontece entre duas passadas |
| 3 | debounce da tampa = 250 leituras do laço livre (~10 ms) | 2 leituras de 21 ms (~42 ms) | o laço do lilos tem período fixo; mesma ordem de grandeza, com rejeição do ruído dos triacs. **Vale conferir em bancada** |
| 4 | flash virgem lida como `0xFFFFFFFF` → as três opções acendem sozinhas | `0xFFFFFFFF` = "não há estado salvo" → começa limpo | placa nova não deve ligar com opções escolhidas |
| 5 | estado na página 15 (`0x08007800`) | última página do GD32 (`0x0800FC00`) | o GD32C231 tem 64 KB e páginas de 1 KB; o `memory.x` reserva essa página |
| 6 | 24 MHz | 48 MHz | os tempos são em segundos/ms, não em ciclos |
| 7 | ADC em modo contínuo, start/stop a cada leitura | uma conversão por disparo | mesmo resultado, menos registrador para mexer |
| 8 | girar o seletor **não** zera o contador de ciclos do molho (`cicloAtual` é `static` dentro de `Molho()`) | zera | sem isso, trocar de programa no meio do molho leva o contador velho para o programa novo, que pode pular quase todo o molho. **Mudança de comportamento assumida** — se a fábrica preferir o jeito antigo, é não chamar `reinicia_molho()` |
| 9 | uma leitura do ADC vale na hora | só vale depois de **se repetir** (±20) | a tensão do divisor sobe gradualmente; uma amostra isolada no meio da subida caía na faixa de outro botão. Era o *Mais Secas* desligando a máquina |
| 10 | `VerificarMudancaEncoder` age na primeira leitura diferente | age depois de **duas leituras iguais** (81–162 ms) | oscilação de contato do seletor recomeçava o ciclo sozinha; e girar devagar disparava uma gravação na flash por posição atravessada |
| 11 | falha de flash congela a etapa, mas deixa os triacs como estavam | desliga os triacs, para os relógios e pisca o erro | o C++ só faz `return` na etapa; aqui a saída já ligada é cortada. Ao recuperar, a estabilização é refeita |
| 12 | tempo estourado pisca os LEDs e **continua tentando**; nível que cai volta a encher **para sempre** | proteção **trava** a máquina com um código contado, e a terceira queda de nível na mesma lavagem é erro | sem isso, torneira fechada deixa a máquina enchendo a noite toda, e um vazamento enche e esvazia indefinidamente. Os códigos são os mesmos das outras placas da linha. O limite de 3 recuperações está em `LIMITE_RECUPERACOES_NIVEL` |
| 13 | nível que volta durante a drenagem liga a bomba, mas **deixa o motor girando** até o nível cair de novo | corta os dois sentidos do motor na hora | o C++ só desliga o motor na passada seguinte, no ramo `reiniciaTempoAposDrenarAgua`, que depende do nível já ter caído. Até lá o cesto gira com água dentro |
| 14 | ligar a máquina pelo botão **não** zera a fase nem a contagem do molho | zera, pelo mesmo `reinicia_molho()` do item 8 | era a mesma falha do item 8 numa segunda porta de entrada: o item 8 cobriu o seletor, este cobre o ON/OFF com a máquina desligada |

Os itens 9, 10 e 11 têm a mesma raiz: **um sinal lido uma única vez não
é um sinal**. Nos três casos a correção foi exigir confirmação antes de
agir, e nos três o defeito só aparecia de vez em quando — que é o pior
tipo de defeito para achar na bancada.

### O ADC, e o que ele custou

Três dos quatro pontos que ficaram em aberto no porte eram sobre o ADC
dos botões, e a bancada resolveu todos. Vale registrar, porque dois
deram trabalho:

| Dúvida do porte | Resposta |
|-----------------|----------|
| PA6 é o canal 6? | **Sim.** No GD32C2x1 a numeração acompanha o pino: IN0..IN7 em PA0..PA7. Coincide com o `ADC1_IN6` do STM32 |
| `DRES = 0b01` é 10 bits? | **Sim.** 00 = 12, 01 = 10, 10 = 8, 11 = 6 |
| O clock do ADC podia ficar no reset? | **Não.** Era bug — veja abaixo |
| As faixas do C++ valiam no GD32? | **Não.** O mesmo divisor lê ~31 contagens abaixo — veja abaixo |

Os dois bugs que a bancada revelou, ambos invisíveis em simples
tradução do código do STM32:

1. **CK_ADC tem limite de 24 MHz.** O firmware C++ roda o núcleo a
   24 MHz e ainda divide o clock do ADC por 4. Este roda a 48 MHz e eu
   havia deixado o `ADCPSC` no valor de reset — o ADC ficava a 48 MHz,
   o dobro do limite, devolvendo valor constante.
2. **O disparo por software exige `ETSRC = 111` e `ETERC = 1`.** Sem os
   dois, escrever `SWRCST` não inicia conversão nenhuma. No STM32 isso
   não existe: `LL_ADC_REG_StartConversion` basta.

E a terceira coisa, que só a medição podia dizer: **as faixas de tensão
dos botões não são as mesmas do STM32.** Medido nesta placa, o divisor
lê cerca de **31 contagens abaixo** — o repouso dá `0` em vez de ~33, e
o botão de ON/OFF dá `789` em vez de ~820. É deslocamento, não escala:
ganho errado daria a mesma razão nos dois pontos.

O efeito do limite inferior de 7 na faixa de repouso era grave e
silencioso: o ferrolho que arma a próxima ação só arma quando a leitura
cai nessa faixa. Lendo `0`, **nunca armava**, e nenhum botão funcionava.
E só o **modelo 6** revelava isso, porque é o único que depende do botão
para retomar depois da tampa — nos outros a retomada é automática.

As faixas de [botoes.rs](src/botoes.rs) são as do C++ **deslocadas** de
31, com a mesma largura de ±25.

A primeira tentativa foi *alargar* as faixas para cobrir as duas
escalas, e isso criou um defeito pior: apertar *Mais Secas* desligava a
máquina de vez em quando. O motivo é que a tensão **sobe
gradualmente** quando o contato fecha — indo de 0 a ~851, ela atravessa
a faixa de Pausa no caminho, e com a faixa de Pausa alargada até 845
uma amostra caindo nesse instante virava Pausa. Pausa com a máquina
ligada desliga o ciclo.

Duas coisas resolveram:

* **Faixas estreitas e centradas**, para a travessia cair nas folgas
  entre elas, que não acionam nada;
* **Confirmação da leitura** em `tarefas::amostragem`: um valor só é
  publicado depois de se repetir, então a amostra solitária no meio da
  subida é descartada.

O `ferramentas/confere_contra_cpp.py` sabe dessa divergência e a trata
como esperada: mostra as duas escalas lado a lado e confere o que
precisa valer — faixas sem sobreposição, largura igual à do C++ e o
mesmo deslocamento nas três.

### A referência do ADC é a fonte, e por que isso quase não importa

O GD32C231 não tem pino VREF+ dedicado: o conversor mede **contra a
VDDA**, que é a fonte de 3,3 V da placa. Então é natural perguntar se
uma placa com 3,28 V e outra com 3,32 V não leriam valores diferentes
nos botões.

Leriam, **se a leitura não fosse ratiométrica.** Mas o divisor dos
botões é alimentado pela mesma fonte que serve de referência, e aí a
tensão da fonte cancela na conta:

```text
  leitura = Vin/Vref × 1023
          = (VDD × k) / VDD × 1023      ← VDD sai dos dois lados
          = k × 1023
```

A leitura depende só da **razão** dos resistores do divisor, não da
tensão da fonte. É a mesma razão pela qual um potenciômetro ligado
entre VDD e GND dá sempre a mesma leitura.

Se **não** fosse ratiométrica, o risco seria concreto:

| Botão | Para de responder | Responde como outro |
|-------|-------------------|---------------------|
| Pausa | 3,2% (105 mV) | 4,7% → vira Mais Secas |
| Mais secas | 2,9% | 4,3% → vira Pausa |
| Enxágue duplo | 2,7% | 5,9% → vira Mais Secas |

Reguladores de 3,3 V têm tolerância de ±1% a ±3% — dentro da
tolerância normal, portanto. Não é uma preocupação exagerada.

**E o que os dados já dizem.** O desvio de 31 contagens que medimos
nesta placa **não tem cara de problema de fonte**: diferença de fonte
apareceria como erro de **escala** (proporcional), e o que medimos é
**deslocamento**. Escala de 0,962 poria o repouso em 32; ele lê 0.

### Como conferir isso entre placas, sem voltímetro

O build de bancada lê a **referência interna de tensão** (`VREFINT`,
canal 14). Ela é uma tensão fixa do chip medida contra a VDDA, então a
leitura dela **sobe quando a fonte cai** — serve de aferição.

Anote `VREFINT` e os patamares dos botões em duas ou três placas
(`python -I ferramentas/le_adc.py`) e compare:

| VREFINT | Patamar do botão | Conclusão |
|---------|------------------|-----------|
| igual | igual | as fontes são iguais; nada a discutir |
| **muda** | igual | a fonte varia e o divisor acompanha — **ratiométrico, faixas seguras** |
| **muda** | **muda junto** | o divisor **não** acompanha a fonte: as faixas vão errar de placa para placa |

### A normalização, e a trava que a torna segura

A correção está **ligada**: `VREFINT_REFERENCIA` em
[botoes.rs](src/botoes.rs) vale **376**, medido na placa de referência
(leituras de 375, 376 e 378 em amostras seguidas — ±1,5 contagem de
ruído).

Na própria placa de referência a correção é 1:1 e não muda nada. Ela só
age em placas cujo `VREFINT` difira, que é exatamente o caso que a
motivou.

**A aferição cruzada que esse número deu de brinde.** 376 de 1023
contra a VDDA significa `VREFINT ≈ 0,367 × VDDA`. Com a referência
interna destes chips em ~1,21 V, isso põe a VDDA em **3,28 a 3,30 V** —
o nominal. Três coisas se confirmam de uma vez, por caminho
independente: o canal 14 é mesmo o `VREFINT`, a escala de 10 bits está
certa, e a fonte desta placa está no lugar.

**A trava.** A normalização é certa num caso e errada no outro: num
divisor ratiométrico a leitura já estava correta, e normalizar
*introduz* um erro do tamanho da variação da fonte. Como a dúvida só
se encerra medindo, a correção é limitada a **1,5%**
(`CORRECAO_MAXIMA_MILESIMOS`), escolhido assim:

| Limite | Pior erro introduzido | Margem da faixa | |
|--------|----------------------|-----------------|---|
| 1,0% | 9 contagens | 25 | sobra muita margem |
| **1,5%** | **14 contagens** | 25 | sobra 44% — escolhido |
| 3,0% | 28 contagens | 25 | **estouraria a faixa** |

Ou seja: **mesmo se o palpite estiver errado**, a normalização não
consegue fazer um botão deixar de ser reconhecido. Há um teste que
prova isso — simula um divisor ratiométrico com a fonte 5% fora (bem
acima da tolerância de um regulador) e confere que os três botões
continuam sendo identificados.

Se a bancada mostrar que o divisor **não** é ratiométrico, esse limite
pode subir, porque aí a correção é ganho e não risco.

Detalhe curioso: o firmware C++ de fábrica **habilitou** o canal
`VREFINT` (`ADC_CHANNEL_VREFINT` no `.ioc`) e nunca o leu. Alguém já
teve essa dúvida e deixou o gancho pronto.

Dos pontos que ficaram para a bancada:

* **As três faixas foram medidas** — essa pendência está encerrada.
  Pausa deu 789, Mais Secas 849 e Enxágue Duplo 931, contra os 789,
  851 e 931 que o deslocamento de −31 previa. O modelo acertou dentro
  de 2 contagens.
* **Debounce da tampa** — item 3 da tabela acima: 2 leituras de 21 ms
  (~42 ms) no lugar das ~250 leituras do laço livre do C++. Vale
  conferir se atende a abertura de tampa desta máquina.

O caminho que fechou isso está no
[Roteiro de bancada](ROTEIRO_BANCADA.md#modo-de-diagnóstico-dos-botões):
o build `--features diag-botoes` varre os 16 canais e pisca o número do
que se move. A referência da sequência correta do periférico é a seção
11 do **GD32C2x1 User Manual Rev1.2**.

---

## 10. Dois pinos com dois nomes

O firmware C++ dá dois nomes a dois pinos, e o porte mantém isso:

```text
  PA9 → motor_c  e  motor_pulse       (centrifugação / pulso)
  PA3 → atuador  e  valvula_amaciante
```

Não é erro de digitação: é o mesmo triac acionado por dois caminhos
lógicos do ciclo. **Mexer num mexe no outro.**

É também por isso que `desliga_todos_triacs()` não cita a válvula de
amaciante: ela divide o pino com o atuador, que já é desligado ali.

---

## 11. Mapa de pinos

Toda saída é de **lógica invertida**: nível BAIXO liga, nível ALTO
desliga (no C++, `ControlarPino(.., true)` escreve no BRR).

### Saídas

| Função | Pino | Observação |
|--------|------|------------|
| Válvula de sabão | PA2 | |
| Atuador / válvula de amaciante | PA3 | mesmo triac |
| Motor C / pulso | PA9 | mesmo triac |
| Motor A | PC6 | |
| Bomba | PB2 | |
| LED ON/OFF (pause) | PA1 | |
| LED roupas mais secas | PC14 | |
| LED enxágue duplo | PB9 | |

### Entradas

| Função | Pino | Pull | Lógica |
|--------|------|------|--------|
| Pressostato | PA5 | nenhum | BAIXO = cheio |
| Tampa eletrônica | PA4 | cima | ALTO = aberta |
| Tampa mecânica | PA7 | nenhum | ALTO = aberta |
| SW1 (modelo) | PA12 | cima | |
| SW2 (modelo) | PA11 | cima | |
| SW3 (modelo) | PB8 | cima | |
| SW4 (modelo) | PB7 | cima | |
| Encoder ENC_1 | PB4 | baixo | |
| Encoder ENC_3 | PB3 | baixo | |
| Encoder ENC_4 | PB5 | baixo | |
| Encoder ENC_5 | PB6 | baixo | |
| Botões do painel | PA6 | analógico | divisor resistivo |

Modelos 5 e 6 leem a tampa pelo sensor **mecânico** (PA7); os outros,
pelo **eletrônico** (PA4).

SWD fica em PA13 (SWDIO) e PA14 (SWCLK).

As saídas saem do boot **desligadas por construção**: `Saida::nova`
aplica o nível antes de comutar o pino para saída, para nenhum triac
piscar na partida.

---

## 12. Estado salvo na flash

A lavadora precisa voltar de onde parou quando a energia cai no meio do
ciclo. Quatro valores vão na última página de 1 KB:

| Endereço | Guarda |
|----------|--------|
| `0x0800_FC00` | etapa do ciclo (0..5) |
| `0x0800_FC20` | roupas mais secas (0/1) |
| `0x0800_FC40` | enxágue duplo (0/1) |
| `0x0800_FC60` | pausa (0/1) |

O espaçamento de `0x20` é o mesmo do C++ (`ADD_15`, `ADD_15_1`…), para
a conferência lado a lado ficar fácil.

A gravação acontece em três momentos: na troca de etapa, ao trocar de
programa e quando o painel volta ao repouso depois de um botão. Só
apaga a página quando o estado realmente mudou — a página tem vida útil
limitada. Depois de gravar, relê e confere; se não bater, marca falha e
a interface acende o erro.

Na partida, os programas de uma etapa só não retomam de uma etapa
anterior: vão direto para a sua etapa, ou já terminaram. A regra muda
por modelo, e isso vem do C++: nos modelos 3 e 5 o "só enxaguar"
recomeça mesmo que o ciclo tenha acabado; nos outros, não.

O `memory.x` reserva essa página deixando o código em **63 KB**.

---

## 13. Compilar e gravar

### Conferir contra o C++ antes de gravar (sem placa)

As tabelas e o mapa de pinos foram transcritos do firmware C++ à mão —
são centenas de números. `ferramentas/confere_contra_cpp.py` lê os dois
projetos e compara valor por valor:

```bash
python -I ferramentas/confere_contra_cpp.py
```

São 45 checagens: tempos de agitação e molho dos 6 modelos, ciclo
turbo, lavagem econômica, os 4 padrões de vai-e-vem, as fases de
drenagem e de centrifugação, o mapa das 16 posições do encoder, os 21
pinos, a chave de modelo e as faixas do ADC dos botões. Sai com código
1 se achar qualquer divergência.

Ele espera o projeto C++ em `../CONSUL_MULT_CUBE/CONSUL_MULT_V1`; se
estiver em outro lugar, passe o caminho da pasta `Core/Src` como
argumento. Rode isto depois de mexer em [tabelas.rs](src/tabelas.rs),
[placa.rs](src/placa.rs) ou [agita.rs](src/agita.rs).

### Rodar os testes da lógica (sem placa)

```bash
rustc --test --edition 2021 -O ferramentas/testa_logica.rs -o target/testes.exe
```

```bash
./target/testes.exe
```

São 19 testes sobre os quatro módulos que não dependem de hardware —
`encoder`, `botoes`, `modelo` e `tabelas` —, que concentram o que foi
transcrito à mão do C++. Cobrem as 16 posições do encoder sem
duplicata, as faixas do ADC sem sobreposição e com folga entre elas, as
sete combinações da chave de modelo, e o comportamento das tabelas nos
limites.

> **`cargo test` não funciona aqui**, e não é descuido: o alvo padrão é
> `thumbv6m-none-eabi`, que não tem a crate `test`, e compilar para o
> host falha porque o `build.rs` do lilos rejeita alvos que não são
> Cortex-M. Daí os testes rodarem pelo `rustc` direto, no mesmo padrão
> das outras ferramentas desta pasta.

### Preparar o ambiente (uma vez)

```bash
rustup target add thumbv6m-none-eabi
```

```bash
cargo install probe-rs-tools cargo-binutils
```

```bash
rustup component add llvm-tools
```

### Compilar

```bash
cargo build --release
```

### Gravar na placa (SWD por PA13/PA14)

Na primeira placa, vá pelo [Roteiro de bancada](ROTEIRO_BANCADA.md).

```bash
cargo run --release
```

O probe-rs 0.31 **não traz a família GD32C2x1** embutida. O descritor
está em `target-probe/GD32C231K8.yaml`, gerado do
`GigaDevice.GD32C2x1_DFP.1.1.0.pack` com o algoritmo oficial
`Flash/GD32C231_64K.FLM`. O caminho já está no `runner` de
`.cargo/config.toml`, e é **relativo** — rode `cargo run` a partir da
raiz do projeto.

Se um dia precisar regerar (pack novo da GigaDevice):

```bash
target-gen pack GigaDevice.GD32C2x1_DFP.1.1.0.pack target-probe/
```

### Gerar o .hex para o gravador de produção

```bash
python -I ferramentas/gera_hex.py
```

Compila o release, converte para Intel HEX e **confere o resultado** —
é nessa última parte que está o valor.

A conferência existe por um motivo concreto: a última página da flash
(`0x0800FC00`) guarda o estado do ciclo, e se a imagem crescer e invadir
essa página o gravador sobrescreve o estado, matando a retomada após
queda de energia — **sem erro de compilação e sem aviso nenhum**. O
`memory.x` reserva a página, mas um `.hex` gerado do ELF errado passaria
batido.

Ele confere, e sai com código 1 se algo não fechar:

* formato e checksum de todas as linhas;
* começa em `0x08000000`, o vetor de reset;
* termina antes de `0x0800FC00`;
* cabe nos 64 KB do chip.

E informa a folga até a página do estado — hoje **44 KB**.

O `.hex` resultante serve no `probe-rs`, no GD-Link e no **GD32
All-In-One Programmer**. Ele **não é versionado** (está no
`.gitignore`): é artefato de build, sempre regerado do código.

<details>
<summary>Gerar na mão, sem o script</summary>

O script acha o `objcopy` sozinho — `arm-none-eabi-objcopy`,
`llvm-objcopy` do PATH, ou o do componente `llvm-tools`, que não entra
no PATH. Para fazer à mão:

```bash
cargo build --release
```

```bash
arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/release/consul_mult_lilos consul_mult_lilos.hex
```

Com o `cargo-binutils` instalado, `cargo objcopy --release -- -O ihex
consul_mult_lilos.hex` faz o mesmo. Em PowerShell, sem instalar nada:

```powershell
$objcopy = Get-ChildItem (rustc --print sysroot) -Recurse -Filter llvm-objcopy.exe |
    Select-Object -First 1 -ExpandProperty FullName
& $objcopy -O ihex target/thumbv6m-none-eabi/release/consul_mult_lilos consul_mult_lilos.hex
```

Gerando à mão, passe o resultado pela conferência do script, que não
compila nem converte nesse modo:

```bash
python -I ferramentas/gera_hex.py --so-confere
```

Com um caminho depois do `--so-confere`, ele confere aquele arquivo —
serve para validar uma imagem que veio de outra pessoa ou de outra
ferramenta.

</details>

### Tamanho

```text
   text    data     bss     dec     hex
  17856       8      80   17944    4618
```

Cerca de **28% dos 63 KB** de flash disponíveis.

### Build de bancada para os botões

Se os botões do painel não responderem, há um build que **soletra nos
três LEDs, em decimal, o valor que o ADC leu no aperto** — sem
osciloscópio e sem debugger:

```bash
cargo run --release --features diag-botoes
```

Os triacs e o watchdog ficam de fora. Para o log das últimas 64
leituras e a tabela dos 16 canais, `python -I ferramentas/le_adc.py` lê
a placa por SWD. Os detalhes estão em [src/diag.rs](src/diag.rs) e no
[Roteiro de bancada](ROTEIRO_BANCADA.md#modo-de-diagnóstico-dos-botões).

---

## 14. Alvo de compilação — por que `thumbv6m` e não `thumbv8m.base`

O Cortex-M23 é ARMv8-M Baseline, cujo alvo natural seria
`thumbv8m.base-none-eabi`. O `lilos` 1.3, porém, ainda não reconhece
esse alvo no seu `build.rs` e aborta a compilação:

```text
unknown target thumbv8m.base-none-eabi, update build.rs
```

Como o **Cortex-M23 executa código ARMv6-M nativamente**, compilamos
para `thumbv6m-none-eabi`. O código roda igual; a única diferença é que
os atomics usam `critical-section` em vez das instruções exclusivas do
ARMv8-M — irrelevante num sistema de núcleo único como este.

### Consequência: a feature `rt` do PAC fica desligada

O GD32C231 tem 39 interrupções, mas o `cortex-m-rt` limita o ARMv6-M a
**32 manipuladores**. Com a feature `rt` do PAC ligada, o link falha:

```text
There can't be more than 32 interrupt handlers.
```

Por isso o `Cargo.toml` pede só a feature `critical-section` do PAC.

Na prática não se perde nada: **este firmware não usa interrupção de
periférico nenhuma** — as entradas são varridas pelas tarefas, e a
única interrupção ativa é o SysTick, que é do núcleo, não do
dispositivo.

---

## 15. O PAC

`crates/gd32c2x1-pac` é gerado por **svd2rust 0.37 + form** a partir de
`svd/GD32C231.svd`, extraído do pack oficial da GigaDevice. O conteúdo
de `src/` e `device.x` é **gerado — não edite à mão**. Para regerar,
veja [svd/README.md](svd/README.md).

Nenhum endereço ou deslocamento de registrador é escrito à mão em
`src/mcu/`: todos vêm do PAC.

---

## 16. Como mexer no firmware

### Mudar um tempo de ciclo

Tudo em [tabelas.rs](src/tabelas.rs), indexado por `[modelo][programa]`:

* `TEMPO_AGITAR` — duração da etapa AGITAR
* `CONFIG_MOLHO` — ciclos, tempo de molho e tempo de agitação
* `CICLO_TURBO` — em qual ciclo de molho a agitação sai em turbo
* `REUTILIZA_AGUA` — quais programas são lavagem econômica

As fases de drenagem e de centrifugação (pulsos de motor) estão em
`FASES_DRENAGEM` e `FASES_CENTRIFUGA`, em [maquina.rs](src/maquina.rs).

Depois de mexer, rode o conferidor da [seção 13](#13-compilar-e-gravar)
— ele vai acusar a divergência contra o C++, que é exatamente o que
você quer ver se a mudança foi intencional.

### Mudar um padrão de agitação

As quatro constantes `Padrao::*` em [agita.rs](src/agita.rs). Os
números são contagens de 21 ms.

### Mudar um pino

Só o `mod pino` de [placa.rs](src/placa.rs). O resto do firmware fala
em nomes (`s.bomba(true)`, `e.tampa_aberta()`), nunca em "PA9".

### Acrescentar uma etapa

1. Nova variante em `Etapa`, **no fim do `enum`** — o número vai para a
   flash, e mudar a ordem invalida o estado das placas em campo;
2. Um `async fn` com o prólogo da
   [seção 5](#o-prólogo-que-toda-etapa-executa);
3. O braço correspondente em `executar_etapa` e no `match` de
   `seleciona_programa_inicial`.

### Acrescentar uma entrada

1. O pino em [placa.rs](src/placa.rs) e o acessor em `Entradas`;
2. O campo em [leituras.rs](src/leituras.rs) e a leitura na tarefa de
   amostragem correspondente (81 ms, ou 21 ms se precisar de resposta
   rápida);
3. A máquina lê pelo `leituras`, nunca do pino direto.

### Acrescentar um modelo de lavadora

1. Nova combinação em [modelo.rs](src/modelo.rs);
2. Nova linha em cada tabela de [tabelas.rs](src/tabelas.rs) e
   `NUM_MODELOS` ajustado;
3. Se o modelo tiver regra própria de agitação ou de tampa, os `match
   modelo` de [agita.rs](src/agita.rs) e [modelo.rs](src/modelo.rs).

---

## 17. Referências

* Firmware C++ de fábrica — projeto CubeIDE `CONSUL_MULT_V1`, a referência deste porte
* [BWG10_LILOS](https://github.com/luisfernandomoreira/BWG10_LILOS) — projeto irmão, de onde veio a base GD32 + lilos
* [lilos](https://github.com/cbiffle/lilos) — o RTOS
* [svd2rust](https://github.com/rust-embedded/svd2rust) — gerador do PAC

---

Alpe Placas Eletrônicas — Divisão Metalúrgica Alado
