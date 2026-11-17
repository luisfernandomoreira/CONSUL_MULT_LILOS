# -*- coding: utf-8 -*-
"""Confere as tabelas portadas contra o firmware C++ de fabrica.

Compara, valor por valor, o que esta em `src/` com o que esta no
projeto CubeIDE original: tempos de agitacao, configuracoes de molho,
ciclo turbo, lavagem economica, padroes de vai-e-vem, fases de
drenagem e centrifugacao, mapa do encoder, mapa de pinos, chave de
modelo e faixas do ADC dos botoes.

Nao precisa de placa nem de cargo. Rode da raiz do projeto:

    python -I ferramentas/confere_contra_cpp.py

Se o projeto C++ estiver em outro lugar, passe o caminho da pasta
`Core/Src` dele como argumento.

Sai com codigo 1 se achar qualquer divergencia — serve para rodar
antes de gravar a placa.
"""
import io
import os
import re
import sys

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)

RUST = os.path.join(RAIZ, "src")

# Padrao: o projeto C++ fica ao lado deste, como na maquina da fabrica.
CPP_PADRAO = os.path.join(
    os.path.dirname(RAIZ), "CONSUL_MULT_CUBE", "CONSUL_MULT_V1", "Core", "Src"
)
CPP = sys.argv[1] if len(sys.argv) > 1 else CPP_PADRAO

if not os.path.isdir(CPP):
    print("Nao achei o projeto C++ em:")
    print("  %s" % CPP)
    print("Passe o caminho da pasta Core/Src como argumento.")
    sys.exit(2)

print("C++  : %s" % CPP)
print("Rust : %s" % RUST)

falhas = []
checagens = 0



def ler(caminho):
    return io.open(caminho, encoding="utf-8", errors="replace").read()


def tira_comentarios(txt):
    txt = re.sub(r"/\*.*?\*/", "", txt, flags=re.S)
    txt = re.sub(r"//[^\n]*", "", txt)
    return txt


def nums(txt):
    return [int(x) for x in re.findall(r"-?\d+", txt)]


def confere(nome, esperado, obtido):
    global checagens
    checagens += 1
    if esperado == obtido:
        print("  OK   %s (%d valores)" % (nome, len(esperado)))
        return
    print("  FALHA %s" % nome)
    print("       C++  : %s" % (esperado,))
    print("       Rust : %s" % (obtido,))
    if len(esperado) == len(obtido):
        difs = [
            (i, e, o) for i, (e, o) in enumerate(zip(esperado, obtido)) if e != o
        ]
        print("       difere nos indices: %s" % (difs,))
    else:
        print("       tamanhos diferentes: %d vs %d" % (len(esperado), len(obtido)))
    falhas.append(nome)


# ═══════════════════════════════════════════════════════════════════════
# Lado C++
# ═══════════════════════════════════════════════════════════════════════

agit_cpp = tira_comentarios(ler(os.path.join(CPP, "agitacaoControl.cpp")))
main_cpp = tira_comentarios(ler(os.path.join(CPP, "main.cpp")))
agita_cpp = tira_comentarios(ler(os.path.join(CPP, "agita.cpp")))
enc_cpp = tira_comentarios(ler(os.path.join(CPP, "encoder.cpp")))

# tempos de agitacao, modelos 1..6
tempos_cpp = {}
for m in range(1, 7):
    bloco = re.search(
        r"temposAgitar%d\[\]\s*=\s*\{(.*?)\}" % m, agit_cpp, re.S
    )
    if not bloco:
        print("!! nao achei temposAgitar%d no C++" % m)
        sys.exit(1)
    tempos_cpp[m] = nums(bloco.group(1))

