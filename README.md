> **Decode CAD — desenvolvido e mantido pela [Decode Analytics](https://www.instagram.com/decode.analytics/).** Esta edição é construída a partir de código aberto e adaptada para distribuição sob a identidade Decode CAD. Os avisos e atribuições exigidos continuam disponíveis em [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE), [NOTICE](NOTICE) e [ATTRIBUTION.md](ATTRIBUTION.md).

<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ai.storyteller.cadcraft.png" width="256" alt="Logo Decode CAD da Decode Analytics">
</p>

<h1 align="center">Decode CAD</h1>

<p align="center">
  <b>CAD 2D e 3D de código aberto para desenho técnico, documentação e projetos, desenvolvido em Rust.</b>
</p>

<p align="center">
  Ferramentas de desenho, camadas, cotas, hachuras, blocos, leitura e gravação DXF/DWG, exportação e automação — para desktop e navegador, conforme as plataformas disponíveis neste repositório.
</p>

**Tecnologia:** [Rust](https://www.rust-lang.org/) · **Licença:** [MIT](LICENSE-MIT) / [Apache 2.0](LICENSE-APACHE) · **Desenvolvido por:** [Decode Analytics](https://www.instagram.com/decode.analytics/) · [Sobre a Decode Analytics](#desenvolvido-pela-decode-analytics)

## Downloads

A versão **Decode CAD 0.12.1** está disponível para computadores **Windows x64** e **Linux x86_64**.

<p align="center">
  <a href="https://github.com/Kaiqueaurelio/decodecad/releases/download/v0.12.1/decode-cad-0.12.1-windows-x64.msi"><strong>⬇ Baixar para Windows (MSI)</strong></a>
  &nbsp; · &nbsp;
  <a href="https://github.com/Kaiqueaurelio/decodecad/releases/download/v0.12.1/decode-cad-0.12.1-linux-x86_64.AppImage"><strong>⬇ Baixar para Linux (AppImage)</strong></a>
  &nbsp; · &nbsp;
  <a href="https://github.com/Kaiqueaurelio/decodecad/releases/download/v0.12.1/decode-cad-0.12.1-linux-x86_64.deb"><strong>⬇ Linux (Debian/Ubuntu .deb)</strong></a>
</p>

- **Windows:** baixe e abra o arquivo .msi para instalar. Há também uma [versão portátil ZIP](https://github.com/Kaiqueaurelio/decodecad/releases/download/v0.12.1/decode-cad-0.12.1-windows-x64-portable.zip).
- **Linux AppImage:** baixe o arquivo, marque-o como executável nas propriedades ou rode chmod +x decode-cad-0.12.1-linux-x86_64.AppImage e abra-o.
- **Linux Debian/Ubuntu:** abra o arquivo .deb com o instalador de pacotes da distribuição.

Estes pacotes são para processadores x64/x86_64. Consulte as [notas e todos os arquivos da versão 0.12.1](https://github.com/Kaiqueaurelio/decodecad/releases/tag/v0.12.1).


<p align="center">
  <img src="docs/images/ui-apartment.png" alt="Decode CAD com uma planta de apartamento aberta: paredes hachuradas, janelas, portas, contornos de móveis, nomes dos cômodos, tabela de áreas, anotação e cotas arquitetônicas" width="100%">
  <br><sub><b>Planta de apartamento</b> (<a href="examples/apartment.dxf">examples/apartment.dxf</a>): paredes com hachura por ponto selecionado, rótulos MTEXT com fontes TrueType, tabela, chamada múltipla e cotas arquitetônicas — criada com comandos do Decode CAD.</sub>
</p>

<p align="center">
  <a href="#capturas-de-tela">Capturas de tela</a> ·
  <a href="#por-que-usar-o-decode-cad">Por que usar o Decode CAD</a> ·
  <a href="#recursos-disponíveis-atualmente">Recursos disponíveis</a> ·
  <a href="#início-rápido">Início rápido</a> ·
  <a href="#controle-por-agentes-mcp-e-cli">Agentes, MCP e CLI</a> ·
  <a href="#arquitetura">Arquitetura</a> ·
  <a href="#plano-de-desenvolvimento">Plano de desenvolvimento</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#licença-e-créditos">Licença e créditos</a>
</p>

## Capturas de tela

<table>
  <tr>
    <td width="50%"><img src="docs/images/ui-layout.png" alt="Layout de impressão no Decode CAD: folha branca, área imprimível tracejada e janela de visualização com a planta em escala"></td>
    <td width="50%"><img src="docs/images/ui-bracket.png" alt="Decode CAD com desenho técnico de um suporte de fixação, furos, linhas de centro, cotas, corte hachurado, anotações e legenda técnica"></td>
  </tr>
  <tr>
    <td><sub><b>Layouts:</b> espaço do papel com janelas de visualização, configurações de página e impressão em PDF. Clique duas vezes em uma janela para trabalhar no espaço do modelo através dela.</sub></td>
    <td><sub><b>Suporte de fixação:</b> desenho de uma peça em duas vistas, com linhas de centro, linhas ocultas, hachura de corte ANSI31 e legenda técnica.</sub></td>
  </tr>
</table>

## Por que usar o Decode CAD

- **Fluxo de trabalho familiar.** Digite `L`, clique em dois pontos, digite `@5<45` e pressione Enter. A linha de comando, sugestões clicáveis, preenchimento automático, atalhos, captura de objetos, rastreamento polar, modo ortogonal, entrada direta de distância, seleção por janela e cruzamento, alças e repetição pelo botão direito seguem convenções conhecidas de desenho técnico.
- **Arquivos abertos.** O DXF é lido e gravado nativamente, em formato ASCII e binário, das versões R12 a 2018. Arquivos DWG das versões R13 a 2018 podem ser abertos e salvos por meio da biblioteca de código aberto acadrust. Também é possível exportar para SVG e PNG.
- **Aplicativo nativo.** Desenvolvido em Rust e egui, sem Electron nem visualização web incorporada. Há suporte a macOS, Windows, Linux e FreeBSD, além de versão WebAssembly para navegadores, conforme os workflows e artefatos disponíveis.
- **Preparado para automação.** Menus, ferramentas e solicitações podem ser acionados como comandos. Programas e agentes podem usar a linha de comando, comandos JSON, inspeção do desenho e geração de imagens via MCP, canal de controle JSON ou CLI.
- **Código aberto.** O repositório inclui licenças MIT e Apache-2.0. Confira os avisos e a licença de cada componente antes de redistribuir.

## Recursos disponíveis atualmente

O Decode CAD está em desenvolvimento. Consulte [ROADMAP.md](ROADMAP.md) para ver os indicadores de compatibilidade e o estado dos recursos.

| Área | Recursos |
|---|---|
| Área de desenho | Espaço do modelo com grade adaptável, eixos, deslocamento e zoom, cursor em mira, ícone UCS, cubo de visualização e identificação da janela |
| Linha de comando | Sugestões, histórico, preenchimento automático, aliases, coordenadas relativas, entrada direta de distância, repetição de comandos e scripts no estilo `.scr` |
| Desenho | `LINE`, `PLINE`, `CIRCLE`, `ARC`, `RECTANG`, `POLYGON`, `ELLIPSE`, `SPLINE`, `POINT`, `XLINE`, `RAY`, `DONUT`, `TEXT` e `MTEXT` |
| Edição | `ERASE`, `MOVE`, `COPY`, `ROTATE`, `SCALE`, `MIRROR`, `STRETCH`, `OFFSET`, `TRIM`, `EXTEND`, `FILLET`, `CHAMFER`, `BREAK`, `JOIN`, `EXPLODE`, `ARRAY` e `OVERKILL` |
| Precisão | Captura de objetos por extremidade, ponto médio, centro, interseção, perpendicular, tangente e outros pontos; rastreamento polar, modo ortogonal e ajuste à grade |
| Camadas e propriedades | Gerenciador de camadas, visibilidade, congelamento, bloqueio, impressão, cor, tipos e espessuras de linha, além de edição das propriedades de cada objeto |
| Anotações | Comandos de cotagem, estilos e ajustes de cotas, cotas associativas, chamadas múltiplas, tabelas, fontes TrueType e formatação de texto |
| Hachuras e blocos | Limites de hachura definidos por ponto, ilhas, padrões, preenchimentos sólidos e gradientes, blocos, atributos e blocos aninhados |
| Arquivos | Leitura de DXF R12–2018 e gravação R2000; leitura de DWG pela biblioteca acadrust e gravação R2000; geração de PDF, SVG e PNG. Elementos ainda não compatíveis podem não ser preservados ao salvar; consulte [compatibilidade de formatos](docs/file-format-parity.md) |
| Layouts e impressão | Layouts no espaço do papel, janelas de visualização, escala, bloqueio, configurações de página, `PLOT` e `EXPORTPDF` |
| Parametria | Restrições geométricas e dimensionais, `AUTOCONSTRAIN`, parâmetros com expressões, detecção de conflitos e recálculo após edições |
| Alças de edição | Alças interativas; use Espaço para alternar entre esticar, mover, girar, dimensionar e espelhar |
| Idiomas da interface | Inglês, ucraniano, português brasileiro, alemão, espanhol, tcheco, russo, francês, chinês simplificado, italiano, coreano, sueco, árabe, húngaro, turco e grego, com troca em tempo real e seleção automática pelo idioma do sistema; [detalhes](docs/localization.md) |
| Automação | Servidor MCP, canal de controle JSON e `cadcraft-cli` para informações, conversão, execução e comandos |

## Início rápido

As versões para computador e navegador exigem Rust 1.95 ou posterior. A versão mínima declarada para o núcleo do projeto continua sendo Rust 1.90.

```sh
git clone https://github.com/Kaiqueaurelio/decodecad.git
cd decodecad
cargo run --release -p cadcraft -- --sample
```

O comando acima abre um desenho de exemplo. Depois, experimente estes comandos na linha de comando:

```text
line 0,0 @10,0 @0,5 c          triângulo fechado
circle 5,2 1                    círculo
offset 0.25                     selecione o círculo e um lado
zoom e                          ajustar zoom à extensão do desenho
```

Para iniciar a versão web, execute `cd apps/cadcraft-web && trunk serve`. É necessário instalar o Trunk.

### Fontes

O Decode CAD pode usar fontes TrueType/OpenType instaladas no sistema. Compilações configuradas com a entrada opcional `CRAFT_FONTS_DIR` podem incorporar fontes adicionais; consulte os avisos dessas fontes antes de redistribuir. Fontes SHX ainda não são lidas diretamente; textos que as utilizam são desenhados com a fonte vetorial interna ou com uma fonte instalada compatível.

| Variável | Variável de ambiente | Finalidade |
|---|---|---|
| `FONTALT` | `CADCRAFT_FONTALT` | Fonte alternativa quando a fonte definida no estilo não é encontrada. Pode ser indicada pelo nome do arquivo ou da família. Vazia (padrão): usa a fonte vetorial interna. |
| `FONTFALLBACK` | `CADCRAFT_FONTFALLBACK` | Lista de fontes separadas por vírgula para caracteres ausentes na fonte principal. |

O ponto (`.`) limpa qualquer uma dessas configurações.

**Suas próprias fontes.** Fontes `.ttf`, `.otf` e `.ttc` podem ser instaladas no sistema e selecionadas pelo nome do arquivo ou da família em um estilo de texto (`STYLE`) ou no texto MTEXT. Reinicie o Decode CAD depois de instalar fontes, pois as pastas são verificadas na inicialização.

**Textos da direita para a esquerda.** TEXT e MTEXT organizam linhas usando o algoritmo bidirecional Unicode, e textos em escrita árabe usam formas contextuais OpenType. A justificação continua relacionada à caixa de texto. Instale uma fonte compatível com o idioma necessário e consulte a documentação das fontes antes de distribuí-las com o aplicativo.

### Registros de execução

O aplicativo para computador grava os registros no erro padrão e em `logs/cadcraft.log`, dentro da pasta de configurações do sistema. O caminho depende da plataforma: Linux/BSD (`~/.config/cadcraft/logs/`), macOS (`~/Library/Application Support/CADCraft/logs/`) e Windows (`%APPDATA%\CADCraft\logs\`), ou a pasta definida em `CADCRAFT_CONFIG_DIR`. Esse arquivo é útil para anexar a relatos de falhas. A cada inicialização, os registros anteriores são rotacionados; o arquivo atual tem limite de 16 MiB. O parâmetro `--version` não cria arquivo de registro. Os nomes internos dos arquivos e variáveis de ambiente permanecem inalterados por compatibilidade.

| Variável | Efeito |
|---|---|
| `CADCRAFT_CONFIG_DIR` | Usa uma pasta personalizada para as configurações e os registros. |
| `RUST_LOG` | Define o nível de detalhamento dos registros. O padrão é `info` para os módulos do aplicativo e `warn` para os demais. Exemplos: `RUST_LOG=debug`, `RUST_LOG=warn,cadcraft_io=trace`. |

Na versão web, os registros são enviados ao console do navegador.

## Controle por agentes, MCP e CLI

O Decode CAD oferece uma interface de linha de comando para automação, execução sem interface gráfica, conversão de arquivos e controle por agentes.

```sh
# Servidor MCP sem interface gráfica, usando entrada e saída padrão:
cadcraft-cli mcp

# Conectar ao aplicativo em execução (inicie-o com --control 7979):
cadcraft-cli mcp --connect 127.0.0.1:7979

# Executar comandos e exportar um desenho:
cadcraft-cli run --sample --script 'CIRCLE 22,3 1\n' --save out.dxf --export out.png
cadcraft-cli info drawing.dxf
cadcraft-cli convert drawing.dxf drawing.svg
```

As ferramentas MCP incluem `command_line` (digitar na linha de comando), `execute` (executar comandos com JSON), `inspect_drawing`, `query_entities` e `render` (gerar PNG). Quando conectado ao aplicativo, também há `screenshot` e `ui_click`. O canal de controle JSON está descrito em [docs/control-protocol.md](docs/control-protocol.md).

Para usar rotinas LISP, scripts, arquivos de aliases ou controlar o Decode CAD a partir de programas próprios, consulte [docs/customization.md](docs/customization.md).

## Arquitetura

```text
geom ─┐                       geometria f64: arcos, splines, interseções e deslocamentos
dxf   │  (independente)       leitura e gravação de tags DXF
color ┤                       índice de cores e cores reais
doc   ┤                       banco de dados do desenho e armazenamento de entidades
fonts ┤ render                fonte vetorial, layout TEXT/MTEXT e renderização
io    ┤                       conversão DXF, exportação SVG/PNG
engine┤                       sessões, comandos, linha de comando, captura, seleção e desfazer
ui-egui · mcp                 interface egui substituível e servidor MCP
apps: cadcraft · cadcraft-cli · cadcraft-web
```

Os nomes internos dos crates e das ferramentas de build permanecem como estão no código. O comando `cargo xtask ci` verifica formatação, Clippy, testes, atribuição de recursos, organização dos módulos e compilação para WebAssembly.

O aplicativo também oferece recursos de acessibilidade, como rótulos acessíveis, foco visível pelo teclado, opção para mostrar o cursor do sistema e respeito à configuração de redução de animações do sistema operacional.

## Acessibilidade

Na área de desenho, o Decode CAD oculta o cursor do sistema e desenha sua própria mira. Isso pode dificultar o acompanhamento por lupas de tela, gravadores e áreas de trabalho remotas. Ative **View ▸ Accessibility ▸ Show system cursor** ou inicie o aplicativo com a variável de ambiente `CADCRAFT_SYSTEM_CURSOR=1`. No Windows, é possível configurar essa variável com `setx CADCRAFT_SYSTEM_CURSOR 1`.

O Decode CAD respeita a preferência do sistema para reduzir animações. A configuração também pode ser ajustada no menu **View ▸ Accessibility ▸ Animation Effects**, escolhendo usar a configuração do sistema, ativar ou desativar os efeitos.

## Plano de desenvolvimento

Consulte [ROADMAP.md](ROADMAP.md) para ver a etapa atual, os indicadores de compatibilidade e a estimativa de trabalho restante. O documento [docs/gaps.md](docs/gaps.md) lista os recursos e as áreas que ainda precisam ser concluídos. A comparação do fork com o CADCraft original está em [docs/decodecad-parity.md](docs/decodecad-parity.md).

## Outras versões

Consulte a página de [Releases do Decode CAD](https://github.com/Kaiqueaurelio/decodecad/releases) para versões futuras e arquivos adicionais.

## Desenvolvido pela Decode Analytics

O Decode CAD é uma edição mantida pela **[Decode Analytics](https://www.instagram.com/decode.analytics/)**. A identidade visual e a distribuição desta edição são administradas pela Decode Analytics; componentes e trechos derivados de dependências preservam seus respectivos avisos legais e licenças.

## Licença e créditos

**Decode CAD é desenvolvido, personalizado e mantido pela Decode Analytics.** Esta edição possui identidade visual e apresentação próprias, sobre uma base de código aberto. Copyright © 2026 Decode Analytics e colaboradores pelas modificações e contribuições desta edição.

O projeto é disponibilizado sob os termos da licença [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), conforme aplicável. Os avisos legais e de distribuição necessários estão em [NOTICE](NOTICE) e [ATTRIBUTION.md](ATTRIBUTION.md); eles são mantidos para respeitar as licenças do código e dos recursos incluídos.

A marca e os materiais promocionais desta edição são apresentados como **Decode CAD / Decode Analytics**. Os avisos de licença e proveniência permanecem nos arquivos legais do repositório, sem transformar nomes ou marcas de origem em parte da identidade visual do Decode CAD.

### Organização dos painéis

Os painéis Conjuntos de ferramentas, Camadas e Propriedades podem ser agrupados, separados, destacados dentro da janela do aplicativo, fechados e reabertos. A área de desenho permanece visível e não pode ser fechada nem transformada em aba. Arraste o cabeçalho de um painel até uma borda ou outro cabeçalho. O menu de contexto oferece as opções **Destacar painel**, **Mover para grupo** e **Fechar painel**. Os controles existentes reabrem os painéis na última posição salva; **Redefinir organização dos painéis** restaura a disposição inicial. Painéis destacados permanecem dentro da janela atual do aplicativo.
