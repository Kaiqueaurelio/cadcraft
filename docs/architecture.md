# Arquitetura do CADCraft

> **Última revisão:** 2026-10-10 · **Última atualização:** 2026-10-10 · **Alteração:** grande (primeira versão do documento de arquitetura, escrita a partir do código na main em bd2412f) · **Referência:** Autodesk AutoCAD 2027

Como o CADCraft é construído atualmente. Este documento descreve o código em `main`, não planos futuros; os planos estão em [roadmap.md](roadmap.md). Agentes: as regras que preservam essa estrutura (camadas, nunca travar, tudo é um comando e clean-room) estão em [AGENTS.md](../AGENTS.md).

## Workspace

Cerca de 57.000 linhas de Rust em 12 crates de biblioteca, 3 aplicativos e `xtask` (contagem de 2026-10-10).

| Camada | Crate | Linhas | Função |
|---|---|---:|---|
| L0 | `geom` | 2.067 | Vetores 2D/3D em f64, `Mat3`/`Mat4`, limites, linhas, arcos e bulges, círculos, elipses, B-splines (racionais) com interpolação por pontos de ajuste, interseções de linhas/arcos, tesselação e rastreamento de regiões planas para HATCH/BOUNDARY (`region.rs`) |
| L0 | `dxf` | 571 | Leitor/gravador independente de tags DXF: ASCII e binário, com limites de quantidade de tags para entradas maliciosas |
| L1 | `color` | 265 | Tabela AutoCAD Color Index, cores verdadeiras, ByLayer/ByBlock |
| L1 | `doc` | 3.054 | Banco de dados do desenho: cabeçalho, tabelas de símbolos (camadas, tipos de linha, estilos de texto/cota/tabela/multileader), blocos, espaço do modelo, layouts e restrições; `EntityStore` com copy-on-write; biblioteca própria de padrões de hachura e tipos de linha (`library.rs`) |
| L2 | `fonts` | 2.062 | Fonte de desenho de traço único, contornos TrueType/OpenType/`.ttc` via `skrifa`, layout e códigos de formatação de TEXT e MTEXT, fallback por caractere e FONTALT |
| L2 | `render` | 3.300 | Listas de exibição (polilinhas, triângulos e pontos identificados por handles de entidade): tipos de linha, hachuras, gradientes, texto, expansão de blocos, geometria de cotas e recorte de paper space; rasterizador de CPU (`tiny-skia`) para PNG, pré-visualização de plotagem e testes |
| L2 | `constraints` | 2.504 | Solver paramétrico próprio: Gauss–Newton amortecido / Levenberg–Marquardt (`lm.rs`), 12 restrições geométricas e as restrições dimensionais, expressões e parâmetros, inferência AutoConstrain |
| L3 | `io` | 5.124 | Mapeamento DXF ⇄ documento (`dxf_read.rs`, `dxf_write.rs`, xdata do CADCraft em `dxf_ext.rs`), plotagem PDF e exportação SVG/PNG |
| L3 | `dwg` | 149 | DWG por meio de uma ponte DXF: o crate `acadrust` (MPL-2.0, sem modificações) converte bytes DWG ⇄ DXF; nenhum outro componente usa seus tipos. Somente nativo |
| L4 | `engine` | 25.090 | `Session`, registro de comandos (`cmd/*`), analisador da linha de comando e máquinas de prompt, object snaps e polar/ortho (`snap.rs`), seleção (`select.rs`), grips (`grips.rs`), associatividade (`assoc.rs`), variáveis do sistema, unidades, desfazer e índice espacial R-tree (`spatial.rs`) |
| L5 | `ui-egui` | 9.849 | Front-end intercambiável egui: canvas GPU (`gpu.rs`, wgpu), menus e atalhos (`menus.rs`), linha de comando, paletas, gerenciador de propriedades de camadas e outros diálogos, ViewCube, ícones desenhados em código, temas e catálogo de strings da interface (`i18n/`, inglês, ucraniano e alemão), canal de controle JSON (`control.rs`) |
| L5 | `mcp` | 559 | Servidor MCP (JSON-RPC 2.0 por stdio) em uma sessão headless ou conectado a um aplicativo em execução |
| app | `cadcraft` | 1.102 | Aplicativo desktop (eframe/wgpu; DirectX 12 por padrão no Windows) |
| app | `cadcraft-cli` | 823 | `info`, `convert`, `run` (scripts), `commands`, `sample`, `mcp` |
| app | `cadcraft-web` | 88 | Versão WASM da mesma interface (sem DWG) |
| ferramenta | `xtask` | 1.064 | `ci`, `layers`, `assets`, `wasm`, `parity`, `version`, `stats`, geração de ícones |