# configuracoes de molho, modelos 1..6  (cases 1..6 de obterConfiguracoesLavagem)
corpo = main_cpp[
    main_cpp.index(
        "const MaquinaDeLavarFacade::ConfiguracaoLavagemMolho* MaquinaDeLavarFacade::obterConfiguracoesLavagem"
    ) :
]
molho_cpp = {}
for m in range(1, 7):
    ini = re.search(r"case\s+%d\s*:" % m, corpo)
    assert ini, m
    resto = corpo[ini.end() :]
    bloco = re.search(r"\[\]\s*=\s*\{(.*?)\};", resto, re.S)
    assert bloco, m
    molho_cpp[m] = nums(bloco.group(1))

# turboTable e eco, de dentro de Molho()
molho_fn = main_cpp[
    main_cpp.index("void MaquinaDeLavarFacade::Molho()") : main_cpp.index(
        "void MaquinaDeLavarFacade::Enxaguar()"
    )
]
def matriz_cpp(txt, marca, preenche):
    """Le uma matriz [7][16] do C++ expandindo inicializadores parciais.

    Em C, `{0}` numa linha de 16 significa 1 valor escrito e 15
    preenchidos com zero; o Rust tem de ter os 16 explicitos.
    """
    bloco = re.search(marca + r"\s*=\s*\{(.*?)\n\s*\};", txt, re.S)
    assert bloco, marca
    nivel, atual, linhas = 0, "", []
    for ch in bloco.group(1):
        if ch == "{":
            nivel += 1
            if nivel == 1:
                atual = ""
                continue
        elif ch == "}":
            nivel -= 1
            if nivel == 0:
                linhas.append(atual)
                continue
        if nivel >= 1:
            atual += ch
    assert len(linhas) == 7, (marca, len(linhas))
    saida = []
    for l in linhas:
        v = nums(l)
        assert len(v) <= 16, (marca, v)
        saida += v + [preenche] * (16 - len(v))
    return saida


turbo_cpp = matriz_cpp(molho_fn, r"turboTable\[7\]\[16\]", -1)
eco_cpp = matriz_cpp(molho_fn, r"eco\[7\]\[16\]", 0)

# padroes de agitacao
pad_cpp = {}
for nome, fn in [
    ("PADRAO", "agitaPadrao"),
    ("DELICADO", "agitaDelicado"),
    ("TURBO", "agitaTurbo"),
    ("TURBO_DELICADO", "agitaTurboDelicado"),
]:
    ch = re.search(
        r"void Agita::%s\(\)\s*\{(.*?)\n\}" % fn, agita_cpp, re.S
    )
    assert ch, fn
    pad_cpp[nome] = nums(
        re.search(r"executarAgitacao\((.*?)\)", ch.group(1), re.S).group(1)
    )

# fases: drenagem (dentro de Enxaguar) e centrifugacao
enx_fn = main_cpp[
    main_cpp.index("void MaquinaDeLavarFacade::Enxaguar()") : main_cpp.index(
        "void MaquinaDeLavarFacade::Centrifugar()"
    )
]
cen_fn = main_cpp[
    main_cpp.index("void MaquinaDeLavarFacade::Centrifugar()") : main_cpp.index(
        "void MaquinaDeLavarFacade::Desligar()"
    )
]


def fases_cpp(txt, marca):
    bloco = re.search(marca + r"\s*=\s*\{(.*?)\n\s*\};", txt, re.S)
    assert bloco, marca
    saida = []
    for m in re.finditer(
        r"\{\s*(\d+)\s*,\s*(\d+)\s*,\s*\[\]\(\)\s*\{\s*ControleHardware::(\w+)\((\w+)\)",
        bloco.group(1),
    ):
        saida.append(
            (int(m.group(1)), int(m.group(2)), m.group(3), m.group(4) == "true")
        )
    return saida


dren_cpp = fases_cpp(enx_fn, r"Phase phases\[\]")
cent_cpp = fases_cpp(cen_fn, r"Phase phases\[\]")

