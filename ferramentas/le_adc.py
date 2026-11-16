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

# Faixas de botoes.rs, para a tabela dizer em que faixa cada valor cai.
FAIXAS = [
    ("repouso", 7, 60),
    ("PAUSA", 795, 845),
    ("MAIS SECAS", 857, 907),
    ("ENXAGUE DUPLO", 937, 987),
]


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

    canais = le(mapa["DIAG_CANAIS"], NUM_CANAIS)
    minimos = le(mapa["DIAG_CANAIS_MIN"], NUM_CANAIS)
    maximos = le(mapa["DIAG_CANAIS_MAX"], NUM_CANAIS)

    if canais is None or minimos is None or maximos is None:
        print("\nNao consegui ler a placa. Ela esta ligada e gravada com o")
        print("build de bancada? (`cargo run --features diag-botoes`)")
        sys.exit(1)

    print("\n%-7s %8s %8s %8s %10s  %s" % (
        "canal", "agora", "min", "max", "variacao", "faixa"))
    print("-" * 62)

    candidatos = []
    for c in range(NUM_CANAIS):
        variacao = max(0, maximos[c] - minimos[c])
        marca = ""
        if variacao >= 20:
            candidatos.append((variacao, c))
            marca = "  <== varia"
        print("%-7d %8d %8d %8d %10d  %-14s%s" % (
            c, canais[c], minimos[c], maximos[c], variacao,
            faixa_de(canais[c]), marca))

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
