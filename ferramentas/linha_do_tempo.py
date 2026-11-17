# -*- coding: utf-8 -*-
"""Simula no PC o que cada saida faz, segundo a segundo, numa etapa.

    python -I ferramentas/linha_do_tempo.py
    python -I ferramentas/linha_do_tempo.py centrifugar 1 --mais-secas
    python -I ferramentas/linha_do_tempo.py drenagem

Le as tabelas de fases direto de `src/maquina.rs`, entao nao tem como
divergir do firmware: mexer numa fase muda a simulacao.

Serve para responder perguntas do tipo "esta saida liga e nao desliga
mais?" sem precisar de placa, osciloscopio e meia hora de ciclo.
"""
import io
import os
import re
import sys

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)
MAQUINA = os.path.join(RAIZ, "src", "maquina.rs")

# Os nomes como o esquema os chama, para o relatorio falar de pinos.
PINOS = {
    "Bomba": "PB2 (bomba)",
    "Atuador": "PA3 (atuador / valvula de amaciante)",
    "Pulso": "PA9 (motor C / pulso)",
}


def tabela_de_fases(nome):
    """Extrai uma tabela `fase(ini, fim, Acao::X(bool))` de maquina.rs."""
    src = io.open(MAQUINA, encoding="utf-8").read()
    m = re.search(nome + r"[^=]*=\s*\[(.*?)\n\];", src, re.S)
    if not m:
        sys.exit("nao achei a tabela %s em %s" % (nome, MAQUINA))
    fases = []
    for f in re.finditer(
        r"fase\((\d+),\s*(\d+),\s*Acao::(\w+)\((\w+)\)\)", m.group(1)
    ):
        fases.append(
            (int(f.group(1)), int(f.group(2)), f.group(3), f.group(4) == "true")
        )
    if not fases:
        sys.exit("a tabela %s veio vazia" % nome)
    return fases


def corte_da_centrifuga(modelo, mais_secas):
    """Quando o relogio salta para 954, encurtando a centrifugacao.

    Espelha a logica de `centrifugar` em maquina.rs.
    """
    if modelo == 5:
        mais_secas = False  # o modelo 5 ignora o botao
    if mais_secas:
        if modelo <= 2 or 4 <= modelo <= 6:
            return 838  # 10 min
        return None  # modelo 3: vai ate o fim, 12 min
    if modelo <= 2 or modelo == 4 or modelo == 6:
        return 538  # 5 min
    if modelo in (3, 5):
        return 598  # 6 min
    return None


def simula_centrifuga(modelo, mais_secas):
    fases = tabela_de_fases("FASES_CENTRIFUGA")
    bomba_sozinha = modelo in (3, 4)
    corte = corte_da_centrifuga(modelo, mais_secas)

    estado = {"Bomba": False, "Atuador": False, "Pulso": False}
    transicoes = []
    ligado_desde = {}
    tempo_ligado = {k: 0 for k in estado}

    # O corte da centrifugacao SALTA o relogio do ciclo, entao o valor
    # do relogio deixa de ser o tempo decorrido. `real` conta os
    # segundos que de fato passam; e ele que vale para dizer quanto
    # tempo uma saida ficou ligada.
    relogio = 0
    real = 0
    while real <= 1200:
        anterior = dict(estado)

        # Regra solta de maquina.rs: o atuador entra nos primeiros 90 s
        # e NINGUEM o desliga antes da fase 982..991.
        if relogio <= 90 and not bomba_sozinha:
            estado["Atuador"] = True

        fim_das_fases = True
        for ini, f, acao, valor in fases:
            if ini <= relogio <= f:
                estado[acao] = valor
                fim_das_fases = False
                break

        if fim_das_fases:
            # Passou de todas as fases: a etapa termina e tudo desliga.
            for k in estado:
                estado[k] = False

        for k, v in estado.items():
            if anterior[k] != v:
                transicoes.append((real, relogio, k, v))
                if v:
                    ligado_desde[k] = real
                elif k in ligado_desde:
                    tempo_ligado[k] += real - ligado_desde.pop(k)

        if fim_das_fases:
            transicoes.append((real, relogio, "FIM DA ETAPA", False))
            break

        real += 1
        # O corte do tempo de centrifugacao salta o relogio, mas nao o
        # tempo real.
        if corte is not None and 238 <= relogio <= 953 and relogio >= corte:
            relogio = 954
        else:
            relogio += 1

    return transicoes, tempo_ligado


def mostra(transicoes, tempo_ligado, titulo):
    print("")
    print(titulo)
    print("-" * len(titulo))
    print("%6s %8s  %-38s %s" % ("real", "relogio", "saida", "passa a"))
    for real, relogio, k, v in transicoes:
        marca = "" if real == relogio else "  <- relogio saltou"
        if k == "FIM DA ETAPA":
            print("%5ds %7ds  %s%s"
                  % (real, relogio, "--- fim das fases: tudo desligado ---", marca))
            continue
        print("%5ds %7ds  %-38s %s%s"
              % (real, relogio, PINOS.get(k, k), "LIGA" if v else "desliga", marca))

    print("")
    print("Tempo total ligado:")
    for k, seg in sorted(tempo_ligado.items(), key=lambda x: -x[1]):
        if seg:
            print("  %-38s %4ds  (%d min %02ds)" % (PINOS.get(k, k), seg, seg // 60, seg % 60))


def main():
    args = [a for a in sys.argv[1:]]
    mais_secas = "--mais-secas" in args
    if mais_secas:
        args.remove("--mais-secas")
    etapa = args[0] if args else "centrifugar"
    modelo = int(args[1]) if len(args) > 1 else 1

    if etapa == "drenagem":
        fases = tabela_de_fases("FASES_DRENAGEM")
        estado = {"Pulso": False}
        trans, ligado = [], {"Pulso": 0}
        desde = None
        for t in range(0, 420):
            for ini, f, acao, valor in fases:
                if ini <= t <= f:
                    if estado[acao] != valor:
                        estado[acao] = valor
                        trans.append((t, t, acao, valor))
                        if valor:
                            desde = t
                        elif desde is not None:
                            ligado["Pulso"] += t - desde
                            desde = None
                    break
        mostra(trans, ligado, "DRENAGEM DO ENXAGUE — pulsos de motor")
        return

    if etapa != "centrifugar":
        sys.exit("etapa desconhecida: %s (use centrifugar ou drenagem)" % etapa)

    trans, ligado = simula_centrifuga(modelo, mais_secas)
    mostra(
        trans,
        ligado,
        "CENTRIFUGACAO — modelo %d, mais secas %s"
        % (modelo, "LIGADO" if mais_secas else "desligado"),
    )


if __name__ == "__main__":
    main()