# encoder: padrao de 4 bits -> posicao
enc_map_cpp = {}
for m in re.finditer(r"case\s+0b([01]{4})\s*:\s*tipo_de_lavagem\s*=\s*(\d+)", enc_cpp):
    enc_map_cpp[int(m.group(1), 2)] = int(m.group(2))

# ═══════════════════════════════════════════════════════════════════════
# Lado Rust
# ═══════════════════════════════════════════════════════════════════════

tab_rs = tira_comentarios(ler(os.path.join(RUST, "tabelas.rs")))
agita_rs = tira_comentarios(ler(os.path.join(RUST, "agita.rs")))
maq_rs = tira_comentarios(ler(os.path.join(RUST, "maquina.rs")))
enc_rs = tira_comentarios(ler(os.path.join(RUST, "encoder.rs")))


def linhas_de(txt, nome, n_modelos=7):
    """Extrai as N linhas [..] de uma tabela [[T; 16]; 7]."""
    ini = txt.index(nome)
    corpo = txt[ini:]
    corpo = corpo[corpo.index("= [") + 3 :]
    # pega os blocos de primeiro nivel
    linhas, nivel, atual = [], 0, ""
    for ch in corpo:
        if ch == "[":
            nivel += 1
            if nivel == 1:
                atual = ""
                continue
        elif ch == "]":
            nivel -= 1
            if nivel == 0:
                linhas.append(atual)
                if len(linhas) == n_modelos:
                    break
                continue
        if nivel >= 1:
            atual += ch
    return linhas


def expande_16(linha, bools=False):
    """Trata `[0; 16]`, `[-1; 16]` e `[false; 16]`."""
    curto = re.match(r"^\s*(-?\w+)\s*;\s*(?:16|NUM_TIPOS)\s*$", linha)
    if curto:
        v = curto.group(1)
        if bools:
            return [1 if v == "true" else 0] * 16
        return [int(v)] * 16
    if bools:
        return [1 if t == "true" else 0 for t in re.findall(r"true|false", linha)]
    return nums(linha)


# TEMPO_AGITAR
tempos_rs = {}
for i, l in enumerate(linhas_de(tab_rs, "TEMPO_AGITAR")):
    if i:
        tempos_rs[i] = expande_16(l)

# CONFIG_MOLHO  (cfg(a,b,c) e SEM_MOLHO)
molho_rs = {}
for i, l in enumerate(linhas_de(tab_rs, "CONFIG_MOLHO")):
    if not i:
        continue
    vals = []
    for item in re.finditer(r"cfg\(([^)]*)\)|SEM_MOLHO", l):
        if item.group(0) == "SEM_MOLHO":
            vals += [0, 0, 0]
        else:
            vals += nums(item.group(1))
    molho_rs[i] = vals

# CICLO_TURBO
turbo_rs = []
for l in linhas_de(tab_rs, "CICLO_TURBO"):
    turbo_rs += expande_16(l)

# REUTILIZA_AGUA
eco_rs = []
for l in linhas_de(tab_rs, "REUTILIZA_AGUA"):
    eco_rs += expande_16(l, bools=True)

# padroes
pad_rs = {}
for nome in ["PADRAO", "DELICADO", "TURBO", "TURBO_DELICADO"]:
    bloco = re.search(
        r"pub const %s: Padrao = Padrao \{(.*?)\};" % nome, agita_rs, re.S
    )
    assert bloco, nome
    pad_rs[nome] = nums(bloco.group(1))

# fases
ACAO = {"Pulso": "MotorPulse", "Bomba": "Bomba", "Atuador": "Atuador"}


def fases_rs(nome):
    bloco = re.search(nome + r"[^=]*=\s*\[(.*?)\n\];", maq_rs, re.S)
    assert bloco, nome
    saida = []
    for m in re.finditer(
        r"fase\((\d+),\s*(\d+),\s*Acao::(\w+)\((\w+)\)\)", bloco.group(1)
    ):
        saida.append(
            (
                int(m.group(1)),
                int(m.group(2)),
                ACAO[m.group(3)],
                m.group(4) == "true",
            )
        )
    return saida


