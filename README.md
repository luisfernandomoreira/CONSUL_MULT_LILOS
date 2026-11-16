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
| **Referência** | [7. Mapa dos arquivos](#7-mapa-dos-arquivos) · [8. O que mudou em relação ao C++](#8-o-que-mudou-em-relação-ao-firmware-c) · [9. Dois pinos com dois nomes](#9-dois-pinos-com-dois-nomes) · [10. Mapa de pinos](#10-mapa-de-pinos) · [11. Estado salvo na flash](#11-estado-salvo-na-flash) |
| **Trabalhar** | [12. Compilar e gravar](#12-compilar-e-gravar) · [13. Alvo de compilação](#13-alvo-de-compilação--por-que-thumbv6m-e-não-thumbv8mbase) · [14. O PAC](#14-o-pac) · [15. Como mexer no firmware](#15-como-mexer-no-firmware) |

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
    7 ..  60   nenhum botão apertado (repouso)
  795 .. 845   Pausa / ON-OFF
  857 .. 907   Roupas Mais Secas
  937 .. 987   Enxágue Duplo
```

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
| pisca lento (700 ms), sem parar | modelo inválido |
| pisca rápido (250 ms), sem parar | tempo de etapa estourou |
| padrão irregular de 301 ms | auto-teste rodando |

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
[seção 13](#13-alvo-de-compilação--por-que-thumbv6m-e-não-thumbv8mbase).

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

Os dois sentidos **nunca** ficam ligados juntos: ligar um desliga o
outro antes, na mesma função ([agita.rs](src/agita.rs)).

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
| **Tempo estourado** | LEDs piscando 250 ms | não volta sozinho |
| **Lavagem econômica** | pausa na entrada do enxágue | abrir a tampa |

Dois detalhes que não são óbvios:

* **Abrir a tampa cancela a lavagem econômica.** Faz sentido: abrir a
  tampa é exatamente o gesto de quem tirou a água e quer que o ciclo
  siga.
* **O modelo 6 exige confirmação.** Nos outros modelos, fechar a tampa
  retoma o ciclo sozinho 2 s depois. No 6, fechar não basta: o LED de
  ON/OFF fica piscando até alguém apertar o botão. É uma exigência
  daquela lavadora, não uma inconsistência.

Os limites de tempo são:

| Etapa | Limite | O que indica |
|-------|--------|--------------|
| ENCHER (e entrada de água do enxágue) | 50 min | não está entrando água |
| Drenagem e centrifugação | 6 min | não está saindo água |

---

## 7. Mapa dos arquivos

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
└── confere_contra_cpp.py   compara as tabelas com o firmware C++

crates/gd32c2x1-pac/    PAC gerado por svd2rust (não editar)
svd/                    SVD oficial da GigaDevice
target-probe/           descritor do chip para o probe-rs
```

A camada `mcu/` é a **única** parte que toca em periférico. Tudo acima
dela fala em nomes (`s.bomba(true)`, `e.tampa_aberta()`) e é código
portável.

---

## 8. O que mudou em relação ao firmware C++

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
| 8 | girar o seletor **não** zera o contador de ciclos do molho (`cicloAtual` é `static` dentro de `Molho()`) | zera | sem isso, trocar de programa no meio do molho leva o contador velho para o programa novo, que pode pular quase todo o molho. **Mudança de comportamento assumida** — se a fábrica preferir o jeito antigo, é remover três linhas de `selecionar_nova_lavagem` |

### Pontos a conferir em bancada

1. **PA6 → canal 6 do ADC.** Vem do `ADC1_IN6` do STM32 (mesma
   pinagem), mas confirme no User Manual do GD32C231 antes de confiar
   nas faixas dos botões.
2. **Resolução do ADC.** `DRES = 0b01` assume a codificação usual da
   GigaDevice (00 = 12 bits, 01 = 10 bits). Se as faixas saírem
   deslocadas, é aqui.
3. **Clock do ADC.** Ficou no valor de reset, compensado pelo tempo de
   amostragem mais longo. Se a leitura vier instável, veja
   `RCU_CFG1.ADCSEL/ADCPSC`.
4. **Debounce da tampa** (item 3 da tabela acima).

Os três primeiros afetam só os botões do painel — o ciclo, a tampa e o
pressostato não dependem do ADC. O
[Roteiro de bancada](ROTEIRO_BANCADA.md) tem um passo para cada um.

---

## 9. Dois pinos com dois nomes

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

## 10. Mapa de pinos

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

## 11. Estado salvo na flash

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

## 12. Compilar e gravar

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
cargo objcopy --release -- -O ihex consul_mult_lilos.hex
```

Esse `.hex` também serve para o **GD32 All-In-One Programmer** com um
GD-Link. Ele **não é versionado**: é artefato de build. Depois de
gerar, vale conferir que começa em `0x08000000` e termina antes de
`0x0800FC00` — a página do estado salvo não entra na imagem.

**Se der `error: no such command: objcopy`**, falta o `cargo-binutils`.
Para gerar sem instalar nada, em PowerShell na raiz do projeto:

```powershell
cargo build --release
$objcopy = Get-ChildItem (rustc --print sysroot) -Recurse -Filter llvm-objcopy.exe |
    Select-Object -First 1 -ExpandProperty FullName
& $objcopy -O ihex target/thumbv6m-none-eabi/release/consul_mult_lilos consul_mult_lilos.hex
```

### Tamanho

```text
   text    data     bss     dec     hex
  17856       8      80   17944    4618
```

Cerca de **28% dos 63 KB** de flash disponíveis.

### Build de bancada para os botões

Se os botões do painel não responderem, há um build que troca a máquina
de estados por um diagnóstico que descobre **em que escala o ADC está
lendo** e mostra nos três LEDs — sem osciloscópio e sem debugger:

```bash
cargo run --release --features diag-botoes
```

Os triacs ficam desligados o tempo todo. A tabela de padrões está em
[src/diag.rs](src/diag.rs) e no
[Roteiro de bancada](ROTEIRO_BANCADA.md#modo-de-diagnóstico-dos-botões).

---

## 13. Alvo de compilação — por que `thumbv6m` e não `thumbv8m.base`

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

## 14. O PAC

`crates/gd32c2x1-pac` é gerado por **svd2rust 0.37 + form** a partir de
`svd/GD32C231.svd`, extraído do pack oficial da GigaDevice. O conteúdo
de `src/` e `device.x` é **gerado — não edite à mão**. Para regerar,
veja [svd/README.md](svd/README.md).

Nenhum endereço ou deslocamento de registrador é escrito à mão em
`src/mcu/`: todos vêm do PAC.

---

## 15. Como mexer no firmware

### Mudar um tempo de ciclo

Tudo em [tabelas.rs](src/tabelas.rs), indexado por `[modelo][programa]`:

* `TEMPO_AGITAR` — duração da etapa AGITAR
* `CONFIG_MOLHO` — ciclos, tempo de molho e tempo de agitação
* `CICLO_TURBO` — em qual ciclo de molho a agitação sai em turbo
* `REUTILIZA_AGUA` — quais programas são lavagem econômica

As fases de drenagem e de centrifugação (pulsos de motor) estão em
`FASES_DRENAGEM` e `FASES_CENTRIFUGA`, em [maquina.rs](src/maquina.rs).

Depois de mexer, rode o conferidor da [seção 12](#12-compilar-e-gravar)
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

## 16. Referências

* Firmware C++ de fábrica — projeto CubeIDE `CONSUL_MULT_V1`, a referência deste porte
* [BWG10_LILOS](https://github.com/luisfernandomoreira/BWG10_LILOS) — projeto irmão, de onde veio a base GD32 + lilos
* [lilos](https://github.com/cbiffle/lilos) — o RTOS
* [svd2rust](https://github.com/rust-embedded/svd2rust) — gerador do PAC

---

Alpe Placas Eletrônicas — Divisão Metalúrgica Alado
