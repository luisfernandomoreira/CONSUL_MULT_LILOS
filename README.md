# CONSUL MULT — Firmware Máquina de Lavar · GD32C231K8T6 + Rust + lilos

Porte do firmware C++ de fábrica (`CONSUL_MULT_CUBE/CONSUL_MULT_V1`,
STM32C031K6T6 + HAL/LL) para o **GD32C231K8T6** em **Rust** com o RTOS
**lilos**.

A placa é a mesma. Só mudou o microcontrolador e a linguagem.

> **Vai gravar a primeira placa?** Siga o
> **[Roteiro de bancada](ROTEIRO_BANCADA.md)** — 12 passos, do teste de
> continuidade ao ciclo completo, com o que medir em cada pino e o que
> cada ritmo de pisca dos LEDs quer dizer.

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
máquina é a **chave SW1..SW4** da placa, lida no boot e reconferida a
cada etapa do ciclo:

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

A ação só acontece na transição "repouso → botão apertado", então um
botão segurado não repete.

### Três LEDs

| LED | Pino | O que mostra |
|-----|------|--------------|
| ON/OFF (pause) | PA1 | aceso: rodando · piscando: tampa aberta |
| Roupas mais secas | PC14 | opção ligada |
| Enxágue duplo | PB9 | opção ligada |

Os três piscando juntos = erro (modelo inválido ou tempo estourado).

---

## 4. As sete etapas do ciclo

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

| Etapa | O que faz |
|-------|-----------|
| **ENCHER** | abre a válvula de sabão até o pressostato acusar nível. Limite: 50 min. No modelo 1, programas 12 e 13, pulsa o motor enquanto enche |
| **AGITAR** | agita pelo tempo da tabela do modelo/programa |
| **MOLHO** | alterna ciclos de parado e agitando; o último ciclo sai em turbo |
| **ENXAGUAR** | três subetapas: drenar → entrar água → agitar. Com enxágue duplo, repete |
| **CENTRIFUGAR** | drena, pulsa o motor para soltar a roupa e centrifuga 5, 6, 10 ou 12 min conforme modelo e "mais secas" |
| **DESLIGAR** | apaga tudo, para os relógios e zera o estado do ciclo |
| **TESTE** | auto-teste de linha: válvula → agita → atuador+bomba → centrifuga, avançando a cada giro do encoder |

### O que para o ciclo

| Condição | Efeito |
|----------|--------|
| Tampa aberta | triacs desligados, relógio parado, LED de ON/OFF piscando |
| Pausa (ON/OFF) | idem, até apertar de novo |
| Seletor girado | recomeça no programa novo |
| Modelo inválido | LEDs piscando, nada liga |
| Tempo estourado | LEDs piscando (não encheu ou não drenou no prazo) |
| Lavagem econômica | pausa na entrada do enxágue até a tampa abrir |

Ao fechar a tampa, os modelos 1–5 e 7 retomam automaticamente 2 s
depois; o **modelo 6** exige o ON/OFF.

### Os dois relógios

| Relógio | Período | Para quê | No C++ |
|---------|---------|----------|--------|
| [relogio.rs](src/relogio.rs) | 1 s | duração das etapas; **pode ser parado** | TIM14 |
| [agita.rs](src/agita.rs) | 21 ms | cadência do vai-e-vem do motor | TIM16 |

O relógio de 1 s mantém três contadores: `total_segundos` (a etapa
decide quando zerar), `delay_5` (satura em 5 — atraso de estabilização
elétrica) e `delay_25` (satura em 25 — atraso longo da drenagem).

### A agitação

O agitador alterna entre dois sentidos numa cadência que define a
força da lavagem:

```text
  contador:  0 ....... tempo_a ..... c_inicio ... c_fim ... reset
  motor:     [--- A ---]  (parado)   [---- C ----] (parado)   ↺
```

| Padrão | A | C início | C fim | reset |
|--------|---|----------|-------|-------|
| Padrão | 13 | 35 | 48 | 70 |
| Delicado | 12 | 74 | 86 | 147 |
| Turbo | 16 | 27 | 43 | 54 |
| Turbo delicado | 12 | 42 | 54 | 84 |

Qual padrão sai em cada momento depende do modelo, do programa e da
etapa — está todo em [agita.rs](src/agita.rs).

---

## 5. Como o firmware está montado

### As sete tarefas do lilos

| Tarefa | Período | O que faz | No C++ |
|--------|---------|-----------|--------|
| `tempo` | 1 s | conta os segundos do ciclo | ISR do TIM14 |
| `agitacao` | 21 ms | move o contador do vai-e-vem | ISR do TIM16 |
| `tampa` | 21 ms | filtra os dois sensores de tampa | `getLeituraTampa` |
| `amostragem` | 81 ms | encoder, botões (ADC), pressostato | TIM3 |
| `maquina` | por sinal | executa a etapa atual | `while(1)` do `main` |
| `gravacao` | por pedido | grava o estado na flash | `gravaDadosFlash` |
| `watchdog` | 1 s | alimenta o FWDGT (2,8 s) | `LL_IWDG_ReloadCounter` |