dren_rs = fases_rs("FASES_DRENAGEM")
cent_rs = fases_rs("FASES_CENTRIFUGA")

# encoder
enc_map_rs = {}
for m in re.finditer(r"0b([01]{4})\s*=>\s*(\d+)", enc_rs):
    enc_map_rs[int(m.group(1), 2)] = int(m.group(2))

# ═══════════════════════════════════════════════════════════════════════
# Comparacoes
# ═══════════════════════════════════════════════════════════════════════

print("\n=== TEMPO DE AGITACAO (agitacaoControl.cpp) ===")
for m in range(1, 7):
    confere("TEMPO_AGITAR modelo %d" % m, tempos_cpp[m], tempos_rs.get(m, []))

print("\n=== CONFIGURACOES DE MOLHO (obterConfiguracoesLavagem) ===")
for m in range(1, 7):
    confere("CONFIG_MOLHO modelo %d" % m, molho_cpp[m], molho_rs.get(m, []))

print("\n=== CICLO TURBO (turboTable) ===")
confere("CICLO_TURBO 7x16", turbo_cpp, turbo_rs)

print("\n=== LAVAGEM ECONOMICA (eco) ===")
confere("REUTILIZA_AGUA 7x16", eco_cpp, eco_rs)

print("\n=== PADROES DE AGITACAO (agita.cpp) ===")
for nome in ["PADRAO", "DELICADO", "TURBO", "TURBO_DELICADO"]:
    confere("Padrao::%s" % nome, pad_cpp[nome], pad_rs[nome])

print("\n=== FASES DE DRENAGEM (Enxaguar) ===")
confere("FASES_DRENAGEM", dren_cpp, dren_rs)

print("\n=== FASES DE CENTRIFUGACAO (Centrifugar) ===")
confere("FASES_CENTRIFUGA", cent_cpp, cent_rs)

print("\n=== ENCODER (encoder.cpp) ===")
confere(
    "mapa de 16 posicoes",
    sorted(enc_map_cpp.items()),
    sorted(enc_map_rs.items()),
)


# =======================================================================
# Mapa de pinos, chave de modelo e faixas dos botoes
# =======================================================================

macros_cpp = tira_comentarios(ler(os.path.join(CPP, "macros.h")))
placa_rs = tira_comentarios(ler(os.path.join(RUST, "placa.rs")))
modelo_rs = tira_comentarios(ler(os.path.join(RUST, "modelo.rs")))
botoes_rs = tira_comentarios(ler(os.path.join(RUST, "botoes.rs")))

# --- pinos do C++: o numero vem do codigo, a porta vem do comentario ---
hw_bruto = ler(os.path.join(CPP, "controle_hardware.hpp"))
pinos_cpp = {}
for m in re.finditer(
    r"(PINO_\w+|SW\d)\s*=\s*LL_GPIO_PIN_(\d+);\s*//\s*GPIO([ABC])", hw_bruto
):
    pinos_cpp[m.group(1)] = (m.group(3), int(m.group(2)))

# encoder: porta e numero no proprio #define de macros.h
for nome in ["ENC_1", "ENC_3", "ENC_4", "ENC_5"]:
    m = re.search(
        r"#define %s .*?GPIO([ABC]), LL_GPIO_PIN_(\d+)" % nome, macros_cpp
    )
    assert m, nome
    pinos_cpp[nome] = (m.group(1), int(m.group(2)))

# --- pinos do Rust ---
pinos_rs = {}
for m in re.finditer(
    r"pub const (\w+): Pino = Pino::novo\(Porta::([ABC]), (\d+)\)", placa_rs
):
    pinos_rs[m.group(1)] = (m.group(2), int(m.group(3)))

