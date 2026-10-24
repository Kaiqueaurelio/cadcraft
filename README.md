> **Edição comunitária de Kaique Aurélio:** este repositório é uma derivação do [CADCraft original](https://github.com/storytold/cadcraft). Consulte [README-FORK.md](README-FORK.md) para conhecer o estado desta edição e as instruções de compilação. Preserve os créditos e as licenças do projeto original.

<p align="center">
  <a href="https://getartcraft.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/artcraft-logo-white.svg">
      <img alt="ArtCraft" src="docs/brand/artcraft-logo.svg" width="200">
    </picture>
  </a>
</p>

<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ai.storyteller.cadcraft.png" width="128" height="128" alt="Ícone do CADCraft: esquadro de desenho e arco de compasso sobre uma grade azul">
</p>

<h1 align="center">CADCraft</h1>

<p align="center">
  <b>Desenho e projeto auxiliados por computador: um aplicativo CAD 2D e 3D moderno, de código aberto e desenvolvido inteiramente em Rust.</b>
</p>

<p align="center">
  Um fluxo de trabalho CAD rápido e de código aberto: linha de comando, captura de objetos, camadas, cotas, hachuras, blocos e desenhos DXF. Funciona nativamente no macOS, Windows, Linux e FreeBSD, além de funcionar no navegador por meio de WebAssembly.<br>
  <i>Desenvolvido pela equipe ArtCraft.</i>
</p>

<p align="center">
  <img alt="Desenvolvido em Rust" src="https://img.shields.io/badge/desenvolvido%20em-Rust-0b6f88?style=flat-square&logo=rust&logoColor=white">
  <img alt="Compatível com macOS, Windows, Linux, FreeBSD e navegador" src="https://img.shields.io/badge/compatibilidade-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20BSD%20%C2%B7%20Web-14a3c7?style=flat-square">
  <img alt="Licença: MIT ou Apache-2.0" src="https://img.shields.io/badge/licença-MIT%20%2F%20Apache--2.0-0b6f88?style=flat-square">
  <img alt="Controle por agentes via MCP" src="https://img.shields.io/badge/agentes-MCP-14a3c7?style=flat-square">
  <img alt="Estado: desenvolvimento inicial" src="https://img.shields.io/badge/estado-desenvolvimento%20inicial-f07a3a?style=flat-square">
  <br>
  <a href="https://sonarcloud.io/project/overview?id=storytold_cadcraft"><img alt="Analisado pelo SonarCloud" src="https://img.shields.io/badge/analisado%20por-SonarCloud-4b9fd5?style=flat-square&logo=sonarqubecloud&logoColor=white"></a>
  <a href="https://coderabbit.ai"><img alt="Revisado pelo CodeRabbit" src="https://img.shields.io/coderabbit/prs/github/storytold/cadcraft?style=flat-square&label=Revisado%20por%20CodeRabbit&logo=coderabbit"></a>
</p>

<p align="center">
  <a href="https://discord.gg/artcraft"><img alt="Participe da comunidade ArtCraft no Discord" src="https://img.shields.io/badge/Participe%20no%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://getartcraft.com/apps/cadcraft"><b>CADCraft no site getartcraft.com</b></a> ·
  <a href="https://getartcraft.com/">ArtCraft</a> ·
  <a href="https://getartcraft.com/apps">Todos os aplicativos criativos</a>
</p>

<br>

<p align="center">
  <img src="docs/images/ui-apartment.png" alt="CADCraft com uma planta de apartamento aberta: paredes hachuradas, janelas, portas, contornos de móveis, nomes dos cômodos, tabela de áreas, anotação e cotas arquitetônicas" width="100%">
  <br><sub><b>Planta de apartamento</b> (<a href="examples/apartment.dxf">examples/apartment.dxf</a>): paredes com hachura por ponto selecionado, rótulos MTEXT com fontes TrueType, tabela, chamada múltipla e cotas arquitetônicas — tudo criado com comandos do CADCraft.</sub>
</p>

> [!NOTE]
> **A ArtCraft é uma comunidade de pessoas criativas de todos os tipos.** Pintores, fotógrafos, cineastas, ilustradores, designers, animadores, entusiastas e pessoas que pegaram um lápis pela primeira vez na semana passada. Se você cria alguma coisa, faz parte da comunidade. [Venha conversar no Discord](https://discord.gg/artcraft).

<p align="center">
  <a href="#capturas-de-tela">Capturas de tela</a> ·
  <a href="#por-que-usar-o-cadcraft">Por que usar o CADCraft</a> ·
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
    <td width="50%"><img src="docs/images/ui-layout.png" alt="Layout de impressão no CADCraft: folha branca, área imprimível tracejada e janela de visualização com a planta em escala"></td>
    <td width="50%"><img src="docs/images/ui-bracket.png" alt="CADCraft com desenho técnico de um suporte de fixação, furos, linhas de centro, cotas, corte hachurado, anotações e legenda técnica"></td>
  </tr>
  <tr>
    <td><sub><b>Layouts:</b> espaço do papel com janelas de visualização, configurações de página e impressão em PDF. Clique duas vezes em uma janela para trabalhar no espaço do modelo através dela.</sub></td>
    <td><sub><b>Suporte de fixação:</b> desenho de uma peça em duas vistas, com linhas de centro, linhas ocultas, hachura de corte ANSI31 e legenda técnica.</sub></td>
  </tr>
</table>

## Por que usar o CADCraft

- **Fluxo de trabalho familiar.** Digite `L`, clique em dois pontos, digite `@5<45` e pressione Enter. A linha de comando, as sugestões clicáveis, o preenchimento automático, os atalhos, a captura de objetos, o rastreamento polar, o modo ortogonal, a entrada direta de distância, a seleção por janela e cruzamento, as alças e a repetição pelo botão direito seguem convenções conhecidas de desenho técnico.
- **Arquivos abertos.** O DXF é lido e gravado nativamente, em formato ASCII e binário, das versões R12 a 2018. Arquivos DWG das versões R13 a 2018 podem ser abertos e salvos por meio da biblioteca de código aberto acadrust. Também é possível exportar para SVG e PNG.
- **Rápido e nativo.** Desenvolvido em Rust e egui, sem Electron nem visualização web incorporada. Há versões para macOS (universal), Windows, Linux e FreeBSD, além de uma versão WebAssembly para navegadores.
- **Preparado para agentes.** Menus, ferramentas e solicitações podem ser acionados como comandos. Agentes podem digitar na linha de comando, executar comandos com JSON, inspecionar o desenho e gerar imagens por meio de MCP, de um canal de controle JSON ou da CLI.
- **Gratuito.** Licenciado sob MIT ou Apache-2.0, sem conta obrigatória nem assinatura.

## Recursos disponíveis atualmente

O CADCraft está em desenvolvimento inicial e acelerado. Consulte [ROADMAP.md](ROADMAP.md) para ver os indicadores de compatibilidade e o estado real dos recursos.

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
git clone https://github.com/Kaiqueaurelio/cadcraft.git
cd cadcraft
cargo run --release -p cadcraft -- --sample
```

O comando acima abre um desenho de exemplo. Depois, experimente estes comandos na linha de comando:

```text
line 0,0 @10,0 @0,5 c          triângulo fechado
circle 5,2 1                    círculo
offset 0.25                     selecione o círculo e um lado
zoom e                          ajustar zoom à extensão do desenho
```

Para iniciar a versão web, execute `cd apps/cadcraft-web && trunk serve`. É necessário instalar o [Trunk](https://github.com/trunk-rs/trunk).

### Fontes

O CADCraft não inclui fontes próprias no repositório: utiliza as fontes TrueType/OpenType instaladas no sistema. Compilações feitas com a opção de fontes [craft-fonts](https://github.com/storytold/craft-fonts) também podem incorporá-las. Fontes SHX ainda não são lidas diretamente; textos que as utilizam são desenhados com a fonte vetorial interna ou com uma fonte instalada compatível.

| Variável | Variável de ambiente | Finalidade |
|---|---|---|
| `FONTALT` | `CADCRAFT_FONTALT` | Fonte alternativa quando a fonte definida no estilo não é encontrada. Pode ser indicada pelo nome do arquivo ou da família. Vazia (padrão): usa a fonte vetorial interna. |
| `FONTFALLBACK` | `CADCRAFT_FONTFALLBACK` | Lista de fontes separadas por vírgula para caracteres ausentes na fonte principal. |

O ponto (`.`) limpa qualquer uma dessas configurações.

**Suas próprias fontes.** Fontes `.ttf`, `.otf` e `.ttc` podem ser instaladas no sistema e selecionadas pelo nome do arquivo ou da família em um estilo de texto (`STYLE`) ou no texto MTEXT. Reinicie o CADCraft depois de instalar fontes, pois as pastas são verificadas na inicialização.

**Textos da direita para a esquerda.** TEXT e MTEXT organizam linhas usando o algoritmo bidirecional Unicode, e textos em escrita árabe usam formas contextuais OpenType. A justificação continua relacionada à caixa de texto. Para persa e outros idiomas com escrita árabe, instale uma fonte compatível, como [Vazirmatn](https://github.com/rastikerdar/vazirmatn), ou configure `FONTFALLBACK`.

### Registros de execução

O aplicativo para computador grava os registros no erro padrão e em `logs/cadcraft.log`, dentro da pasta de configurações do sistema. O caminho depende da plataforma: Linux/BSD (`~/.config/cadcraft/logs/`), macOS (`~/Library/Application Support/CADCraft/logs/`) e Windows (`%APPDATA%\CADCraft\logs\`), ou a pasta definida em `CADCRAFT_CONFIG_DIR`. Esse arquivo é útil para anexar a relatos de falhas. A cada inicialização, os registros anteriores são rotacionados; o arquivo atual tem limite de 16 MiB. O parâmetro `--version` não cria arquivo de registro.

| Variável | Efeito |
|---|---|
| `CADCRAFT_CONFIG_DIR` | Usa uma pasta personalizada para as configurações e os registros. |
| `RUST_LOG` | Define o nível de detalhamento dos registros. O padrão é `info` para os módulos do CADCraft e `warn` para os demais. Exemplos: `RUST_LOG=debug`, `RUST_LOG=warn,cadcraft_io=trace`. |

O código de registro fica em `apps/cadcraft/src/logging.rs`. Na versão web, os registros são enviados ao console do navegador.

## Controle por agentes, MCP e CLI

O CADCraft oferece uma interface de linha de comando para automação, execução sem interface gráfica, conversão de arquivos e controle por agentes.

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

Para usar rotinas LISP, scripts, arquivos de aliases ou controlar o CADCraft a partir de programas próprios, consulte [docs/customization.md](docs/customization.md).

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

Os módulos abaixo de `ui-egui` não dependem de egui, permitindo substituir a interface. O comando `cargo xtask ci` verifica formatação, Clippy, testes, atribuição de recursos, organização dos módulos e compilação para WebAssembly.

O aplicativo também oferece recursos de acessibilidade, como rótulos acessíveis, foco visível pelo teclado, opção para mostrar o cursor do sistema e respeito à configuração de redução de animações do sistema operacional.

## Acessibilidade

Na área de desenho, o CADCraft oculta o cursor do sistema e desenha sua própria mira. Isso pode dificultar o acompanhamento por lupas de tela, gravadores e áreas de trabalho remotas. Ative **Exibir ▸ Acessibilidade ▸ Mostrar cursor do sistema** ou inicie o aplicativo com a variável de ambiente `CADCRAFT_SYSTEM_CURSOR=1`. No Windows, é possível configurar essa variável com `setx CADCRAFT_SYSTEM_CURSOR 1`.

O CADCraft respeita a preferência do sistema para reduzir animações. A configuração também pode ser ajustada no menu **Exibir ▸ Acessibilidade ▸ Efeitos de animação**, escolhendo usar a configuração do sistema, ativar ou desativar os efeitos.

## Plano de desenvolvimento

Consulte [ROADMAP.md](ROADMAP.md) para ver a etapa atual, os indicadores de compatibilidade e a estimativa de trabalho restante. O documento [docs/gaps.md](docs/gaps.md) lista os recursos e as áreas que ainda precisam ser concluídos.

## Downloads

Consulte a página de [versões mais recentes](https://github.com/storytold/cadcraft/releases/latest) para baixar as compilações disponíveis e a página de [todas as versões](https://github.com/storytold/cadcraft/releases) para consultar versões anteriores e suas notas. O marcador `<ver>` nos nomes dos arquivos representa o número da versão; o arquivo `SHA256SUMS.txt` contém as somas de verificação dos arquivos.

> [!WARNING]
> Baixe as versões oficiais pelos [lançamentos do repositório original](https://github.com/storytold/cadcraft/releases) e confira se o proprietário é `storytold`. Antes de executar um arquivo, verifique se sua soma de verificação corresponde à linha dele em `SHA256SUMS.txt`.

### Windows

| Arquitetura | Instalador | Versão portátil |
|---|---|---|
| x64 (Intel/AMD de 64 bits) | `cadcraft-<ver>-windows-x64.msi` | `cadcraft-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon e outros computadores ARM) | `cadcraft-<ver>-windows-arm64.msi` | `cadcraft-<ver>-windows-arm64-portable.zip` |
| x86 (32 bits) | `cadcraft-<ver>-windows-x86.msi` | `cadcraft-<ver>-windows-x86-portable.zip` |

Os instaladores e executáveis são assinados digitalmente. O instalador adiciona o CADCraft ao menu Iniciar e oferece a opção de criar um atalho na área de trabalho.

Se o aplicativo não abrir no Windows, tente iniciar com o backend gráfico Vulkan. No PowerShell, na pasta do executável:

```powershell
$env:WGPU_BACKEND = "vulkan"
& .\cadcraft.exe
Remove-Item Env:WGPU_BACKEND
```

### macOS

| Versão | Arquivo | Observações |
|---|---|---|
| Aplicativo universal (Apple Silicon e Intel) | `cadcraft-<ver>-macos-universal.dmg` | Assinado e autenticado |
| Ferramenta de linha de comando universal | `cadcraft-cli-<ver>-macos-universal.zip` | Assinada e autenticada |

### Linux

| Formato | x86_64 | aarch64 (ARM64) | Observações |
|---|---|---|---|
| AppImage | `cadcraft-<ver>-linux-x86_64.AppImage` | `cadcraft-<ver>-linux-aarch64.AppImage` | Executável portátil |
| Flatpak | `cadcraft-<ver>-linux-x86_64.flatpak` | `cadcraft-<ver>-linux-aarch64.flatpak` | Instalação isolada; use `flatpak install --user <arquivo>` |
| Debian/Ubuntu | `cadcraft-<ver>-linux-x86_64.deb` | `cadcraft-<ver>-linux-aarch64.deb` | Pacote DEB |
| Fedora/RHEL/openSUSE | `cadcraft-<ver>-linux-x86_64.rpm` | `cadcraft-<ver>-linux-aarch64.rpm` | Pacote RPM |
| Arquivo compactado | `cadcraft-<ver>-linux-x86_64.tar.gz` | `cadcraft-<ver>-linux-aarch64.tar.gz` | Extraia para a pasta desejada |

Para RISC-V (riscv64), há também `cadcraft-<ver>-linux-riscv64.tar.gz`, compilado de forma cruzada; exige glibc 2.39 ou posterior.

### FreeBSD

| Arquitetura | Arquivo |
|---|---|
| x86_64 | `cadcraft-<ver>-freebsd-x86_64.tar.gz` |

### Navegador (WebAssembly)

| Versão | Arquivo | Observações |
|---|---|---|
| Site estático | `cadcraft-web-<ver>.zip` | Funciona em navegadores modernos e pode ser hospedado em qualquer servidor de arquivos estáticos |

## Aplicativos criativos da ArtCraft

O CADCraft faz parte dos **Aplicativos criativos** da equipe [ArtCraft](https://getartcraft.com/): ferramentas criativas gratuitas e de código aberto, desenvolvidas em Rust e capazes de funcionar de forma independente.

| Aplicativo | Finalidade | Código-fonte | Saiba mais |
|---|---|---|---|
| **PhotoCraft** | Edição de imagens: camadas, máscaras, texto e arquivos PSD | [GitHub](https://github.com/storytold/photocraft) | [Site](https://getartcraft.com/apps/photocraft) |
| **VectorCraft** | Ilustração vetorial | [GitHub](https://github.com/storytold/vectorcraft) | [Site](https://getartcraft.com/apps/vectorcraft) |
| **FilmCraft** | Edição de vídeo, cor e áudio | [GitHub](https://github.com/storytold/filmcraft) | [Site](https://getartcraft.com/apps/filmcraft) |
| **LightCraft** | Biblioteca de fotos e revelação de arquivos RAW | [GitHub](https://github.com/storytold/lightcraft) | [Site](https://getartcraft.com/apps/lightcraft) |
| **PdfCraft** | Leitura, organização e proteção de PDFs | [GitHub](https://github.com/storytold/pdfcraft) | [Site](https://getartcraft.com/apps/pdfcraft) |
| **EffectCraft** | Animação gráfica e efeitos visuais | [GitHub](https://github.com/storytold/effectcraft) | [Site](https://getartcraft.com/apps/effectcraft) |
| **DesignCraft** | Diagramação e publicação | [GitHub](https://github.com/storytold/designcraft) | [Site](https://getartcraft.com/apps/designcraft) |
| **CADCraft** | Desenho técnico e projeto auxiliados por computador | [GitHub](https://github.com/storytold/cadcraft) | [Site](https://getartcraft.com/apps/cadcraft) |

A [ArtCraft](https://getartcraft.com/) também oferece um estúdio de criação de imagens e vídeos com inteligência artificial.

<p align="center">
  <a href="https://discord.gg/artcraft"><img alt="Participe da comunidade ArtCraft no Discord" src="https://img.shields.io/badge/Participe%20no%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Venha criar com a gente</h3>

<p align="center">
  O Discord é o espaço da comunidade para compartilhar projetos, pedir ajuda, relatar problemas e sugerir melhorias. Pessoas de todas as áreas e níveis de experiência são bem-vindas.
</p>

<p align="center">
  <a href="https://discord.gg/artcraft"><b>discord.gg/artcraft</b></a> ·
  <a href="https://getartcraft.com/">getartcraft.com</a> ·
  <a href="https://getartcraft.com/apps">Aplicativos criativos</a> ·
  <a href="https://getartcraft.com/apps/cadcraft">CADCraft</a>
</p>

## Licença e créditos

O CADCraft é disponibilizado sob a licença [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), à sua escolha. Direitos autorais © 2026 ArtCraft Team e colaboradores do CADCraft. Os avisos obrigatórios estão em [NOTICE](NOTICE).

Fontes, ícones, imagens e outros recursos incluídos mantêm suas próprias licenças de código aberto. Consulte [ATTRIBUTION.md](ATTRIBUTION.md) para ver autores, origens e licenças.

Os ícones do CADCraft, sua fonte vetorial de desenho técnico, os padrões de hachura e os tipos de linha são criações originais. Os desenhos de exemplo também são gerados por código.

O nome ArtCraft, sua marca nominativa e seus logotipos em `docs/brand/` são marcas registradas da equipe ArtCraft e não estão cobertos pela licença do código. Só podem ser usados sem alterações e como parte deste repositório e do CADCraft, conforme `docs/brand/LICENSE-brand.txt`. Derivações e versões modificadas devem removê-los.

<p align="center">
  <a href="https://getartcraft.com/"><img alt="ArtCraft" src="docs/brand/artcraft-mark.svg" width="28"></a><br>
  <sub>Feito pela equipe e pela comunidade <a href="https://getartcraft.com/">ArtCraft</a>.</sub>
</p>

### Organização dos painéis

Os painéis Conjuntos de ferramentas, Camadas e Propriedades podem ser agrupados, separados, destacados dentro da janela do aplicativo, fechados e reabertos. A área de desenho permanece visível e não pode ser fechada nem transformada em aba. Arraste o cabeçalho de um painel até uma borda ou outro cabeçalho. O menu de contexto oferece as opções **Destacar painel**, **Mover para grupo** e **Fechar painel**. Os controles existentes reabrem os painéis na última posição salva; **Redefinir organização dos painéis** restaura a disposição inicial. Painéis destacados permanecem dentro da janela atual do aplicativo.