### Por que não há fila de eventos

O firmware C++ é um laço que lê o hardware e chama a etapa atual, sem
parar. Aqui o papel do laço é da tarefa `maquina`, que em vez de girar
à toa espera o sinal de [evento.rs](src/evento.rs).

Vários sinais entre duas passadas valem um só, e isso é de propósito:
o que a máquina lê — `relogio`, `agita` e `leituras` — é **estado**,
não mensagem. Uma fila de eventos aqui só criaria a chance de um tique
se perder por fila cheia, e tique perdido é tempo de ciclo perdido.

A tarefa mais rápida é a de 21 ms, então é ela que dá a cadência: a
etapa atual é reavaliada a cada 21 ms.

### Estado compartilhado

Três módulos ficam em `static` porque quem escreve e quem lê são
tarefas diferentes — exatamente o papel das variáveis `volatile
static` que as ISRs do C++ atualizam:

* [relogio.rs](src/relogio.rs) — os contadores de tempo
* [agita.rs](src/agita.rs) — o contador do vai-e-vem
* [leituras.rs](src/leituras.rs) — o que a amostragem viu

Em `leituras`, dois valores merecem atenção:

* `nivel_agua` já vem **invertido e filtrado**: o pressostato em nível
  baixo significa cheio, e a média de 10 amostras evita que a
  oscilação da água faça o ciclo pular de etapa.
* `encoder_mudou` é **pegajoso**: fica marcado até a máquina
  consumi-lo, para a troca de programa não se perder porque a máquina
  estava ocupada piscando um LED.

### Mapa dos arquivos

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
├── relogio.rs         contador de segundos do ciclo
├── agita.rs           padrões de vai-e-vem do motor
├── tabelas.rs         tempos de cada programa, por modelo
├── leituras.rs        estado dos sensores, entre tarefas
├── evento.rs          o sinal que acorda a máquina
├── maquina.rs         as 7 etapas do ciclo
├── estado_salvo.rs    retomada após queda de luz
└── tarefas.rs         as 7 tarefas do lilos
```

---

## 6. O que mudou em relação ao firmware C++

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

---

## 7. Dois pinos com dois nomes

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

## 8. Mapa de pinos

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

---

## 9. Estado salvo na flash

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

A gravação só apaga a página quando o estado realmente mudou — a
página tem vida útil limitada. Depois de gravar, relê e confere; se não
bater, marca falha e a interface acende o erro.

O `memory.x` reserva essa página deixando o código em **63 KB**.

---

## 10. Compilar e gravar

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

---

## 11. Alvo de compilação — por que `thumbv6m` e não `thumbv8m.base`

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

## 12. O PAC

`crates/gd32c2x1-pac` é gerado por **svd2rust 0.37 + form** a partir de
`svd/GD32C231.svd`, extraído do pack oficial da GigaDevice. O conteúdo
de `src/` e `device.x` é **gerado — não edite à mão**. Para regerar,
veja [svd/README.md](svd/README.md).

Nenhum endereço ou deslocamento de registrador é escrito à mão em
`src/mcu/`: todos vêm do PAC.

---

## 13. Como mexer no firmware

### Mudar um tempo de ciclo

Tudo em [tabelas.rs](src/tabelas.rs), indexado por `[modelo][programa]`:

* `TEMPO_AGITAR` — duração da etapa AGITAR
* `CONFIG_MOLHO` — ciclos, tempo de molho e tempo de agitação
* `CICLO_TURBO` — em qual ciclo de molho a agitação sai em turbo
* `REUTILIZA_AGUA` — quais programas são lavagem econômica

As fases de drenagem e de centrifugação (pulsos de motor) estão em
`FASES_DRENAGEM` e `FASES_CENTRIFUGA`, em [maquina.rs](src/maquina.rs).

### Mudar um padrão de agitação

As quatro constantes `Padrao::*` em [agita.rs](src/agita.rs).

### Mudar um pino

Só o `mod pino` de [placa.rs](src/placa.rs). O resto do firmware fala
em nomes (`s.bomba(true)`, `e.tampa_aberta()`), nunca em "PA9".

### Acrescentar um modelo de lavadora

1. Nova combinação em [modelo.rs](src/modelo.rs);
2. Nova linha em cada tabela de [tabelas.rs](src/tabelas.rs) e
   `NUM_MODELOS` ajustado;
3. Se o modelo tiver regra própria de agitação ou de tampa, os `match
   modelo` de [agita.rs](src/agita.rs) e [modelo.rs](src/modelo.rs).

---

## 14. Referências

* [Firmware C++ de fábrica](../CONSUL_MULT_CUBE/CONSUL_MULT_V1) — a referência deste porte
* [BWG10_LILOS](https://github.com/luisfernandomoreira/BWG10_LILOS) — projeto irmão, de onde veio a base GD32 + lilos
* [lilos](https://github.com/cbiffle/lilos) — o RTOS
* [svd2rust](https://github.com/rust-embedded/svd2rust) — gerador do PAC
