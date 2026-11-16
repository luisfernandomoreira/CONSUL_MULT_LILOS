# SVD e geração do PAC

`GD32C231.svd` é o arquivo oficial da GigaDevice, extraído de
`GigaDevice.GD32C2x1_DFP.1.1.0.pack` (pasta `SVD/` dentro do pack —
o `.pack` é um zip).

A partir dele é gerado o PAC em `crates/gd32c2x1-pac/`, que é a única
coisa no firmware que conhece endereços de registrador.

## Duas correções no SVD da GigaDevice

O SVD oficial traz dois valores errados na seção `<cpu>`:

| Campo | SVD diz | Correto | Fonte |
|---|---|---|---|
| `nvicPrioBits` | 4 | **2** | User Manual §5.2: "2 bits interrupt priority configuration - 4 priority levels" |
| `fpuPresent` | 1 | **0** | o Cortex-M23 não tem unidade de ponto flutuante |

O `nvicPrioBits` é o que importa: ele vira `NVIC_PRIO_BITS` no PAC, e é
o que um framework como o RTIC usa para codificar prioridade de
interrupção. Gerar com 4 faria as prioridades serem escritas nos bits
errados do NVIC.

O arquivo do fabricante fica intocado; a correção é feita numa cópia
(`GD32C231.corrigido.svd`) no momento de gerar.

## Regerar o PAC

Só é preciso quando a GigaDevice soltar um pack novo. O conteúdo de
`crates/gd32c2x1-pac/src/` e `device.x` é **gerado — não edite à mão**.

Ferramentas (uma vez):

```bash
cargo install svd2rust form
```

Corrigir o SVD (a partir da raiz do projeto):

```bash
sed -e 's|<nvicPrioBits>4</nvicPrioBits>|<nvicPrioBits>2</nvicPrioBits>|' -e 's|<fpuPresent>1</fpuPresent>|<fpuPresent>0</fpuPresent>|' svd/GD32C231.svd > svd/GD32C231.corrigido.svd
```

Gerar:

```bash
cd crates/gd32c2x1-pac && svd2rust -i ../../svd/GD32C231.corrigido.svd --target cortex-m && mkdir -p src && form -i lib.rs -o src/ && rm lib.rs && cargo fmt
```

O `Cargo.toml` da crate é escrito à mão e **não** deve ser sobrescrito.

## Baixar o pack de novo

<https://www.gd32mcu.com/en/download/7?kw=GD32C2> → "GD32C2x1 AddOn".
Renomeie para `.zip`, extraia, e o SVD está em `SVD/GD32C231.svd`.
O mesmo pack traz `Flash/GD32C231_64K.FLM`, usado para gerar
`target-probe/GD32C231K8.yaml` (veja o README principal).