# de que constante do Rust cada pino do C++ deve sair
EQUIV = {
    "PINO_BOMBA": "BOMBA",
    "PINO_ATUADOR": "ATUADOR",
    "PINO_VALVULA_AMACIANTE": "ATUADOR",  # mesmo triac
    "PINO_MOTOR_C": "MOTOR_C",
    "PINO_MOTOR_PULSE": "MOTOR_C",  # mesmo triac
    "PINO_MOTOR_A": "MOTOR_A",
    "PINO_VALVULA_SABAO": "VALVULA_SABAO",
    "PINO_LED": "LED_PAUSE",
    "PINO_LED_MAIS_SECAS": "LED_MAIS_SECAS",
    "PINO_LED_ENXAGUE_DUPLO": "LED_ENXAGUE_DUPLO",
    "PINO_PRESSOSTATO": "PRESSOSTATO",
    "PINO_TAMPA": "TAMPA",
    "PINO_TAMPA_MECANICA": "TAMPA_MECANICA",
    "SW1": "SW1",
    "SW2": "SW2",
    "SW3": "SW3",
    "SW4": "SW4",
    "ENC_1": "ENC_1",
    "ENC_3": "ENC_3",
    "ENC_4": "ENC_4",
    "ENC_5": "ENC_5",
}

print("\n=== MAPA DE PINOS (controle_hardware.hpp + macros.h) ===")
for nome_cpp, nome_rs in sorted(EQUIV.items()):
    assert nome_cpp in pinos_cpp, "nao achei %s no C++" % nome_cpp
    confere(
        "%s -> %s" % (nome_cpp, nome_rs),
        [pinos_cpp[nome_cpp]],
        [pinos_rs.get(nome_rs, ("?", -1))],
    )

# --- chave de modelo ---
mod_cpp_txt = tira_comentarios(ler(os.path.join(CPP, "modeloMaquina.cpp")))
mod_map_cpp = {}
for m in re.finditer(r"case\s+(\d+):\s*tipoDoModelo\s*=\s*(\d+)", mod_cpp_txt):
    mod_map_cpp[int(m.group(1))] = int(m.group(2))
inval = re.search(r"default:\s*tipoDoModelo\s*=\s*(\d+)", mod_cpp_txt)
assert inval
invalido_cpp = int(inval.group(1))

mod_map_rs = {}
for m in re.finditer(r"0b([01]{4}) => (\d+),", modelo_rs):
    mod_map_rs[int(m.group(1), 2)] = int(m.group(2))
inval_rs = re.search(r"_ => (\d+),", modelo_rs)
assert inval_rs, "nao achei o braco de modelo invalido no Rust"

print("\n=== CHAVE DE MODELO (modeloMaquina.cpp) ===")
confere("combinacoes validas", sorted(mod_map_cpp.items()), sorted(mod_map_rs.items()))
confere("modelo invalido", [invalido_cpp], [int(inval_rs.group(1))])

# --- faixas do ADC dos botoes ---
ger = main_cpp[
    main_cpp.index("void MaquinaDeLavarFacade::gerenciaInterface()") : main_cpp.index(
        "void MaquinaDeLavarFacade::selecionaProgramaInicial()"
    )
]
faixas_cpp = [
    tuple(nums(m.group(1)))
    for m in re.finditer(r"isAdButtonInRange\(this->adButton,\s*([^)]*)\)", ger)
]
faixas_rs = []
for nome in ["REPOUSO", "FAIXA_PAUSE", "FAIXA_MAIS_SECAS", "FAIXA_ENXAGUE_DUPLO"]:
    m = re.search(r"const %s: \(u16, u16\) = \(([^)]*)\)" % nome, botoes_rs)
    assert m, nome
    faixas_rs.append(tuple(nums(m.group(1))))