`cargo xtask layers` verifica a arquitetura em camadas: nenhum componente abaixo de L5 depende de egui, eframe, winit, wgpu ou rfd.

## Modelo de dados

- Um `Drawing` contém variáveis de cabeçalho, tabelas de símbolos, definições de blocos, espaço do modelo e layouts do espaço do papel. As entidades têm handle, camada, cor, tipo de linha, espessura de linha, transparência e um `EntityKind`: Line, Point, Circle, Arc, Ellipse, LwPolyline, Polyline3d, Spline, Ray, XLine, Text, MText, AttDef, Insert (com atributos), Dimension (linear, alinhada, angular, diâmetro, raio, ordenada e comprimento de arco), Leader, MLeader, Hatch (padrão, sólido e gradiente), Solid, Trace, Face3d, Viewport, Image, Wipeout, Table e Unknown (tags DXF brutas).
- Ainda não há sólidos 3D, superfícies, malhas, regiões (ACIS), multilinhas, quadros de tolerância ou referências externas no modelo ([gaps.md](gaps.md)).
- As coleções de entidades usam copy-on-write, então desfazer corresponde a um snapshot do desenho inteiro com baixo custo.
- As coordenadas são unidades do mundo em f64. O trabalho 2D usa WCS; UCS ainda não está disponível.

## Comandos

Toda ação visível ao usuário é um `CommandSpec` (`crates/engine/src/cmd/mod.rs`): id = nome do comando do AutoCAD em letras minúsculas, label, caminho de menu, atalho, aliases, função `run` em JSON, verificação `enabled` e uma máquina de prompt interativo opcional. Há 295 comandos do engine registrados (128 com prompts interativos, contados no código em 2026-10-10), além de cerca de 30 comandos exclusivos da interface em `crates/ui-egui/src/menus.rs`. A linha de comando, os menus, a barra de ferramentas, Tool Sets, scripts, CLI, canal de controle e MCP usam os mesmos comandos. Chamadas JSON nunca abrem diálogos.

## Renderização

`render::build` converte um espaço em uma lista de exibição a cada alteração; a interface envia essa lista à wgpu em lotes (`gpu.rs`). A tolerância de tesselação acompanha a vista. A R-tree (`spatial.rs`) é atualizada por bloco alterado do armazenamento após uma edição, mantendo a seleção perto de 0,001 ms em desenhos com 200 mil entidades. PNG, PDF, SVG e a pré-visualização de plotagem reutilizam a mesma lista de exibição por meio do rasterizador de CPU ou dos gravadores vetoriais.

## Arquivos

DXF é o caminho nativo: `dxf` analisa as tags, `io::dxf_read` as mapeia para o documento e `io::dxf_write` grava um DXF R2000 (AC1015). O DWG é convertido em memória para DXF por `acadrust` e lido pelo mesmo caminho; ao salvar em DWG, o CADCraft grava seu DXF e o converte de volta. Dados exclusivos do CADCraft (restrições, detalhes de associatividade e flags de tabelas) são armazenados em xdata sob o nome da aplicação. Detalhes e lacunas: [file-format-parity.md](file-format-parity.md).

## Controle por agentes

- **Canal de controle:** `cadcraft --control PORT` aceita linhas JSON (`cmdline.input`, `engine.execute`, `ui.screenshot`, `ui.render` etc.); consulte [control-protocol.md](control-protocol.md).
- **MCP:** `cadcraft-cli mcp` (headless) ou `--connect HOST:PORT` (controla o aplicativo em execução); consulte [mcp.md](mcp.md).
- **CLI:** `cadcraft-cli run --script … --export out.png` para processamento em lote headless. Um leitor que fecha stdout antes do fim (`… | head`) descarta o restante da saída, mas o trabalho continua — inclusive comandos posteriores, `--save` e `--export` — e o código de saída ainda indica falhas; outro erro ao gravar em stdout é informado e faz o código de saída ser 1.

## Segurança

Os crates de produção proíbem `unwrap`/`expect`/`panic!` e vetam `unsafe`. `Session::execute` e a entrada interativa são executadas sob `catch_unwind`, restaurando o desenho se um panic escapar. Alocações dimensionadas pela entrada têm limites (quantidade de tags DXF, tamanho DWG, segmentos e ilhas de hachura, `MAX_BLOCK_DEPTH`).

## Histórico de revisões

| Data | Alteração | Resumo |
|---|---|---|
| 2026-10-10 | grande | Primeiro documento de arquitetura integrado, medido a partir do código (tamanho dos crates, tipos de entidade e contagem de comandos) |
