# Janela do DMG no macOS

Esta é a aparência exibida pelo Finder quando o DMG é aberto: um fundo de 660 × 400 pt, com o ícone do aplicativo e o atalho para `Applications` lado a lado. O script `package.sh` copia esses arquivos para a imagem. Nada é gerado durante a compilação, portanto o DMG continua sendo criado com `hdiutil makehybrid` sem dispositivo montado, automação do Finder no CI ou instalação extra durante a tarefa de assinatura.

| Arquivo | Descrição |
|---|---|
| `background.svg` | Arquivo-fonte do fundo: o ícone do aplicativo (`assets/app-icon/cadcraft-1024.png`, vinculado, não copiado) recortado para preencher o campo de cores do CADCraft (`#14a3c7`), Ink e Paper. O texto (Inter e JetBrains Mono) é convertido em contornos, então a renderização não depende de fontes instaladas. Não há textura de papel: ruído não comprime bem e aumentaria o tamanho de `background.tiff`. |
| `background.tiff` | Fundo em 1x (660 × 400 px, 72 dpi) e 2x (1320 × 800 px, 144 dpi) em um TIFF HiDPI (paleta de 16 cores, Deflate, sRGB). Vai para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Configurações de visualização do volume no Finder: tamanho da janela, ícones de 128 px, `CADCraft.app` em (326, 205), `Applications` em (574, 205) e o fundo. Vai para `.DS_Store` na imagem; o nome evita confusão com um `.DS_Store` gerado pelo Finder. |
| `generate.py` | Gera `background.tiff` e `dmg-layout.DS_Store` a partir do SVG e do layout descrito acima. |

## O nome do volume não inclui a versão

O volume montado chama-se `CADCraft`, não `CADCraft <versão>`. O layout aponta para o fundo por meio de um alias que inclui o nome do volume, e o Finder resolve esse alias usando esse nome. Se o volume tiver um nome com versão, a janela mantém o tamanho e as posições dos ícones, mas o fundo não aparece (comportamento testado).

O nome do arquivo DMG (`cadcraft-<version>-macos-<arch>.dmg`) continua incluindo a versão.

## Regras de layout

- **O Finder desenha os rótulos dos ícones em preto nos modos claro e escuro** quando a janela tem fundo. Por isso, a área sob os dois ícones permanece clara (Paper).
- **Não coloque arte dentro das caixas dos ícones:** mantenha 10 pt de espaço em relação a cada caixa de ícone de 128 pt e à faixa do rótulo abaixo dela.

## Regenerar os arquivos

Edite `background.svg` (ou as constantes de layout em `generate.py`) e execute, em qualquer sistema operacional:

```sh
pip install 'pillow>=12' ds_store==1.3.3 mac_alias==2.2.3
python3 packaging/macos/dmg/generate.py   # requer resvg no PATH
```

As duas saídas são reproduzíveis byte a byte e não incluem dados da máquina que as gerou: o SVG é renderizado sem fontes instaladas e `dmg-layout.DS_Store` é criado do zero, com um alias de fundo que contém apenas o nome do volume e `/.background/background.tiff`. Se adicionar texto ao SVG, converta-o em contornos (`usvg`, do resvg, converte texto em caminhos).

A janela mede 660 × 432 pt, incluindo a barra de título de 32 pt do Finder; a área de conteúdo mede 660 × 400 pt.

Este procedimento segue o diretório `packaging/macos/dmg/` do VectorCraft (storytold/vectorcraft#493).
