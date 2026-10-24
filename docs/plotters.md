# Plotters

Um plotter (dispositivo de saída) determina o que uma plotagem gera e quais tamanhos de papel estão disponíveis. As configurações de página e as plotagens selecionam um dispositivo e um dos papéis disponíveis nele. Cada papel tem margens que delimitam a área imprimível. Os layouts mostram essa área como um contorno tracejado na folha.

## Dispositivos

Os dispositivos integrados aparecem primeiro:

| Dispositivo | Saída |
|---|---|
| `CADCraft PDF` | PDF vetorial |
| `CADCraft PNG` | PNG rasterizado em papel branco, na resolução do dispositivo (300 dpi) |
| `CADCraft SVG` | SVG vetorial |

Eles oferecem os tamanhos padrão:

- ISO 216 A0–A6 e B0–B5
- ANSI A–E e Legal
- ARCH A–E e E1

As folhas ISO têm margens de 5 mm. As folhas em polegadas têm margens de um quarto de polegada (6,35 mm). Uma configuração de página com o dispositivo `None` gera um PDF no papel definido para a configuração, como antes.

Os dispositivos do usuário são arquivos no formato próprio do CADCraft, armazenados na pasta `plotters` do diretório de configurações:

| Sistema | Pasta |
|---|---|
| Linux e FreeBSD | `$XDG_CONFIG_HOME/cadcraft/plotters` ou `~/.config/cadcraft/plotters` |
| macOS | `~/Library/Application Support/CADCraft/plotters` |
| Windows | `%APPDATA%\CADCraft\plotters` |

A variável `CADCRAFT_CONFIG_DIR` substitui o diretório de configurações. Na versão web, os dispositivos ficam disponíveis durante a sessão, a menos que o host forneça armazenamento (`cmd::plotters::set_plotter_store`). Arquivos binários `.pc3` não são lidos; não encontramos uma especificação pública desse formato.

## Arquivos

`<name>.plotter.json`, JSON em UTF-8 com um marcador de formato:

```json
{
  "cadcraftPlotter": 1,
  "name": "Office A3",
  "description": "Raster proofs",
  "output": "png",
  "resolution": 150,
  "orientation": "landscape",
  "defaultPaper": "A3 borderless",
  "papers": [
    { "name": "A3 borderless", "width": 297, "height": 420, "margins": [0, 0, 0, 0] },
    { "name": "ISO A4 (210.00 x 297.00 MM)", "width": 210, "height": 297, "margins": [5, 5, 5, 5] }
  ]
}
```

- `output` aceita `pdf`, `png` ou `svg`.
- `resolution` é medida em pontos por polegada (50–2400) e se aplica ao PNG.
- `orientation` (`portrait` ou `landscape`) é usada quando uma configuração de página troca para esse dispositivo.
- Os tamanhos de papel são informados em milímetros na folha em orientação retrato. `margins` representa esquerda, inferior, direita e superior, em mm.

Ao ler arquivos, valores potencialmente maliciosos ou inválidos são limitados:

- Os tamanhos ficam entre 1 e 5080 mm; largura e altura são trocadas quando necessário para usar orientação retrato.
- As margens ficam entre 0 e 1000 mm. Margens que eliminem toda a área imprimível são descartadas.
- Os nomes dos papéis são únicos, e cada dispositivo aceita no máximo 512 papéis.
- Se `defaultPaper` estiver ausente ou for desconhecido, o primeiro papel será usado.
- Arquivos com mais de 1 MB são recusados.

No nome do arquivo, caracteres que não sejam letras, números, espaços ou `-_()+,` são substituídos por `_`.

## Comandos

- `plottermanager {action, …}` (também disponível em **File ▸ Plotter Manager...**) gerencia os dispositivos:
  - `list` (padrão) lista os dispositivos integrados primeiro.
  - `get {name}` retorna um dispositivo.
  - `new {name, from?, output?, …}` copia o dispositivo indicado em `from` ou cria uma cópia do dispositivo integrado correspondente a `output`, com os papéis padrão.
  - `set {name, description?, output?, resolution?, orientation?, defaultPaper?, paper?: {name, width, height, margins?}, removePaper?}` edita um dispositivo. `paper` adiciona um papel ou substitui aquele que tiver o mesmo nome.
  - `delete {name}` remove um dispositivo.
  - `load {text, name?}` armazena o conteúdo de um arquivo de dispositivo.
  - `save {name, path?}` grava o arquivo em `path` ou retorna seu conteúdo em `text`.

  Os dispositivos integrados não podem ser editados nem excluídos.
