# -*- coding: utf-8 -*-
"""Le os canais do ADC da placa pelo SWD e mostra numa tabela.

Serve quando o pisca dos LEDs do build de bancada nao basta e voce quer
os numeros. Faz tudo sozinho: compila, levanta os enderecos no ELF e
chama o probe-rs.

    python -I ferramentas/le_adc.py

IMPORTANTE: use o build **dev** com a feature, porque o perfil release
apaga os simbolos e os enderecos mudam:

    cargo run --features diag-botoes

O dev tem a mesma otimizacao do release (opt-level "z" + LTO), so
mantem os simbolos. Grave esse binario, deixe a placa rodando e rode
este script. Aperte um botao do painel e rode de novo para comparar.

O canal certo e o que MUDA quando voce aperta um botao: o divisor
resistivo do painel mexe a tensao de um canal so.
"""
import io
import os
import re
import subprocess
import sys

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)

ELF = os.path.join(RAIZ, "target", "thumbv6m-none-eabi", "debug", "consul_mult_lilos")
YAML = os.path.join("target-probe", "GD32C231K8.yaml")
CHIP = "GD32C231K8"

NUM_CANAIS = 16

# O que a placa tem em cada canal do ADC. No GD32C2x1 a numeracao
# acompanha o pino: IN0..IN7 em PA0..PA7, IN8..IN10 em PB0..PB2 e
# IN12 em PA12. Os canais 13 e 14 sao sensores internos.
#
# Saber quais pinos o FIRMWARE aciona e essencial: a tensao deles muda
# por conta do proprio diagnostico, nao de botao nenhum. Sem isso o
# canal 1 (o LED de ON/OFF, que o diagnostico pisca) aparece como o
# que "mais varia" e manda procurar no lugar errado.
CANAIS = {
    0:  ("PA0",  "livre", False),
    1:  ("PA1",  "LED ON/OFF - SAIDA, o diagnostico pisca", True),
    2:  ("PA2",  "valvula de sabao - SAIDA", True),
    3:  ("PA3",  "atuador - SAIDA", True),
    4:  ("PA4",  "tampa eletronica - entrada", False),
    5:  ("PA5",  "pressostato - entrada", False),
    6:  ("PA6",  "BOTOES DO PAINEL", False),
    7:  ("PA7",  "tampa mecanica - entrada", False),
    8:  ("PB0",  "livre", False),
    9:  ("PB1",  "livre", False),
    10: ("PB2",  "bomba - SAIDA", True),
    11: ("?",    "", False),
    12: ("PA12", "SW1 - entrada", False),
    13: ("-",    "sensor de temperatura interno", False),
    14: ("-",    "referencia interna", False),
    15: ("?",    "", False),
}

# Faixas de botoes.rs, para a tabela dizer em que faixa cada valor cai.
def faixas_do_firmware():
    """Le as faixas direto de src/botoes.rs.

    Elas JA estiveram escritas aqui na mao, e deu errado: o firmware
    mudou e o script continuou reportando as faixas antigas, dizendo
    "NENHUMA" para valores que o firmware reconhecia perfeitamente.
    Fonte unica agora.
    """
    arquivo = os.path.join(RAIZ, "src", "botoes.rs")
    src = io.open(arquivo, encoding="utf-8").read()
    nomes = [
        ("repouso", "REPOUSO"),
        ("PAUSA", "FAIXA_PAUSE"),
        ("MAIS SECAS", "FAIXA_MAIS_SECAS"),
        ("ENXAGUE DUPLO", "FAIXA_ENXAGUE_DUPLO"),
    ]
    saida = []
    for rotulo, constante in nomes:
        m = re.search(
            r"const %s:\s*\(u16,\s*u16\)\s*=\s*\((\d+),\s*(\d+)\)" % constante,
            src,
        )
        if not m:
            sys.exit("nao achei a constante %s em %s" % (constante, arquivo))
        saida.append((rotulo, int(m.group(1)), int(m.group(2))))
    return saida


FAIXAS = faixas_do_firmware()


def roda(cmd, **kw):
    return subprocess.run(cmd, cwd=RAIZ, capture_output=True, text=True, **kw)


