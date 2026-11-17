# -*- coding: utf-8 -*-
"""Gera o .hex de producao e confere que ele e gravavel.

    python -I ferramentas/gera_hex.py

Para apenas CONFERIR um .hex que ja existe, sem compilar nem converter
(util para validar uma imagem gerada por outra pessoa ou por outra
ferramenta):

    python -I ferramentas/gera_hex.py --so-confere
    python -I ferramentas/gera_hex.py --so-confere outra_imagem.hex

Faz tres coisas, nessa ordem:

1. compila o perfil release (sem a feature de bancada);
2. converte o ELF em Intel HEX, usando o objcopy que estiver disponivel;
3. CONFERE o resultado — e aqui que esta o valor.

A conferencia existe por um motivo concreto: a ultima pagina da flash
(0x0800FC00) guarda o estado do ciclo, usado para retomar depois de uma
queda de energia. Se a imagem crescer e invadir essa pagina, o gravador
sobrescreve o estado e a retomada para de funcionar — sem erro de
compilacao e sem aviso nenhum. O `memory.x` reserva a pagina, mas um
`.hex` gerado do ELF errado, ou um memory.x alterado, passaria batido.

Sai com codigo 1 se algo nao fechar, entao serve para rodar antes de
mandar a imagem para a producao.
"""
import io
import os
import shutil
import subprocess
import sys

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.dirname(AQUI)

ELF = os.path.join(RAIZ, "target", "thumbv6m-none-eabi", "release", "consul_mult_lilos")
HEX = os.path.join(RAIZ, "consul_mult_lilos.hex")

# Mapa da flash do GD32C231K8, conforme memory.x e estado_salvo.rs.
FLASH_INICIO = 0x0800_0000
PAGINA_ESTADO = 0x0800_FC00  # ultima pagina de 1 KB, reservada
FLASH_FIM = 0x0801_0000  # 64 KB


def roda(cmd):
    return subprocess.run(cmd, cwd=RAIZ, capture_output=True, text=True)


def compila():
    print("Compilando o perfil release...")
    r = roda(["cargo", "build", "--release"])
    if r.returncode != 0:
        print(r.stdout)
        print(r.stderr)
        sys.exit("cargo build falhou")
    if not os.path.exists(ELF):
        sys.exit("nao achei o ELF em %s" % ELF)


def acha_objcopy():
    """Devolve o comando de objcopy disponivel, em ordem de preferencia."""
    for nome in ("arm-none-eabi-objcopy", "llvm-objcopy"):
        caminho = shutil.which(nome)
        if caminho:
            return [caminho]

    # O llvm-objcopy do componente llvm-tools nao entra no PATH.
    r = roda(["rustc", "--print", "sysroot"])
    if r.returncode == 0:
        raiz = r.stdout.strip()
        for pasta, _, arquivos in os.walk(os.path.join(raiz, "lib", "rustlib")):
            for a in arquivos:
                if a in ("llvm-objcopy", "llvm-objcopy.exe"):
                    return [os.path.join(pasta, a)]

    sys.exit(
        "Nao achei objcopy. Instale uma destas:\n"
        "  rustup component add llvm-tools\n"
        "  (ou use o arm-none-eabi-objcopy do toolchain ARM)"
    )


def converte():
    objcopy = acha_objcopy()
    print("Convertendo com %s" % os.path.basename(objcopy[0]))
    r = roda(objcopy + ["-O", "ihex", ELF, HEX])
    if r.returncode != 0:
        print(r.stderr)
        sys.exit("objcopy falhou")


def confere():
    """Le o Intel HEX e confere formato, checksums e faixa de enderecos."""
    base = 0
    baixo = alto = None
    bytes_dados = 0
    linhas = 0
    erros = []

    for n, linha in enumerate(io.open(HEX, encoding="ascii"), 1):
        linha = linha.strip()
        if not linha:
            continue
        linhas += 1
        if not linha.startswith(":"):
            erros.append("linha %d nao comeca com ':'" % n)
            continue
        try:
            cru = bytes.fromhex(linha[1:])
        except ValueError:
            erros.append("linha %d nao e hexadecimal valido" % n)
            continue
        if len(cru) < 5:
            erros.append("linha %d curta demais para ser um registro" % n)
            continue
        if (sum(cru) & 0xFF) != 0:
            erros.append("linha %d com checksum errado" % n)
        conta, endereco, tipo = cru[0], (cru[1] << 8) | cru[2], cru[3]
        # 4 bytes de cabecalho + os dados + 1 de checksum.
        if len(cru) != conta + 5:
            erros.append(
                "linha %d diz %d bytes de dados mas tem %d"
                % (n, conta, max(0, len(cru) - 5))
            )
            continue
        dados = cru[4 : 4 + conta]
        if tipo == 0x04:  # endereco linear estendido
            base = ((dados[0] << 8) | dados[1]) << 16
        elif tipo == 0x00:  # dados
            ini = base + endereco
            fim = ini + conta - 1
            baixo = ini if baixo is None else min(baixo, ini)
            alto = fim if alto is None else max(alto, fim)
            bytes_dados += conta

    if baixo is None:
        print("Nenhum registro de dados utilizavel no .hex.")
        for e in erros[:10]:
            print("  %s" % e)
        if not erros:
            print("  (o arquivo so tem registros de cabecalho ou de fim)")
        sys.exit(1)

    print("")
    print("%-22s %s" % ("arquivo", HEX))
    print("%-22s %d" % ("linhas", linhas))
    print("%-22s %d" % ("bytes de dados", bytes_dados))
    print("%-22s 0x%08X" % ("primeiro endereco", baixo))
    print("%-22s 0x%08X" % ("ultimo endereco", alto))
    print("")

    tudo_ok = True

    def checa(cond, msg):
        nonlocal tudo_ok
        print("  %s  %s" % ("OK   " if cond else "FALHA", msg))
        tudo_ok = tudo_ok and bool(cond)

    checa(not erros, "formato e checksums de todas as linhas")
    for e in erros[:5]:
        print("         %s" % e)

    checa(baixo == FLASH_INICIO, "comeca em 0x%08X (vetor de reset)" % FLASH_INICIO)
    checa(
        alto < PAGINA_ESTADO,
        "termina antes de 0x%08X — nao invade a pagina do estado salvo" % PAGINA_ESTADO,
    )
    checa(alto < FLASH_FIM, "cabe nos 64 KB do chip")

    gravavel = PAGINA_ESTADO - FLASH_INICIO
    folga = PAGINA_ESTADO - 1 - alto
    print("")
    print(
        "ocupa %d bytes dos %d gravaveis (%.1f%%)"
        % (bytes_dados, gravavel, bytes_dados * 100.0 / gravavel)
    )
    print("folga ate a pagina do estado salvo: %d bytes (%.1f KB)" % (folga, folga / 1024.0))

    return tudo_ok


def main():
    global HEX
    argumentos = sys.argv[1:]
    so_confere = "--so-confere" in argumentos
    if so_confere:
        argumentos.remove("--so-confere")
        if argumentos:
            HEX = os.path.abspath(argumentos[0])
        if not os.path.exists(HEX):
            sys.exit("nao achei %s" % HEX)
        print("Conferindo %s (sem compilar)" % HEX)
    else:
        if argumentos:
            sys.exit("argumento inesperado: %s" % argumentos[0])
        compila()
        converte()

    if not confere():
        sys.exit("\nO .hex NAO esta bom para gravar. Veja as falhas acima.")
    print("")
    print("Pronto para o gravador (probe-rs, GD-Link ou GD32 All-In-One).")


if __name__ == "__main__":
    main()