# As faixas do ADC sao o UNICO lugar em que o porte divergiu de
# proposito. Medido na bancada, o mesmo divisor resistivo le cerca de
# 31 contagens abaixo no GD32: o repouso da 0 em vez de ~33, e o botao
# de ON/OFF da 789 em vez de ~820. As faixas do Rust sao centradas nos
# valores MEDIDOS na placa (789, 849 e 931), mantendo a largura de +-25
# que o C++ usava.
#
# (A primeira tentativa foi alargar as faixas para cobrir as duas
# escalas. Deu defeito: a tensao sobe gradualmente quando o contato
# fecha, e a faixa de Pausa alargada capturava a subida do Mais Secas,
# desligando a maquina. Por isso: deslocar, nao alargar.)
#
# Aqui nao se exige igualdade. Exige-se o que precisa valer: as faixas
# nao podem se sobrepor, a largura tem de ser a mesma do C++, e o
# deslocamento tem de ser igual nas tres — se alguem mexer numa so,
# isto acusa.
print("\n=== FAIXAS DO ADC DOS BOTOES (divergencia esperada) ===")
NOMES_FAIXAS = ["repouso", "PAUSA", "MAIS SECAS", "ENXAGUE DUPLO"]

if len(faixas_cpp) == len(faixas_rs) == 4:
    print("  %-14s %-12s %-12s %-9s %s"
          % ("faixa", "C++ (STM32)", "Rust (GD32)", "largura", "desloc"))
    deslocamentos = []
    for nome, c, r in zip(NOMES_FAIXAS, faixas_cpp, faixas_rs):
        desloc = (r[0] + r[1]) // 2 - (c[0] + c[1]) // 2
        if nome != "repouso":  # o repouso satura em zero, nao desloca
            deslocamentos.append(desloc)
        print("  %-14s %-12s %-12s %-9s %+d"
              % (nome, "%d..%d" % c, "%d..%d" % r,
                 "%d/%d" % (c[1] - c[0], r[1] - r[0]), desloc))

    # 1. Sem sobreposicao entre faixas vizinhas — foi o que causou o
    #    defeito do Mais Secas desligando a maquina.
    for i in range(3):
        confere(
            "folga entre %s e %s" % (NOMES_FAIXAS[i], NOMES_FAIXAS[i + 1]),
            [True],
            [faixas_rs[i][1] < faixas_rs[i + 1][0]],
        )

    # 2. Deslocar nao muda largura.
    for nome, c, r in zip(NOMES_FAIXAS, faixas_cpp, faixas_rs):
        if nome == "repouso":
            continue
        confere("largura da faixa %s" % nome, [c[1] - c[0]], [r[1] - r[0]])

    # 3. Deslocamentos PARECIDOS entre si.
    #
    #    Quando dois dos tres centros eram inferidos de um unico
    #    deslocamento, exigir que os tres fossem identicos fazia
    #    sentido. Agora os tres sao MEDIDOS na placa, e medidas reais
    #    nao batem na casa decimal: Pausa -31, Mais Secas -33, Enxague
    #    Duplo -31.
    #
    #    O que continua valendo e que o desvio tem a MESMA natureza nas
    #    tres — se uma faixa fugir das outras, ou alguem errou um
    #    numero, ou aquele botao tem outro problema.
    ESPALHAMENTO_MAXIMO = 6
    espalhamento = max(deslocamentos) - min(deslocamentos)
    confere(
        "deslocamentos parecidos (espalhamento <= %d)" % ESPALHAMENTO_MAXIMO,
        [True],
        [espalhamento <= ESPALHAMENTO_MAXIMO],
    )
    print(
        "  (deslocamento medido na placa: %+d a %+d contagens, espalhamento %d)"
        % (max(deslocamentos), min(deslocamentos), espalhamento)
    )
else:
    confere("4 faixas", faixas_cpp, faixas_rs)

print("\n" + "=" * 62)
if falhas:
    print("%d de %d checagens FALHARAM:" % (len(falhas), checagens))
    for f in falhas:
        print("  - %s" % f)
    sys.exit(1)
print("Tudo certo: %d checagens, nenhuma divergencia." % checagens)