- Digitar `PLOTTERMANAGER` ou `-PLOTTERMANAGER` solicita uma opção:
  - `?` lista os dispositivos.
  - `New` solicita um nome e uma saída (`Pdf`, `PNg`, `Svg`).
  - `Delete` solicita um dispositivo.
  - `Paper` solicita um dispositivo, o nome do papel e `width,height` em mm, e adiciona esse papel.
- `pagesetup {layout, device, paper?}` define o dispositivo do layout. `paper` identifica um dos papéis do dispositivo ou um tamanho padrão pelo nome curto (`A3`, `Letter`). As margens do papel passam a definir a área imprimível do layout. Se `paper` não for informado, a troca de dispositivo mantém o papel atual se ele existir no novo dispositivo; caso contrário, usa o papel padrão desse dispositivo. A troca de dispositivo também usa a orientação do dispositivo, a menos que `landscape` seja informado. `device: "None"` remove a seleção do dispositivo.
- `plot {device?, paper?, …}` gera a plotagem usando `device` ou o dispositivo da configuração de página do layout.
  - O dispositivo define o formato de saída e a resolução.
  - `paper` (um dos papéis do dispositivo) define o tamanho e as margens dessa plotagem.
  - Uma plotagem do espaço do modelo sem `paper` usa o papel e a orientação padrão do dispositivo.
  - Para plotagens do Model, `center` é `true` por padrão, preservando a saída centralizada existente. Defina `center: false` com `offset: [x, y]` para posicionar o canto inferior esquerdo dos limites plotados a partir da origem inferior esquerda da área imprimível; os deslocamentos são em milímetros, têm padrão `[0, 0]` e ficam limitados a ±100.000 mm. PDF e PNG aceitam esses controles. Plotagens SVG mantêm a visualização dos limites; um deslocamento ativo é recusado em vez de ser ignorado. Plotagens de layout não usam deslocamentos do modelo.
  - `exportpdf` aceita as mesmas opções e sempre gera PDF.

As plotagens PDF do espaço do modelo ainda não aplicam as margens do papel. Elas ajustam o desenho dentro das margens padrão. As plotagens PNG aplicam essas margens. As plotagens SVG mostram a folha inteira de um layout ou os limites do espaço do modelo.

## Transparência na plotagem

Os objetos são plotados com sua transparência (a própria, a da camada quando definida como ByLayer ou a da referência de bloco quando definida como ByBlock) somente quando essa opção é solicitada, como no AutoCAD:

- `pagesetup {layout, plotTransparency: bool}` armazena a opção "Plot transparency" do layout (desativada por padrão). O Page Setup Manager e a caixa de diálogo Plot têm essa opção.
- `plot {plotTransparency?: bool}` (assim como `exportpdf` e `preview`) substitui essa configuração para uma única plotagem. Nas plotagens do espaço do modelo, a opção fica desativada, a menos que seja solicitada.
- `PLOTTRANSPARENCYOVERRIDE` tem prioridade: 0 nunca plota transparência, 1 (padrão) usa a configuração acima e 2 sempre plota transparência.

As plotagens PDF gravam a transparência como alpha (`/ExtGState` com `/ca` e `/CA`), as PNG fazem a mesclagem e as SVG usam `opacity`. Uma plotagem sem transparência é gravada exatamente como antes. Os sinalizadores de layout de plotagem do DXF não têm um bit para essa opção, portanto ela não é salva em arquivos DXF ou DWG.

Na tela, `TRANSPARENCYDISPLAY` (ativado por padrão; botão Transparency na barra de status) exibe a transparência. As exportações PNG e SVG de um desenho sempre a representam.