def compila():
    print("Compilando o build de bancada (dev, com simbolos)...")
    r = roda(["cargo", "build", "--features", "diag-botoes"])
    if r.returncode != 0:
        print(r.stdout)
        print(r.stderr)
        sys.exit("cargo build falhou")


def enderecos():
    """Levanta o endereco de cada simbolo DIAG_ no ELF."""
    for nm in ("arm-none-eabi-nm", "llvm-nm"):
        r = roda([nm, ELF])
        if r.returncode == 0:
            break
    else:
        sys.exit(
            "Nao achei arm-none-eabi-nm nem llvm-nm no PATH.\n"
            "O arm-none-eabi-nm vem com o toolchain ARM; o llvm-nm, com\n"
            "`rustup component add llvm-tools`."
        )

    mapa = {}
    for linha in r.stdout.splitlines():
        partes = linha.split()
        if len(partes) == 3 and partes[2].startswith("DIAG_"):
            mapa[partes[2]] = int(partes[0], 16)

    if "DIAG_CANAIS" not in mapa:
        sys.exit(
            "Nao achei os simbolos DIAG_ no ELF.\n"
            "Compilou com --features diag-botoes?"
        )
    return mapa


def le(endereco, quantos):
    """Le `quantos` valores de 16 bits a partir de `endereco`."""
    cmd = [
        "probe-rs", "read", "b16", hex(endereco), str(quantos),
        "--chip", CHIP, "--chip-description-path", YAML,
    ]
    r = roda(cmd)
    if r.returncode != 0:
        print("\nO probe-rs falhou. Rode na mao:\n  " + " ".join(cmd))
        print(r.stderr.strip())
        return None
    # A saida traz os valores em hexadecimal; pega todos.
    tokens = re.findall(r"\b[0-9a-fA-F]{1,4}\b", r.stdout.replace(hex(endereco), " "))
    valores = [int(t, 16) for t in tokens]
    return valores[:quantos] if len(valores) >= quantos else None


def faixa_de(valor):
    for nome, lo, hi in FAIXAS:
        if lo <= valor <= hi:
            return nome
    return ""


