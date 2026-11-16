# CONSUL MULT — Firmware Máquina de Lavar · GD32C231K8T6 + Rust + lilos

Reescrita do firmware [BWG10_BWC07_RUST](https://github.com/luisfernandomoreira/BWG10_BWC07_RUST)
(STM32C031K6T6 + Embassy) para o **GD32C231K8T6** usando o RTOS **lilos**,
com uma arquitetura **baseada em eventos**.

A placa é a mesma. Só mudou o microcontrolador.

> Atendendo a máquina em campo? Vá direto ao
> **[Apêndice — Manual do técnico](#apêndice--manual-do-técnico-em-campo)**:
> proteções, códigos de erro piscados e auto-teste, autocontido e sem
> precisar do resto.

---

## 1. Por que a placa não muda

O `GD32C231K8T6` (LQFP32) é **pino a pino idêntico** ao `STM32C031K6T6` (LQFP32):

| Pino | GD32C231Kx | STM32C031KxT | | Pino | GD32C231Kx | STM32C031KxT |
|---|---|---|---|---|---|---|
| 1 | PB9 | PB9 | | 17 | PB2 | PB2 |
| 2 | PC14 | PC14 | | 18 | PA8 | PA8 |
| 3 | PC15 | PC15 | | 19 | PA9 | PA9 |
| 4 | VDD/VDDA | VDD/VDDA | | 20 | PC6 | PC6 |
| 5 | VSS/VSSA | VSS/VSSA | | 21 | PA10 | PA10 |
| 6 | PF2-NRST | PF2-NRST | | 22 | PA11 | PA11 |
| 7 | PA0 | PA0 | | 23 | PA12 | PA12 |
| 8 | PA1 | PA1 | | 24 | PA13 (SWDIO) | PA13 |
| 9 | PA2 | PA2 | | 25 | PA14-BOOT0 | PA14-BOOT0 |
| 10 | PA3 | PA3 | | 26 | PA15 | PA15 |
| 11 | PA4 | PA4 | | 27 | PB3 | PB3 |
| 12 | PA5 | PA5 | | 28 | PB4 | PB4 |
| 13 | PA6 | PA6 | | 29 | PB5 | PB5 |
| 14 | PA7 | PA7 | | 30 | PB6 | PB6 |
| 15 | PB0 | PB0 | | 31 | PB7 | PB7 |
| 16 | PB1 | PB1 | | 32 | PB8 | PB8 |

Por isso **o mapa de pinos do firmware é exatamente o mesmo do original**.

> Antes de montar, confira a alimentação e os capacitores de desacoplamento
> no AN240 (*GD32C2x1 Hardware Development Guide*): o GD32 tem um único par
> VDD/VSS no LQFP32, igual ao STM32C0, mas vale conferir o circuito de reset.

---

## 2. O microcontrolador

| | GD32C231K8T6 |
|---|---|
| Núcleo | Arm Cortex-M23 (ARMv8-M Baseline) |
| Clock | IRC48M — 12 MHz no reset, **48 MHz** depois de `mcu::clock` |
| Flash | 64 KB (64 páginas de 1 KB) |
| RAM | 12 KB |
| Watchdog | FWDGT no IRC32K, configurado para **8 s** |

**Uso atual do firmware: ~11,4 KB de flash e 100 bytes de RAM estática**
(`text` 11.600 + `data` 60 bytes; `data` 60 + `bss` 40 em RAM).
Sobra espaço de sobra para crescer.

---
## 3. O lilos — o RTOS deste firmware

### O que é

O [lilos](https://docs.rs/lilos/1.3.0/lilos/) é um RTOS mínimo em Rust, de
Cliff Biffle, para microcontroladores Cortex-M. Ele **não** é um kernel no
sentido clássico: não existe thread, não existe pilha por tarefa e não
existe troca de contexto. O que existe é um **executor de `async`/`await`**
rodando dentro de um único programa `no_std`.

| | lilos 1.3 |
|---|---|
| Tamanho do kernel | ~4 mil linhas de Rust, comentários incluídos |
| Pilhas | **uma só** — a do `main` |
| Escalonamento | cooperativo, sem preempção |
| Prioridades | nenhuma (ordem de varredura) |
| Alocação dinâmica | nenhuma — sem heap, sem `alloc` |
| Base de tempo | SysTick do núcleo, 1 ms |

Uma "tarefa" do lilos é só uma `async fn` que nunca retorna. O compilador
transforma cada `await` numa máquina de estados (um `Future`) cujo tamanho
é **conhecido em tempo de compilação**. É por isso que não se dimensiona
pilha nenhuma: desaparece o erro clássico do FreeRTOS, em que uma tarefa
estoura a pilha que lhe foi arbitrada e corrompe a memória da vizinha.

### Como as tarefas nascem

Em `main.rs`, os cinco futures são criados, **fixados** na pilha do `main`
com `pin!` e entregues ao executor:

```rust
let t_entradas = pin!(tarefas::entradas(&entradas));
let t_tempo    = pin!(tarefas::tempo());
let t_maquina  = pin!(tarefas::maquina(&painel, &atuadores, &entradas, salvo));
let t_gravacao = pin!(tarefas::gravacao(&p.fmc));
let t_watchdog = pin!(tarefas::watchdog(&p.fwdgt, &p.rcu));

lilos::exec::run_tasks(
    &mut [t_entradas, t_tempo, t_maquina, t_gravacao, t_watchdog],
    lilos::exec::ALL_TASKS,   // todas elegíveis na primeira passada
)
```

`run_tasks` tem tipo de retorno `!`: **nunca** volta. E é justamente por o
`main` nunca retornar que as tarefas podem *emprestar* (`&`) o painel, os
atuadores e as entradas — o empréstimo vale para sempre, e o verificador
de empréstimos do Rust aceita isso sem `static mut`, sem `unsafe` e sem
`Option` global.

O tipo de retorno de cada tarefa é `Infallible`, que é como se escreve
"esta função não termina" no Rust estável de hoje. Um `loop` sem `break`
em cada uma garante isso.

### O laço do executor

```
          ┌──────────────────────────────────────────────┐
          │ 1. venceu algum temporizador? → acorda       │
          │ 2. lê e zera a máscara de "acordados"        │
          │ 3. faz poll() só nas tarefas marcadas        │
          │ 4. ninguém acordado? ──────────► WFI (dorme) │
          └───────────────▲──────────────────────────┬───┘
                          │                          │
                          └──── interrupção ─────────┘
                           (SysTick de 1 ms, ou futura ISR)
```

Três coisas importantes aqui:

1. **Só as tarefas acordadas são varridas.** Cada tarefa tem um bit numa
   máscara (`WAKE_BITS`). Quem está esperando os próximos 10 ms não é nem
   tocada até a hora chegar. Não há varredura em vão.

2. **Quando ninguém tem o que fazer, o núcleo dorme.** O laço executa
   `WFI` (*Wait For Interrupt*): o Cortex-M23 para de buscar instruções e
   só volta na próxima interrupção. Neste firmware isso é o estado normal:
   entre dois tiques de 50 ms a máquina já decidiu tudo o que tinha para
   decidir, e os ~49 ms restantes o chip passa dormindo. Consumo menor e,
   o que importa mais num controle de potência, **latência previsível** —
   não há laço de espera ocupada competindo com o trabalho real.

3. **O despertador é o SysTick.** `lilos::time::initialize_sys_tick` o
   programa para 1 ms a partir da frequência real do núcleo (48 MHz, vinda
   de `mcu::clock::FREQUENCIA_HZ`). Toda a temporização do firmware — os
   10 ms das entradas, os 50 ms do tique, o 1 s do watchdog — sai daí.
   É a única interrupção ativa no firmware inteiro.

### Quem acorda quem

O lilos oferece poucas primitivas, e este firmware usa três:

| Primitiva | Onde | Para quê |
|---|---|---|
| `PeriodicGate` | `entradas`, `tempo`, `watchdog` | ritmo fixo **sem acúmulo de atraso** |
| `sleep_for` | `gravacao` | espera única antes de repetir uma gravação que falhou |
| `Notify` | `evento.rs`, `estado_salvo.rs` | acordar uma tarefa quando chega trabalho |

**`PeriodicGate` é melhor que `sleep_for` num laço.** O `sleep_for` conta a
partir do instante em que foi chamado, então o tempo de processamento da
volta anterior se soma ao período e o ritmo escorrega. O `PeriodicGate`
guarda o **próximo instante absoluto** e dorme até ele:

```rust
let mut relogio = PeriodicGate::from(Millis(10));
loop {
    relogio.next_time().await;   // acorda exatamente de 10 em 10 ms
    // ... varre os botões ...
}
```

**`Notify` é o despertador das tarefas que não têm período.** A fila de
eventos (`evento.rs`) tem um `Notify` dentro: `publica()` enfileira e
chama `notify()`, o que marca o bit da tarefa da máquina na máscara; o
executor a varre na próxima volta. Quando a fila está vazia,
`FILA.proximo().await` simplesmente não acorda — e, se mais ninguém tiver
o que fazer, o chip vai para o `WFI`.

Vale notar a escolha de `until_racy` em vez de `until` no `proximo()`:
ela assina o aviso **antes** de testar a fila. Custa alguns ciclos, e em
troca um evento publicado de dentro de uma interrupção nunca se perde na
janela entre o teste e a espera. Hoje não há ISR de periférico neste
firmware (seção 8); a margem está paga para quando houver.

### O que o cooperativo dá de graça

Uma tarefa só cede o controle **onde está escrito `.await`**. Entre dois
`await` ela roda sozinha, do começo ao fim, sem ninguém interrompendo.
Isso elimina de uma vez três classes de bug de RTOS preemptivo:

* **Não há disputa por dado compartilhado.** `maquina.trata(evento)` roda
  inteira ou não roda. Não existe o cenário "fui interrompido no meio de
  atualizar dois triacs". Daí a máquina de estados ser uma função
  **síncrona**, sem `async`, sem `Mutex`, sem `AtomicBool`.
* **Não há inversão de prioridade**, porque não há prioridade.
* **Não há travamento mútuo por semáforo**, porque não há semáforo.

O `critical_section::Mutex` que aparece na fila de eventos não está lá
para proteger uma tarefa da outra — está para o caso de uma interrupção
publicar um evento. Em troca, é preciso saber o que não fazer:

### O preço, e como ele é pago aqui

**Uma tarefa que demora atrasa todas as outras.** Não há preempção para
salvar ninguém. O firmware lida com isso de três maneiras:

* **A gravação na flash mora em outra tarefa.** Apagar/gravar uma página
  da FMC trava o núcleo por alguns milissegundos. Se isso acontecesse
  dentro de `trata()`, a máquina ficaria surda nesse intervalo. A máquina
  só *pede* a gravação; a tarefa `gravacao` a executa entre dois eventos.
* **A animação de boot é um estado, não um `sleep`.** O teste de LEDs de
  2,5 s é tocado pelos mesmos `Tique` de 50 ms, dentro do estado
  `Inicializando`. Se a tarefa da máquina parasse para animar, os eventos
  publicados nesse intervalo lotariam a fila e seriam descartados logo no
  boot.
* **O contador `perdidos` da fila é o sensor disso.** A fila tem 16 vagas
  e a máquina consome uma por volta sem nunca bloquear, então na prática
  ela vive quase vazia. Se `perdidos` sair de zero, alguma tarefa está
  demorando demais — é o primeiro lugar para olhar.

E se uma tarefa travar de vez? O executor para de rodar, a tarefa
`watchdog` para de alimentar o FWDGT e o MCU **reinicia em 8 s**
(seção 10). O travamento vira um reset, não uma máquina de lavar com um
triac ligado.

### Por que lilos e não RTIC, Embassy ou FreeRTOS

| | lilos (este) | RTIC | Embassy | FreeRTOS |
|---|---|---|---|---|
| Modelo | executor async cooperativo | interrupções com prioridade | executor async + HAL completa | threads preemptivas |
| Pilhas | 1 | 1 | 1 | uma por tarefa |
| Prioridades | — | sim (NVIC) | — | sim |
| HAL própria | não | não | sim, por família | não |
| Tamanho | mínimo | mínimo | grande | médio |

A máquina de lavar é um sistema **lento**: o evento mais rápido é um tique
de 50 ms, e o trabalho entre dois tiques é um `match` e alguns registros
de GPIO. Não há nada aqui que precise de prioridade nem de preempção —
precisar disso seria sinal de que algo está no lugar errado. O que esse
problema pede é o contrário: pouca coisa entre o código e o hardware, e um
comportamento que caiba inteiro na cabeça de quem for ler o firmware
depois.

A versão anterior (Embassy, no STM32C031) trazia a HAL da família junto.
Aqui o GD32 não tem HAL em Rust — então `src/mcu/` fala com o PAC gerado
do SVD oficial (seção 5), e do RTOS só se precisava do executor e do
relógio. O lilos dá exatamente isso e nada mais.

---


## 4. A lógica do programa: tudo gira em torno de eventos

```
 ┌───────────────┐                                  ┌──────────────────┐
 │ botões        │  10 ms   ┌──────────────────┐    │ máquina de       │
 │ sensores      │─────────▶│ tarefas::entradas│───┐│ estados          │
 │ tampa         │  debounce└──────────────────┘   ││ (maquina.rs)     │
 └───────────────┘                                 ▼│                  │
                                             ┌─────────┐   ┌──────────┐│
 ┌───────────────┐  50 ms   ┌──────────────┐ │  FILA   │──▶│  trata() ││
 │ relógio       │─────────▶│ tarefas::tempo│▶│DE EVENTOS│  └──────────┘│
 └───────────────┘          └──────────────┘ └─────────┘        │       │
                                                                └───────┘
                                                                    │
                                                                    ▼
                                                   ┌────────────────────────┐
                                                   │ triacs + LEDs do painel│
                                                   └────────────────────────┘
```

**A ideia:** quem lê hardware **publica** eventos. A máquina de estados
**consome** eventos. Ninguém mais precisa conhecer ninguém.

A máquina de estados é uma função **síncrona**: `trata(evento)`.
Não tem `async`, não tem `select!`, não tem espera escondida.
Dá para ler o comportamento inteiro da máquina de cima a baixo num arquivo só.

### As 5 tarefas

| Tarefa | Período | O que faz |
|---|---|---|
| `entradas` | 10 ms | lê botões e sensores, faz debounce, publica eventos |
| `tempo` | 50 ms | publica `Tique` e, a cada 20, `Segundo` |
| `maquina` | por evento | consome a fila e decide |
| `gravacao` | por pedido | grava o ciclo na flash |
| `watchdog` | 1 s | alimenta o FWDGT |

### Os estados da máquina

```
 Inicializando ──┐ teste dos LEDs (2,5 s)
                 ▼
            Desligada ◀──────────── Erro  (pisca o código até on/off)
             │    ▲                   ▲
      on/off │    │ on/off            │ falha de sensor
             ▼    │                   │
          Selecionando ──2 s──▶  Executando ──tampa──▶ Pausada
                 ▲                            ◀─2 s──┘
                 └─ seletor cancela e avança o programa

 AutoTeste ◀── Seletor+Nível nos primeiros 15 s (seção 12)
```

A animação de boot é um **estado**, não um `sleep` dentro da tarefa.
Se a tarefa da máquina parasse para animar, os eventos publicados nesse
meio-tempo encheriam a fila e seriam descartados logo no boot.

### Os ciclos de lavagem

Pesada e Normal encerram após agitação e molho, mantendo a água no tanque.
Rápido drena e centrifuga. Enxaguar e Centrifugar são selecionados separadamente.

| Programa | Etapas | Tempo fixo | Com enxágue extra |
|---|---|---|---|
| Pesada | 13 | 36 min | mesmo programa |
| Normal | 9 | 21 min | mesmo programa |
| Rápido | 8 | 10 min | — (sem enxágue) |
| Enxaguar | 8 | 7 min | 16 etapas, 14 min |
| Centrifugar | 6 | 7 min | — |

"Tempo fixo" não conta enchimento, drenagem e pulsos, que dependem dos
sensores (os pulsos somam ~2 min por bloco).

**O amaciante entra sempre no último enxágue:** sem a opção "enxágue
extra" ele sai junto com o primeiro (e único) enxágue; com ela ligada,
fica guardado para o segundo. Isso está nas ações `EncherEnxague` e
`EncherAmaciante`.

Para ver a sequência completa de qualquer programa, rode a ferramenta
de conferência (seção 7).

### O caminho de um evento, de ponta a ponta

Vale seguir um exemplo concreto: **a máquina está desligada e alguém
aperta o liga/desliga.**

| | O que acontece | Onde |
|---|---|---|
| 1 | A tarefa `entradas` acorda (passaram 10 ms) e lê PC14: nível BAIXO = pressionado | `tarefas.rs` |
| 2 | O filtro de ruído exige **5 leituras iguais seguidas** (50 ms). Nas quatro primeiras, nada sai | `Debounce::amostra` |
| 3 | Na quinta, o filtro devolve a mudança e a tarefa publica `BotaoPressionado(OnOff)` | `tarefas.rs` |
| 4 | `publica()` enfileira o evento e chama `notify()` — o bit da tarefa `maquina` é marcado | `evento.rs` |
| 5 | O executor, na próxima volta, varre só a tarefa `maquina`; o `.await` dela retorna com o evento | `lilos::exec` |
| 6 | `trata()` cai no braço do botão, registra o bit no espelho de botões e chama `botao_on_off()` | `maquina.rs` |
| 7 | Estava `Desligada` → vai para `Selecionando { restante: 2 s }`, acende o LED do programa e as opções | `maquina.rs` |
| 8 | O estado novo é gravado na flash (pedido para a tarefa `gravacao`) | `estado_salvo.rs` |
| 9 | A fila ficou vazia, mais ninguém tem o que fazer: **o núcleo volta a dormir** até o próximo tique | `WFI` |

Nos 2 s seguintes os `Tique` vão descontando `restante`. Se nada mudar, a
máquina confirma sozinha e chama `inicia_execucao()` — que já olha a tampa:
aberta, vai para `Pausada`; fechada, liga a primeira etapa do programa.

Repare no que **não** aparece nessa lista: nenhuma interrupção de
periférico, nenhum EXTI, nenhuma configuração de NVIC, nenhuma variável
global compartilhada e nenhum `Mutex` entre tarefas. Um botão é uma leitura
de GPIO a cada 10 ms, e a mudança viaja como dado.

### Os dois relógios da máquina

Toda a passagem de tempo entra por **dois** eventos, publicados pela mesma
base de 50 ms (então nunca saem de fase):

| Evento | Período | Quem usa |
|---|---|---|
| `Tique` | 50 ms | animação de boot, piscas do painel, o código de erro piscado, padrão A/C da agitação, pulsos do motor C, as contagens de 2 s (confirmação e religar após tampa), a espera de 10 s do câmbio |
| `Segundo` | 1 s | duração das etapas, vigilância dos sensores, tempos limite de enchimento e drenagem |

A divisão é a regra de leitura do código: **`Tique` mexe em hardware,
`Segundo` decide se a etapa acabou.**

### Como uma etapa de ciclo anda

Um programa é uma **tabela de etapas** (`programa.rs`), e cada etapa é um
par *(ação, duração)*. A máquina guarda só dois números: qual etapa
(`etapa`) e quantos segundos ela já tem (`segundos_etapa`). O que termina a
etapa depende da ação — e é aí que está o coração da lógica:

| Ação | A etapa termina quando… | Falha se… |
|---|---|---|
| `EncherSabao`, `EncherEnxague`, `EncherAmaciante` | o **sensor** do nível escolhido é atingido | passam 40 min → `Erro::Encher` (3 piscadas) |
| `Agitar`, `Molho` | o tempo da tabela se esgota | o nível cai em 5 leituras → `Erro::Nivel` (2 piscadas) |
| `MolhoEnxague` | o tempo se esgota, ou a sequência do amaciante acaba | o nível cai em 5 leituras → `Erro::Nivel` (2 piscadas) |
| `Drenar` | os **dois** sensores de nível soltam | passam 4 min 20 s → `Erro::Drenar` (4 piscadas) |
| `Pulsos` | a tabela de pulsos progressivos acaba | — |
| `DrenarExtra`, `Centrifugar`, `BombaPos`, `PausaFinal` | o tempo da tabela se esgota | — |

Ou seja: **etapa com água é governada por sensor; etapa sem água é
governada por tempo.** Uma etapa de enchimento com duração `0` na tabela
não é um erro — é o jeito de dizer "até o sensor".

Quando a etapa acaba, `avanca_etapa()` faz quatro coisas, nessa ordem:

1. incrementa o índice e zera os contadores;
2. **pula as etapas inaplicáveis** — hoje, as marcadas `ex(...)` quando o
   enxágue extra está desligado;
3. liga as saídas que a etapa nova exige (`aplica_acao_da_etapa`), ou
   encerra o ciclo se a tabela acabou;
4. **grava a etapa nova na flash**, para uma queda de luz não perder o
   ciclo (seção 11).

Vale notar uma separação deliberada: `etapa_atual()` é consulta **pura**,
nunca mexe no índice. Quem avança é `avanca_etapa()`, e só ele. Se a
consulta pudesse avançar sozinha, mudar uma opção no meio do ciclo pularia
uma etapa sem ninguém ter ligado os atuadores dela.

### As duas coisas que a máquina nunca faz

**Nunca lê um pino.** Tudo o que ela sabe das entradas chega por evento e
fica em espelhos internos (`nivel_medio`, `nivel_alto`, `tampa_aberta`,
máscara de botões de 6 bits). Ela só **escreve** — no painel e nos triacs.
É isso que permite rodar a `maquina.rs` *real* no PC, sem placa, trocando
só o módulo `placa` por um dublê (seção 7).

**Nunca espera.** Não há `sleep`, `await` nem laço de espera dentro de
`trata()`. Toda espera é um **estado com contador**: `Selecionando
{ restante }`, `Pausada { religar_em }`, `Inicializando { passo }`,
`espera_cambio`. Por isso a máquina está sempre disponível para o próximo
evento — e por isso a tampa aberta desliga os triacs no mesmo instante em
que o evento chega, em qualquer ponto do ciclo.

### Comparação com a versão Embassy

| | Embassy (anterior) | lilos + eventos (este) |
|---|---|---|
| Lógica principal | `main.rs` com `select3` aninhados, 532 linhas | `maquina.rs`, um `match` por evento |
| Ciclos de lavagem | `programas.rs`, 1685 linhas de `async` | `programa.rs`, tabelas de etapas |
| Botões | 6 tasks + EXTI + NVIC + SYSCFG | 1 tarefa que varre a 10 ms |
| Estado compartilhado | 10+ `AtomicBool` / `Signal` globais | um `struct Maquina` |
| HAL | crate `embassy-stm32` | `src/mcu/` sobre o PAC gerado do SVD |

---

## 5. Mapa dos arquivos

```
src/
├── main.rs            inicialização e lançamento das 5 tarefas
│
├── mcu/               ← a ÚNICA parte que toca em periférico
│   ├── mod.rs         reexporta o PAC + auxiliar de campo de bits
│   ├── clock.rs       12 MHz → 48 MHz + clock dos periféricos
│   ├── gpio.rs        Saida / Entrada
│   ├── wdt.rs         FWDGT (watchdog de 8 s)
│   └── flash.rs       FMC (gravação do estado do ciclo)
│
├── placa.rs           mapa de pinos da CONSUL MULT + Painel / Atuadores / Entradas
├── evento.rs          ← o coração: enum Evento + fila
├── tarefas.rs         as 5 tarefas do lilos
├── maquina.rs         a máquina de estados
├── programa.rs        os 5 ciclos de lavagem, em tabelas
├── agitacao.rs        padrão motor A / motor C
├── opcoes.rs          turbo, nível, amaciante, enxágue extra
└── estado_salvo.rs    retomada depois de queda de energia

ferramentas/
├── conferir_programas.rs   valida as tabelas de ciclo no PC
└── simular_ciclo.rs        roda a maquina.rs real no PC, ciclo completo

target-probe/
└── GD32C231K8.yaml         descritor do chip para o probe-rs

svd/
├── GD32C231.svd            SVD oficial da GigaDevice
└── README.md               como regerar o PAC

crates/gd32c2x1-pac/        GERADO por svd2rust — não editar
```

Nenhum endereço ou deslocamento de registrador é escrito à mão no
firmware: todos vêm do PAC, gerado do SVD oficial. O `src/mcu/` só
dá nomes em português às operações que a placa precisa.

---

## 6. Como mexer no firmware

### Mudar um ciclo de lavagem

Só a tabela em `programa.rs`:

```rust
const NORMAL: &[Etapa] = &[
    e(Acao::EncherSabao, 0),      // 0 = até o sensor de nível
    e(Acao::Agitar, 5 * 60),      // 5 minutos
    e(Acao::Molho, 4 * 60),
    ...
    ex(Acao::Agitar, AGITAR_ENXAGUE_S),  // `ex` = só com enxágue extra
];
```

Nenhum outro arquivo muda. Depois de mexer, confira no PC (seção 7).

Os tempos que se repetem entre programas (drenagem extra, bomba, as
centrifugações) ficam em constantes nomeadas no alto do arquivo — mudar
um deles continua sendo **uma edição só**.

### Acrescentar uma entrada nova (ex.: sensor de temperatura)

1. `placa.rs` — acrescente o pino e o campo em `Entradas`;
2. `evento.rs` — acrescente a variante em `Evento`;
3. `tarefas.rs` — publique o evento na varredura de `entradas`;
4. `maquina.rs` — trate a variante no `match` de `trata()`.

### Acrescentar uma ação nova (ex.: aquecedor)

1. `placa.rs` — o pino e o método em `Atuadores`;
2. `programa.rs` — a variante em `Acao`;
3. `maquina.rs` — um braço em `aplica_acao_da_etapa()`;
4. use a ação nas tabelas.

### Mudar um pino

Só o módulo `pino` dentro de `placa.rs`. Mais nada no firmware
menciona "PA7" ou "PB2".

---

## 7. Compilar e gravar

### Conferir os ciclos no PC (sem placa)

Duas ferramentas, nenhuma com dependência: elas incluem os arquivos de
`src/` como módulo, então testam o código de verdade, não uma cópia.

**1. As tabelas** — `ferramentas/conferir_programas.rs` lê
`src/programa.rs`, imprime cada ciclo por extenso e verifica as regras
de segurança:

```bash
rustc -O --edition 2021 ferramentas/conferir_programas.rs -o conferir && ./conferir
```

**2. A máquina de estados** — `ferramentas/simular_ciclo.rs` roda a
`src/maquina.rs` **real** no PC, com um modelo simples de tanque, e
confere o ciclo de ponta a ponta:

```bash
rustc -O --edition 2021 ferramentas/simular_ciclo.rs -o simular && ./simular
```

Só o que toca hardware (`placa`), a fila de eventos (`evento`, que
depende do lilos) e a gravação na flash (`estado_salvo`) são
substituídos por dublês dentro da ferramenta. Ela roda as 5 cadeias (uma por programa de partida) ×
enxágue extra ligado/desligado × tanque vazio/cheio = 20 ciclos, e
verifica:

* a máquina entra em execução depois do liga/desliga;
* passa por **todas** as etapas aplicáveis, em ordem, sem pular e sem repetir;
* chega ao fim da tabela;
* termina com todos os triacs desligados;
* a cadeia de programas vai até a centrifugação (seção 13);
* no fim da cadeia volta para lavagem pesada, zera as opções e fica
  **desligada** — igual ao firmware original.

Depois dos ciclos ela confere os **códigos de erro** (seção 10). Para cada
um dos três erros, força a falha — válvula aberta e nível que nunca sobe,
bomba ligada e tanque que nunca esvazia, nível que cai durante a
agitação —, espera o tempo limite estourar e lê o painel tique a tique:

* entrou no erro certo;
* os 10 LEDs piscam **juntos** (nunca um subconjunto aceso);
* a contagem dá exatamente N piscadas, três repetições seguidas;
* toda piscada tem a mesma duração;
* existem só dois tamanhos de escuro, e a pausa é ao menos 3× o intervalo
  entre piscadas;
* dois erros nunca piscam o mesmo número de vezes.

Ela imprime o traço de cada código em `▀` e `_`, do jeito que se vê na
máquina. É o único jeito prático de conferir isso: na placa, cada erro
custaria até 40 min de tempo limite para aparecer uma vez.

**O mesmo arquivo em modo de teste** — `simular_ciclo.rs` também carrega
um módulo `#[cfg(test)]` com uma varredura mais larga, que o `main()` não
faz. Na raiz do projeto, em PowerShell:

```powershell
New-Item -ItemType Directory -Force target/revisao | Out-Null
rustc --test --edition 2021 -O ferramentas/simular_ciclo.rs -o target/revisao/testes.exe
if ($LASTEXITCODE -eq 0) { & ./target/revisao/testes.exe }
```

Além dos padrões de agitação, os testes verificam 160 ciclos (5 programas
× **todas** as 16 combinações de opções × tanque vazio/cheio), a queda de
nível no molho do enxágue com e sem amaciante e nos dois níveis, a pausa e
a retomada pela tampa, e a restauração do estado salvo no boot (seção 11)
com a espera de câmbio antes da centrifugação.

A persistência física na flash e o debounce das entradas não dão para
cobrir aqui — dependem de validação na placa.

O `--edition 2021` é obrigatório: o `rustc` chamado solto usa a edição
2015, onde os `use` internos da `maquina.rs` não resolvem.

Ele checa, nos 5 programas × com e sem enxágue extra:

1. nunca enche com o tanque já cheio;
2. nunca agita nem deixa de molho com o tanque vazio;
3. nunca centrifuga com o tanque cheio;
4. todo `Centrifugar` tem um `Pulsos` antes (roupa distribuída);
5. `DrenarExtra` e `BombaPos` só depois de `Drenar`;
6. toda etapa de tempo fixo tem duração maior que zero;
7. o ciclo termina numa etapa estável.

As duas saem com código de erro se algo não bater — **rode as duas
depois de mexer numa tabela ou na máquina de estados, antes de gravar.**

### Preparar o ambiente (uma vez)

```bash
rustup target add thumbv6m-none-eabi
```

```bash
cargo install probe-rs-tools cargo-binutils
```

(`target-gen` só é necessário para regerar o descritor do chip — veja
abaixo. Ele já vem pronto no repositório.)

```bash
rustup component add llvm-tools
```

### Compilar

```bash
cargo build --release
```

### O target do probe-rs (já está no repositório)

O probe-rs 0.31 **não traz a família GD32C2x1** embutida. O descritor
está em `target-probe/GD32C231K8.yaml` — o mesmo arquivo usado nos
projetos BRM35, BRM40 e BWC07, gerado do
`GigaDevice.GD32C2x1_DFP.1.1.0.pack` com o algoritmo oficial
`Flash/GD32C231_64K.FLM`.

Não precisa gerar nada: o caminho já está no `runner` de
`.cargo/config.toml`. Ele é **relativo**, então rode `cargo run` a
partir da raiz do projeto — foi isso que causava o
`os error 3 / não pode encontrar o caminho especificado` quando o
arquivo não existia.

Se um dia precisar regerar (pack novo da GigaDevice):

```bash
target-gen pack GigaDevice.GD32C2x1_DFP.1.1.0.pack target-probe/
```

### Gravar na placa (SWD por PA13/PA14)

```bash
cargo run --release
```

### Gerar o .hex para o gravador de produção

```bash
cargo objcopy --release -- -O ihex consul_mult_lilos.hex
```

Esse `.hex` também serve para o **GD32 All-In-One Programmer** com um
GD-Link — alternativa válida se o probe-rs der problema com o gravador.

O `.hex` **não é versionado** (está no `.gitignore`): é artefato de build,
sempre regerado do código. Depois de gerar, vale conferir que ele começa em
`0x08000000` e termina antes de `0x0800FC00` — a página 63, que guarda o
estado salvo (seção 11) e não entra na imagem.

**Se der `error: no such command: objcopy`**, falta o `cargo-binutils` do
passo "Preparar o ambiente" acima. Para gerar sem instalar nada, dá para
chamar direto o `llvm-objcopy` — é o mesmo programa que o `cargo objcopy`
embrulha, e a saída é byte a byte igual. Ele não compila por conta própria,
então o `cargo build --release` vem antes. Em PowerShell, na raiz do
projeto:

```powershell
cargo build --release
$objcopy = Get-ChildItem (rustc --print sysroot) -Recurse -Filter llvm-objcopy.exe |
    Select-Object -First 1 -ExpandProperty FullName
& $objcopy -O ihex target/thumbv6m-none-eabi/release/consul_mult_lilos consul_mult_lilos.hex
```

O `Get-ChildItem` evita escrever o *host triple* na mão: o executável fica
em `<sysroot>/lib/rustlib/<triple>/bin/`, e o `<triple>` muda de máquina
para máquina. Ele vem do componente `llvm-tools`, que também está no
"Preparar o ambiente".

---

## 8. Alvo de compilação — por que `thumbv6m` e não `thumbv8m.base`

O Cortex-M23 é ARMv8-M Baseline, cujo alvo natural seria
`thumbv8m.base-none-eabi`. O `lilos` 1.3, porém, ainda não reconhece esse
alvo no seu `build.rs` e aborta a compilação:

```
unknown target thumbv8m.base-none-eabi, update build.rs
```

Como o **Cortex-M23 executa código ARMv6-M nativamente**, compilamos para
`thumbv6m-none-eabi`. O código roda igual; a única diferença é que os
atomics usam `critical-section` em vez das instruções exclusivas do ARMv8-M
— irrelevante num sistema de núcleo único como este.

Se um dia quiser o alvo nativo, basta acrescentar o braço
`"thumbv8m.base-none-eabi" => {}` ao `build.rs` do lilos e trocar a linha
`target` em `.cargo/config.toml`. É o que os projetos BRM35, BRM40 e
BWC07 usam — eles são RTIC, que não tem essa limitação.

### Consequência: a feature `rt` do PAC fica desligada

O GD32C231 tem 39 interrupções, mas o `cortex-m-rt` limita o ARMv6-M a
**32 manipuladores**. Com a feature `rt` do PAC ligada, o link falha:

```text
There can't be more than 32 interrupt handlers.
```

Por isso o `Cargo.toml` pede só a feature `critical-section` do PAC, e
o `cortex-m-rt` vai sem a feature `device`.

Na prática não se perde nada: **este firmware não usa interrupção de
periférico nenhuma** — as entradas são varridas a cada 10 ms e a única
interrupção ativa é o SysTick, que é do núcleo, não do dispositivo.

Se um dia precisar de EXTI, DMA ou timer por interrupção, aí sim vale
migrar para `thumbv8m.base` (com o lilos corrigido) e ligar o `rt`.

---

## 9. Mapa de pinos da placa

Toda saída é de **lógica invertida**: nível BAIXO liga, nível ALTO desliga.

### LEDs do painel

| Pino | Função |
|---|---|
| PA7 | LED lavagem pesada |
| PA6 | LED lavagem normal |
| PA4 | LED lavagem rápida |
| PA1 | LED enxaguar |
| PA0 | LED centrifugar |
| PA11 | LED enxágue extra |
| PA15 | LED turbo |
| PC6 | LED amaciante |
| PA3 | LED nível médio |
| PA2 | LED nível alto |

### Triacs (placa de potência)

| Pino | Função |
|---|---|
| PB2 | Motor C — centrifugação |
| PA8 | Motor A — agitação |
| PA9 | Eletrobomba — drenagem |
| PB6 | Válvula de sabão |
| PB0 | Válvula de amaciante |

### Botões — pull-up interno, pressionado = nível BAIXO

| Pino | Função |
|---|---|
| PC14 | Liga/desliga |
| PA5 | Seletor de programa (cancela o ciclo) |
| PC15 | Nível médio ↔ alto |
| PA12 | Enxágue extra |
| PB9 | Turbo |
| PA10 | Amaciante |

### Sensores

| Pino | Função | Lógica |
|---|---|---|
| PB7 | Nível médio | pull-down; ALTO = nível atingido |
| PB8 | Nível alto | pull-down; ALTO = nível atingido |
| PB1 | Tampa | sem pull; ALTO = tampa ABERTA |

---

## 10. Segurança

* **Triacs desligados no boot** — `placa::inicializa()` escreve o nível de
  repouso *antes* de configurar o pino como saída, para o pino nunca passar
  por um estado indefinido.
* **Tampa aberta desliga tudo na hora**, em qualquer estado, antes de
  qualquer outra decisão — exceto no auto-teste, que ignora a tampa de
  propósito (seção 12).
* **2 s de carência** depois que a tampa fecha, antes de religar os triacs.
* **Watchdog de 8 s** — se qualquer tarefa travar, o executor para,
  a tarefa `watchdog` para de alimentar o FWDGT e o MCU reinicia.
* **`panic` trava o núcleo** (`panic-halt`) e o FWDGT reseta em 8 s.
* **Erros travam a máquina** com tudo desligado e o painel piscando um
  código, até o usuário apertar on/off: enchimento (40 min), drenagem
  (4 min 20 s) e queda de nível (5 leituras).

### Códigos de erro piscados no painel

Não há display na BWG10, então o erro se anuncia pelo **número de
piscadas**. Os 10 LEDs piscam juntos N vezes, o painel fica escuro por
1,5 s e o código repete — para sempre, até alguém apertar liga/desliga.

| Piscadas | Erro | O que aconteceu | O que suspeitar |
|---|---|---|---|
| **2** | `Nivel` | o nível caiu sozinho numa etapa que retém água — agitação, molho ou molho do enxágue (5 leituras seguidas) | vazamento, mangueira solta, sensor de nível solto |
| **3** | `Encher` | passaram 40 min enchendo e o nível escolhido não chegou | torneira fechada, válvula de sabão, sensor de nível |
| **4** | `Drenar` | passaram 4 min 20 s drenando e o tanque não esvaziou | eletrobomba, mangueira de saída dobrada, filtro entupido |

```text
Drenar (4 piscadas):

  ▀▀▀▀▀_____▀▀▀▀▀_____▀▀▀▀▀_____▀▀▀▀▀___________________________________
  └ 250 ms acesa, 250 ms apagada ────────────┘└──── 1,5 s escuro ─────┘
  └─────────────── uma repetição: 3,5 s ────────────────────────────────┘
```

A contagem **começa em 2** de propósito: uma piscada só se confundiria
com o pisca do LED de programa, que é o aviso de tampa aberta. E a pausa
é três vezes mais longa que o intervalo entre piscadas, senão não se sabe
onde a contagem começa.

Quem define o código é `Erro::piscadas()`, em `maquina.rs` — uma linha por
erro. Para acrescentar um erro novo, dê a ele um número ainda não usado;
o simulador reprova se dois erros piscarem a mesma quantidade (seção 7).

---

## 11. Retomada após queda de energia

`estado_salvo.rs` guarda `(programa, etapa, opções, ligado)` na última página de
1 KB da flash (página 63, em `0x0800_FC00` — reservada no `memory.x`,
que deixa o código em 63 KB).

A página cabe 128 registros de 8 bytes. Em vez de apagar a página a cada
gravação, grava-se um registro novo no primeiro espaço livre e só se apaga
quando a página enche: **1 apagamento a cada 128 gravações**.

Cada registro é uma *double word* (a FMC do GD32C2x1 só grava assim):

```
palavra_baixa = 0xA5 | programa<<8 | etapa<<16 | opcoes<<24 | ligado<<28
palavra_alta  = !palavra_baixa            ← verificação
```

Ao energizar, o firmware executa o teste dos LEDs e lê o último registro válido.
Se estava ligada, retoma automaticamente o programa na etapa salva, após os 2 s
de confirmação e respeitando a tampa. O tempo da etapa recomeça.
Se foi desligada pelo botão, aguarda liga/desliga. Registros antigos, sem o bit
ligado, aguardam o botão até uma nova gravação com este firmware.

---

## 12. Auto-teste (modo de fábrica / manutenção)

Sequência de diagnóstico para conferir cada atuador isoladamente, sem
rodar um ciclo inteiro.

### Como entrar

Com a máquina **recém-energizada**, segure **SELETOR + NÍVEL** ao mesmo
tempo dentro dos **primeiros 15 s**. Pode já estar segurando na hora de
ligar na tomada. Passados os 15 s, a combinação não faz mais nada.

Os dois LEDs de nível piscam: solte os dois botões para começar.

### Os passos

| LED aceso | O que está ligado | Como sair do passo |
|---|---|---|
| Lavagem pesada | válvula de sabão | SELETOR |
| Lavagem normal | válvula de amaciante | SELETOR |
| Lavagem rápida | motor em agitação (padrão, sem turbo) | SELETOR |
| Enxaguar | eletrobomba | 60 s, ou NÍVEL para atalhar |
| Centrifugar | motor C + eletrobomba | SELETOR encerra |

Cada passo desliga tudo antes de ligar só o que vai testar, então nunca
há dois atuadores ligados por engano.

No passo da bomba o SELETOR é ignorado de propósito: são os 60 s (ou o
atalho pelo NÍVEL) que garantem ao câmbio tempo de posicionar antes do
motor C entrar.

**O botão ON/OFF sai do auto-teste a qualquer momento** e desliga tudo.
Isso é um acréscimo: no firmware original só se saía tirando da tomada.

### ⚠ O auto-teste ignora o sensor de tampa

É proposital, e é o mesmo comportamento do firmware original: o técnico
precisa ver as válvulas e o motor com a máquina aberta. Por isso o modo
só abre com um gesto de dois botões e só nos primeiros 15 s — não dá
para cair nele sem querer durante o uso normal.

---

## 13. O ciclo completo: os programas se encadeiam

Ao terminar a sua tabela, cada programa **avança para o seguinte e já
começa a rodar**. A centrifugação encerra tudo:

```
Pesada → Normal → Rapido → Enxaguar → Centrifugar → desliga
```

É o mesmo comportamento do
[firmware original](https://github.com/luisfernandomoreira/BWG10_BWC07_RUST),
onde quatro das cinco funções de programa terminam com
`SIG_CANCELAR.signal(())` (`programas.rs`, linhas 278, 450, 844 e 1441 —
todas menos `centrifugar`) e o laço `'idle` do `main.rs` trata esse sinal
como se fosse o botão seletor.

Escolher um programa no painel define **por onde o ciclo começa**, não um
programa isolado: escolher "enxaguar" roda enxágue e depois
centrifugação; escolher "centrifugar" roda só a centrifugação.

### Onde isso está escrito

A regra é declarativa, em `programa.rs`:

```rust
pub fn fim_de_ciclo(self) -> FimDeCiclo {
    match self {
        Programa::Centrifugar => FimDeCiclo::Desliga,
        _ => FimDeCiclo::Encadeia,
    }
}
```

Para um programa passar a terminar sozinho, é só devolver
`FimDeCiclo::Desliga` para ele. Quem lê isso é o `ciclo_concluido()` da
máquina de estados — o resto do firmware não muda.

### O fim da cadeia

No fim da centrifugação, igual ao original: `PausaFinal` de 5 s com tudo
desligado, triacs desligados, LED do programa apagado, o programa volta
para **lavagem pesada**, as quatro opções são **zeradas** e a máquina
fica **desligada**, esperando o on/off.

### Tempos de cada cadeia

Tempo fixo, sem contar enchimento, drenagem e pulsos (que dependem dos
sensores):

| Começando em | Cadeia | Tempo fixo |
|---|---|---|
| Pesada | Pesada → … → Centrifugar | ~81 min |
| Normal | Normal → … → Centrifugar | ~45 min |
| Rápido | Rápido → Enxaguar → Centrifugar | ~24 min |
| Enxaguar | Enxaguar → Centrifugar | ~14 min |
| Centrifugar | Centrifugar | ~7 min |

O simulador (seção 7) confere as cinco cadeias, com enxágue extra
ligado/desligado e tanque vazio/cheio — 20 ciclos — e reprova se alguma
não chegar na centrifugação ou não desligar no fim.

### Uma diferença deliberada: o auto-teste sai pelo on/off

No original só se saía do auto-teste tirando a máquina da tomada.
Aqui o botão liga/desliga encerra o modo a qualquer momento e desliga
tudo (seção 12).

---

## 14. O que ainda falta

A máquina já roda com este firmware e se comporta bem em ciclo
completo. O que ainda merece atenção:

* **A vigilância de nível no molho do enxágue é nova.** É a única etapa
  cujo comportamento mudou: antes ela não podia gerar erro. Com amaciante
  ligado, fica 32 min 40 s segurando água, e é o caso que vale rodar até
  o fim depois de gravar. Um erro de nível (2 piscadas) ali sem vazamento
  real significa que o flutuador oscila com a água parada — o ajuste é
  `LIMITE_NIVEL_CAIU`, em `programa.rs`.
* **A persistência na flash e o debounce** não são cobertos pelas
  conferências no PC (seção 7). A retomada dá para testar tirando da
  tomada no meio de um ciclo; o debounce só aparece com uso.

---

## 15. Referências

* [GD32C2x1 User Manual Rev1.0](https://download.gigadevice.com/User_Manual/GD32C2x1_User_Manual_Rev1.0.pdf) — registradores (RCU §4, GPIO §6, FMC §2, FWDGT §12)
* [GD32C2x1 AddOn (pack com SVD e FLM)](https://www.gd32mcu.com/en/download/7?kw=GD32C2) — origem do PAC e do target do probe-rs
* [GD32C231xx Datasheet Rev1.7](https://www.gd32mcu.com/data/documents/datasheet/GD32C231xx_Datasheet_Rev1.7.pdf) — pinagem LQFP32 e mapa de memória
* [AN240 — GD32C2x1 Hardware Development Guide](https://www.gd32mcu.com/data/documents/applicationNote/AN240%20GD32C2x1%20Hardware%20Development%20Guide_Rev1.0.pdf)
* [lilos 1.3 — documentação](https://docs.rs/lilos/1.3.0/lilos/)
* [Firmware original (STM32 + Embassy)](https://github.com/luisfernandomoreira/BWG10_BWC07_RUST)

---

# Apêndice — Manual do técnico em campo

As seções acima são para quem mexe no firmware. Este apêndice é para quem
atende a máquina: o que o firmware trava por segurança, como ler o código
piscado no painel e como rodar o auto-teste. É autocontido — dá para
imprimir só ele.

A máquina não tem display. **Todo diagnóstico sai pelos 10 LEDs do painel.**

## A.1 Proteções

Todas agem sozinhas. Três param a máquina e piscam um código; as outras só
desligam saídas.

| Proteção | O que faz | Tempo |
|---|---|---|
| Triacs desligados no boot | escreve o nível de repouso no pino antes de configurá-lo como saída | imediato |
| Tampa aberta | desliga tudo na hora, em qualquer estado | imediato |
| Religamento após fechar a tampa | espera antes de reenergizar; o tempo da etapa fica congelado | 2 s |
| Confirmação no boot | após queda de energia, espera antes de retomar o ciclo salvo | 2 s |
| Troca de sentido do motor | retira o sentido anterior antes de energizar o outro | imediato |
| Espera de câmbio | antes de `Pulsos` e `Centrifugar`, dá tempo ao câmbio de posicionar | 10 s |
| Não encheu | trava e pisca | 40 min |
| Não esvaziou | trava e pisca | 4 min 20 s |
| Nível caiu sozinho | trava e pisca, em qualquer etapa que retém água | 5 s, após 3 s de carência |
| Watchdog | se qualquer tarefa travar, o MCU reinicia sozinho | 8 s |
| `panic` | trava o núcleo e o watchdog reseta | 8 s |

Dois detalhes que mudam o diagnóstico:

* **A carência de 3 s** no início de cada etapa existe porque o flutuador
  balança quando a água entra. Antes disso o nível não é cobrado.
* **As 5 leituras são consecutivas.** Qualquer segundo com o sensor de
  volta zera a contagem. Erro de nível significa 5 segundos inteiros sem
  sensor, não uma oscilação.

Erro travado só sai apertando **ON/OFF**. Não precisa tirar da tomada, e o
ciclo salvo volta no próximo boot.

## A.2 Códigos de erro

Os **10 LEDs piscam juntos**, N vezes, e o código repete para sempre até
alguém apertar ON/OFF. Conte as piscadas de um grupo.

| Piscadas | Erro | O que aconteceu | O que suspeitar, na ordem |
|---|---|---|---|
| **2** | Nível | o nível caiu sozinho numa etapa que retém água — agitação, molho ou molho do enxágue (5 leituras seguidas) | vazamento; mangueira solta; sensor de nível solto ou com mau contato |
| **3** | Encher | passaram 40 min enchendo e o nível escolhido não chegou | torneira fechada ou com pressão baixa; válvula de sabão; sensor de nível |
| **4** | Drenar | passaram 4 min 20 s drenando e o tanque não esvaziou | eletrobomba; mangueira de saída dobrada; filtro entupido |

Os três códigos no tempo — cada caractere vale 50 ms:

```text
2 piscadas · Nível    ▀▀▀▀▀_____▀▀▀▀▀___________________________________            repete a cada 2,5 s
3 piscadas · Encher   ▀▀▀▀▀_____▀▀▀▀▀_____▀▀▀▀▀___________________________________       repete a cada 3,0 s
4 piscadas · Drenar   ▀▀▀▀▀_____▀▀▀▀▀_____▀▀▀▀▀_____▀▀▀▀▀___________________________________  repete a cada 3,5 s
```

O que separa um grupo do seguinte é o **escuro longo de 1,5 s** — três
vezes maior que o intervalo entre piscadas, justamente para se saber onde
a contagem começa. Se não der para contar de primeira, espere o escuro
longo e conte do início.

A contagem **começa em 2 de propósito**: uma piscada só se confundiria com
o pisca do LED de programa, que é o aviso de tampa aberta. Então:

* **um LED piscando sozinho** = tampa aberta, não é erro;
* **os 10 juntos** = erro travado, conte as piscadas.

Se as piscadas tiverem durações diferentes entre si, se o escuro longo não
aparecer, ou se só um subconjunto dos LEDs acender, não é código de erro —
suspeite da fiação do painel ou da placa.

## A.3 Auto-teste da placa

Liga **um atuador por vez**, com a máquina aberta, para conferir cada um
isoladamente sem rodar um ciclo inteiro. Cada passo desliga tudo antes de
ligar o seu, então nunca há dois atuadores energizados por engano.

### Como entrar

1. Com a máquina **recém-energizada e desligada**, segure **SELETOR +
   NÍVEL** juntos dentro dos **primeiros 15 s**. Pode já estar segurando
   na hora de ligar na tomada.
2. Os **dois LEDs de nível piscam** — é a confirmação de que o modo abriu.
3. **Solte os dois botões.** O primeiro passo só começa depois disso, para
   o avanço não acontecer sozinho.

Passados os 15 s, ou com a máquina em ciclo, a combinação é ignorada. Não
dá para cair no modo sem querer durante o uso normal.

### Os cinco passos

| LED aceso | O que liga | Como sair do passo |
|---|---|---|
| Lavagem pesada | válvula de sabão | SELETOR |
| Lavagem normal | válvula de amaciante | SELETOR |
| Lavagem rápida | motor agitando (padrão normal, sem turbo) | SELETOR |
| Enxaguar | eletrobomba | 60 s, ou NÍVEL para atalhar |
| Centrifugar | motor C **+** eletrobomba | SELETOR encerra o modo |

Nos três primeiros passos e no último **o tempo não corre**: o atuador fica
ligado até apertar o botão. Só o passo da bomba tem tempo próprio.

No passo da bomba o **SELETOR é ignorado de propósito**: são os 60 s (ou o
atalho pelo NÍVEL) que garantem ao câmbio tempo de posicionar antes do
motor C entrar no passo seguinte.

**ON/OFF sai do auto-teste a qualquer momento** e desliga tudo.

> ⚠ **O auto-teste ignora o sensor de tampa.** É proposital — o técnico
> precisa ver as válvulas e o motor com a máquina aberta. Por isso o modo
> só abre com gesto de dois botões e só nos primeiros 15 s. Com a máquina
> aberta e o motor energizado, **não ponha a mão no tanque.**

## A.4 Como testar

Em campo você está testando **a máquina**, não o firmware: que a válvula
abre, que a bomba puxa, que o sensor responde. A lógica dos códigos de erro
já é conferida no PC antes de cada gravação (seção 7).

### Confira no auto-teste

Um item por passo. Se algum falhar, o problema está naquele atuador ou na
fiação dele — não na lógica.

- [ ] **Válvula de sabão** (LED lavagem pesada): entra água pelo compartimento de sabão
- [ ] **Válvula de amaciante** (LED lavagem normal): entra água pelo compartimento de amaciante
- [ ] **Motor agitando** (LED lavagem rápida): o tambor vai para um lado, pausa, vai para o outro — nunca os dois sentidos juntos
- [ ] **Eletrobomba** (LED enxaguar): a bomba puxa e sai água pela mangueira de saída
- [ ] **Motor C + bomba** (LED centrifugar): o tambor gira num sentido só, com a bomba ligada
- [ ] **ON/OFF** encerra e desliga tudo em qualquer passo

Um atuador que não liga em nenhum passo, mas os outros ligam, aponta o
triac ou o fio dele. Nada ligando em passo nenhum aponta a alimentação da
placa de potência.

### Provocar cada erro

| Erro | Como provocar | Espera | O que deve acontecer |
|---|---|---|---|
| **Encher** (3 piscadas) | feche a torneira e inicie qualquer programa | 40 min | trava, desliga tudo, pisca 3 |
| **Drenar** (4 piscadas) | com água no tanque, dobre a mangueira de saída e deixe chegar na drenagem | 4 min 20 s | trava, desliga tudo, pisca 4 |
| **Nível** (2 piscadas) | numa etapa com água parada, desligue o conector do sensor de nível | 5 s a 8 s | trava, desliga tudo, pisca 2 |

O teste do nível é o único rápido. Os outros dois são tempos limite longos
de propósito — uma torneira com pressão baixa não pode virar erro. Na
prática, confirme os dois longos uma vez na bancada e em campo confie no
código piscado.

Depois de cada teste, **aperte ON/OFF** para destravar. Desfaça a falha
antes de rodar um ciclo de verdade.

> ⚠ Provocar erro de propósito mexe numa placa **energizada na rede**.
> Prefira sempre o conector do sensor e a mangueira — nunca os terminais
> dos triacs. Com o motor energizado, não ponha a mão no tanque.

### O que o teste em campo não cobre

Duas coisas só aparecem com uso prolongado:

* a gravação do estado na memória, que faz a máquina retomar o ciclo
  depois de uma queda de energia (seção 11);
* o filtro de ruído dos botões e sensores.

Para conferir a retomada, basta tirar da tomada no meio de um ciclo e
religar: a máquina deve voltar na mesma etapa, após 2 s de confirmação e
respeitando a tampa.
