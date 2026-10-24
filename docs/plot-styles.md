# Estilos de plotagem

Uma tabela de estilos de plotagem define como os objetos serão impressos. O CADCraft aplica os estilos às plotagens em PDF (PLOT, EXPORTPDF e qualquer recurso de plotagem que passe por eles). Também os aplica ao layout exibido quando **Display plot styles** está ativado, tanto na tela quanto nas exportações PNG e SVG do layout.

## Tabelas de estilos

| Tipo | Como escolhe o estilo | Estilos |
|---|---|---|
| Dependente de cor (`.ctb`) | Pela cor de índice do objeto (uma cor verdadeira usa a cor de índice mais próxima) | Exatamente 255: `Color_1` a `Color_255` |
| Nomeado (`.stb`) | Pela propriedade **PlotStyle** do objeto | `Normal` primeiro, seguido dos demais nomes |

Um estilo pode definir os seguintes campos:

| Campo | Significado | Valor para “usar o objeto” |
|---|---|---|
| `color` | Cor de saída, no formato `"#rrggbb"` | Ausente ou `null` |
| `grayscale` | Imprime a cor como cinza com base na luminância (pesos Rec. 709) | `false` |
| `screening` | Intensidade da tinta entre 0 e 100%, misturada com o papel branco | `100` |
| `linetype` | Nome do tipo de linha, carregado ou da biblioteca interna; `Continuous` desenha linha contínua | Ausente ou `null` |
| `lineweight` | Centésimos de milímetro, de 0 a 211; usado quando a espessura de linha é plotada | Ausente ou `null` |
| `end`, `join`, `fill` | Extremidade da linha, junção e padrão de preenchimento | `object` |

A cor 7 é impressa em preto.

### Extremidades, junções e padrões de preenchimento

- **`end` e `join`:** definem a aparência de linhas com espessura superior a 0,25 mm no papel. A linha é desenhada como um contorno preenchido: `butt` termina no ponto final; `square` se estende por metade da espessura; `round` adiciona um semicírculo; `diamond` forma uma ponta. A junção `miter` cria um vértice (que vira chanfro quando passa de 10 meias-espessuras), `bevel` corta o canto, `round` arredonda e `diamond` cria uma ponta com meia-espessura. `object` equivale a `round`, pois as linhas já são desenhadas dessa forma.
- **`fill`:** substitui o preenchimento sólido de áreas preenchidas: sólidos 2D, traces, polilinhas largas, hachuras sólidas e textos preenchidos. `solid` e `object` mantêm o preenchimento sólido. Os padrões são definições próprias do CADCraft, dimensionadas no papel e ancoradas à origem da folha. Repetem-se a cada 1 mm.

| Preenchimento | Padrão |
|---|---|
| `checkerboard` | Quadrados de 0,5 mm, alternando os preenchidos |
| `crosshatch` | Barras horizontais e verticais de 0,2 mm |
| `diamonds` | Quadrados de 0,35 mm girados em 45°, a cada 0,7 mm |
| `horizontalBars`, `verticalBars` | Barras com largura equivalente a um terço do período |
| `slantLeft`, `slantRight` | Barras inclinadas `\\` e `/`, com largura de um terço do período |
| `squareDots` | Quadrados de 0,2 mm a cada 0,5 mm |

Wipeouts nunca recebem padrões. Se a plotagem exigir mais de dois milhões de células de padrão, as áreas restantes ficam sólidas. Nos layouts, os padrões são desenhados durante a criação da lista de exibição; em plotagens do espaço do modelo, são desenhados na escala da plotagem.

### Tabelas internas

Estas tabelas são definições próprias do CADCraft, não cópias de arquivos de fornecedores.

| Tabela | Comportamento |
|---|---|
| `default.ctb` | Mantém as propriedades de cada objeto |
| `monochrome.ctb` | Imprime todas as cores em preto |
| `grayscale.ctb` | Imprime todas as cores como tons de cinza baseados na luminância |
| `default.stb` | `Normal` (propriedades do objeto), `Black`, `Screened 50%` e `Thick` (0,50 mm) |
| `monochrome.stb` | `Normal` imprime em preto; `Screened 50%` imprime em preto com 50% de intensidade |