def main():
    compila()
    mapa = enderecos()

    print("\nEnderecos no ELF:")
    for nome in sorted(mapa, key=lambda n: mapa[n]):
        print("  %-18s %s" % (nome, hex(mapa[nome])))

    # Log circular das ultimas leituras do canal dos botoes.
    TAM_LOG = 64
    log = le(mapa["DIAG_LOG"], TAM_LOG)
    pos = le(mapa["DIAG_LOG_POS"], 1)
    pico = le(mapa["DIAG_PICO"], 1)

    if log and pos:
        p = pos[0] % TAM_LOG
        ordenado = log[p:] + log[:p]   # do mais antigo para o mais recente
        print("")
        print("Ultimas %d leituras do canal dos botoes (antiga -> recente):" % TAM_LOG)
        for i in range(0, TAM_LOG, 8):
            print("  " + " ".join("%4d" % v for v in ordenado[i:i + 8]))

        validos = [v for v in ordenado if v != 0]
        if validos:
            print("")
            print("  menor: %d   maior: %d   espalhamento: %d"
                  % (min(validos), max(validos), max(validos) - min(validos)))
            vistas = sorted(set(f for f in (faixa_de(v) for v in validos) if f))
            if vistas:
                print("  faixas tocadas: %s" % ", ".join(vistas))
            else:
                print("  nenhuma leitura caiu numa faixa de botoes.rs")

    if pico:
        print("")
        print("Maior valor fora do repouso: %d  (faixa: %s)"
              % (pico[0], faixa_de(pico[0]) or "nenhuma"))
        if pico[0] == 0:
            print("  Nenhum aperto registrado ainda.")

    # Patamares: um por botao, se o diagnostico ja os viu.
    NUM_PLATOS = 6
    platos = le(mapa["DIAG_PLATOS"], NUM_PLATOS)
    platos_n = le(mapa["DIAG_PLATOS_N"], 1)
    if platos and platos_n:
        n = min(platos_n[0], NUM_PLATOS)
        print("")
        print("Patamares estaveis no canal dos botoes: %d" % n)
        if n == 0:
            print("  Nenhum ainda. Aperte e SEGURE cada botao por ~1 s.")
        for i in range(n):
            v = platos[i]
            print("  %4d   faixa: %s" % (v, faixa_de(v) or "NENHUMA"))
        if n:
            print("")
            print("  O patamar baixo e o repouso. Os outros sao os botoes.")
            print("  Se algum sair como NENHUMA, me passe o numero: e so")
            print("  ajustar a faixa correspondente em src/botoes.rs.")

    # Referencia interna: afere a fonte desta placa contra outra.
    vref = le(mapa["DIAG_VREFINT"], 1) if "DIAG_VREFINT" in mapa else None
    if vref:
        print("")
        print("Referencia interna (VREFINT): %d" % vref[0])
        print("  E VREFINT/VDDA, entao SOBE quando a fonte de 3,3 V CAI.")
        print("  Anote junto com os patamares e compare entre placas:")
        print("")
        print("   VREFINT   patamar   conclusao")
        print("   igual     igual     fontes iguais; nada a discutir")
        print("   MUDA      igual     divisor acompanha a fonte: leitura")
        print("                       RATIOMETRICA, faixas seguras")
        print("   MUDA      MUDA      divisor NAO acompanha: as faixas vao")
        print("                       errar de placa para placa")
        print("")
        print("Para LIGAR a normalizacao, ponha este valor em src/botoes.rs:")
        print("")
        print("    pub const VREFINT_REFERENCIA: u16 = %d;" % vref[0])
        print("")
        print("  Use a leitura da placa de REFERENCIA — a mesma em que as")
        print("  faixas de botoes.rs foram medidas. Enquanto a constante")
        print("  estiver em 0, a normalizacao fica inerte e a leitura passa")
        print("  intocada.")

    canais = le(mapa["DIAG_CANAIS"], NUM_CANAIS)
    minimos = le(mapa["DIAG_CANAIS_MIN"], NUM_CANAIS)
    maximos = le(mapa["DIAG_CANAIS_MAX"], NUM_CANAIS)

    if canais is None or minimos is None or maximos is None:
        print("\nNao consegui ler a placa. Ela esta ligada e gravada com o")
        print("build de bancada? (`cargo run --features diag-botoes`)")
        sys.exit(1)

    print("")
    print("%-3s %-5s %7s %7s %7s %6s  %-13s %s" % (
        "ch", "pino", "agora", "min", "max", "varia", "faixa", "o que e"))
    print("-" * 96)

    candidatos = []
    for c in range(NUM_CANAIS):
        variacao = max(0, maximos[c] - minimos[c])
        pino, papel, saida = CANAIS.get(c, ("?", "", False))

        marca = ""
        if variacao >= 20:
            if saida:
                # Pino que o firmware aciona: a variacao e nossa, nao de botao.
                marca = "  (saida do firmware)"
            else:
                candidatos.append((variacao, c))
                marca = "  <== varia"

        print("%-3d %-5s %7d %7d %7d %6d  %-13s %s%s" % (
            c, pino, canais[c], minimos[c], maximos[c], variacao,
            faixa_de(canais[c]), papel, marca))

    print()
    if not candidatos:
        print("Nenhum canal variou. Aperte e solte um botao do painel")
        print("algumas vezes com a placa ligada e rode de novo.")
    else:
        candidatos.sort(reverse=True)
        variacao, canal = candidatos[0]
        print("O canal que mais varia e o %d (variacao de %d)." % (canal, variacao))
        print("Se foi um botao do painel que mexeu nele, e esse o canal do PA6:")
        print("  ajuste CANAL_BOTOES em src/mcu/adc.rs")
        if len(candidatos) > 1:
            print("\nOutros canais tambem variaram — pode ser ruido ou pino")
            print("digital chaveando por perto. O do painel e o que responde")
            print("ao aperto do botao, e com amplitude grande.")


def como_ler():
    print("")
    print("Como ler o log acima:")
    print("  - degraus estaveis, um por botao  -> divisor bom; ajuste as")
    print("    faixas em src/botoes.rs para os valores vistos")
    print("  - numeros pulando sem padrao, que mudam se voce encostar a mao")
    print("    na placa          -> entrada FLUTUANDO. Confira a solda do PA6")
    print("    e a continuidade dele ate o divisor resistivo")
    print("  - tudo igual, sem reagir a nada -> o divisor nao chega ao pino")


if __name__ == "__main__":
    main()
    como_ler()