Uma tabela armazenada no desenho tem prioridade sobre uma tabela interna de mesmo nome.

## Estilos nomeados de plotagem

As camadas têm um nome de estilo de plotagem (padrão `Normal`). Os objetos podem usar `ByLayer` (padrão), `ByBlock` ou o nome de um estilo. `ByLayer` usa o estilo da camada. Na camada 0 dentro de um bloco, é usado o estilo da camada da referência do bloco. `ByBlock` usa o estilo da referência do bloco. Se a tabela não contiver o nome escolhido, o objeto é plotado como `Normal`. Uma tabela dependente de cor ignora essa propriedade.

## Comandos

- `pagesetup {layout, plotStyleTable, displayPlotStyles}` define a tabela do layout e a opção **Display plot styles**. Sem um layout, informa as tabelas internas em `plotStyleTables`.
- `plot` / `exportpdf {plotStyleTable}` substitui a tabela definida na configuração da página para uma única plotagem. `"None"` usa as cores dos objetos.
- `plotstyle {name, handles?}` define o estilo de plotagem dos objetos. Sem seleção, define o estilo atual para novos objetos (`CPLOTSTYLE`). Sem `name`, informa o estilo atual e os estilos nomeados.
- `layer.set {name, plotStyle}` define o estilo de plotagem de uma camada.
- `stylesmanager {action, …}` gerencia tabelas: `list`, `get`, `new` (`kind`, `from`), `set` (`style: {name, …fields}`, `removeStyle`, `description`), `delete`, `load` (`path` ou `text`) e `save` (`path` ou retorno de `text`). Digitado ou escolhido em **File ▸ Plot Styles...**, abre o gerenciador de estilos de plotagem.

## Arquivos de tabela

As tabelas são salvas no formato próprio do CADCraft: JSON UTF-8 com um marcador de formato. Campos definidos como “usar as propriedades do objeto” são omitidos.

```json
{
  "cadcraftPlotStyleTable": 1,
  "name": "office.stb",
  "description": "Office pens",
  "kind": "named",
  "styles": [
    { "name": "Normal" },
    { "name": "Pen 5", "color": "#000000", "screening": 40, "lineweight": 35, "linetype": "HIDDEN", "end": "round" }
  ]
}
```

- `kind` aceita `colorDependent` ou `named`.
- `end` aceita `object`, `butt`, `square`, `round` ou `diamond`.
- `join` aceita `object`, `miter`, `bevel`, `round` ou `diamond`.
- `fill` aceita `object`, `solid`, `checkerboard`, `crosshatch`, `diamonds`, `horizontalBars`, `slantLeft`, `slantRight`, `squareDots` ou `verticalBars`.
- `color` também pode ser informado como `[r, g, b]`.

A leitura limita valores potencialmente problemáticos:

- Screening é limitado a 100 e a espessura de linha a 211.
- Uma tabela dependente de cor sempre recebe 255 estilos chamados `Color_n`.
- Uma tabela nomeada recebe nomes únicos (até 1.024), com `Normal` primeiro.
- Arquivos maiores que 4 MB são rejeitados.

Arquivos binários `.ctb` e `.stb` ainda não são lidos. Não foi encontrada uma especificação pública desse formato.

## Nos arquivos DXF

- A tabela de configuração da página é armazenada no grupo 7 de PLOTSETTINGS; **Display plot styles** é a flag 2 do layout (grupo 70).
- Os nomes de estilos seguem a referência DXF. A entrada de dicionário de objetos nomeados `ACAD_PLOTSTYLENAME` é um `ACDBDICTIONARYWDFLT`, cujas entradas apontam para objetos `ACDBPLACEHOLDER` (padrão `Normal`). Camadas e objetos os referenciam pelo grupo 390. Um objeto sem grupo 390 usa `ByLayer`. `ByBlock` não é gravado e volta como `ByLayer` ao reabrir.
- Tabelas armazenadas no desenho ficam no XRECORD `CADCRAFT_PLOTSTYLES` (JSON) do dicionário de objetos nomeados.
