# CADCraft e agentes de IA (MCP)

O CADCraft implementa o [Model Context Protocol](https://modelcontextprotocol.io) por meio de stdio.

## Como iniciar

```sh
# Sem interface: sessão de desenho no próprio processo, sem janela.
cadcraft-cli mcp

# Conectado ao aplicativo em execução (acompanhe o desenho do agente):
cadcraft --control 7979 &
cadcraft-cli mcp --connect 127.0.0.1:7979
```

Um cliente MCP normalmente inicia seus servidores ao iniciar, muitas vezes antes de o aplicativo estar aberto. Com `--lazy`, a ponte inicia mesmo assim e tenta alcançar o aplicativo na primeira chamada de ferramenta; até receber resposta, cada chamada falha com uma mensagem explicando como abri-lo. A opção `--launch` também abre o CADCraft no macOS com essa porta de controle quando não encontra uma instância e aguarda a conexão:

```json
{ "mcpServers": { "cadcraft": { "command": "cadcraft-cli", "args": ["mcp", "--connect", "127.0.0.1:7979", "--launch"] } } }
```

A opção `--connect` exige o endereço `HOST:PORT` do aplicativo. Sem ele (ou com um argumento desconhecido), `cadcraft-cli mcp` encerra com erro em vez de iniciar silenciosamente uma sessão sem interface. Isso evita que um cliente destinado ao aplicativo altere por engano outro desenho. Se o aplicativo não puder ser alcançado, a inicialização também falha.

Exemplo de configuração de cliente (Claude Code / Claude Desktop):

```json
{ "mcpServers": { "cadcraft": { "command": "cadcraft-cli", "args": ["mcp"] } } }
```

## Ferramentas disponíveis

| Ferramenta | Função |
|---|---|
| `command_line {text}` | Digita na linha de comando como uma pessoa: `line 0,0 @10,0 @0,5 c`, `circle 5,2 1`, `offset 0.5`, `zoom e`. Espaços funcionam como Enter; texto vazio também envia Enter. Retorna o prompt, as palavras-chave e a saída. |
| `script {text}` | Executa um script de várias linhas, como um arquivo `.scr`. |
| `execute {command, params}` | Executa qualquer comando com parâmetros JSON, sem abrir diálogos. `list_commands` mostra identificadores e documentação dos parâmetros. |
| `list_commands {filter?}` | Catálogo de comandos; o filtro procura parte do identificador, do rótulo ou de um alias. |
| `inspect_drawing {entities?, limit?}` | Retorna contagens, limites geométricos, camadas, estilos, blocos, layouts, seleção, histórico de desfazer e entidades. |
| `query_entities {type?, layer?, window?, crossing?, limit?, offset?}` | Consulta entidades filtradas com identificadores e geometria. `window` usa `[[x0,y0],[x1,y1]]` para localizar objetos dentro da área ou que a cruzam; `crossing: false` exige que estejam inteiramente dentro. `count` é o total de correspondências; `returned` é o tamanho da página (`offset`, `limit`, padrão 500). |
| `execute {"command": "shapes", "params": {...}}` | Analisa formas fechadas (polilinhas fechadas, círculos, elipses completas e splines fechadas), incluindo perímetro, área, centroide, sentido, primeira auto-interseção e vértices coincidentes, com precisão para arcos e bulges. Outras entidades são contabilizadas em `skipped`, com o motivo. `selfCheck` informa o que foi encontrado, analisado ou ignorado e os limites atingidos. |
| `execute {"command": "usage", "params": {...}}` | Consulta somente leitura das camadas, blocos, estilos de texto e tipos de linha: mostra quais objetos os usam no espaço do modelo, nos layouts e nas definições de blocos, além de indicar itens vazios ou não utilizados. `summary` resume os tipos e `unused` lista o que PURGE ofereceria. A varredura lê no máximo 5.000.000 objetos; acima disso, informa `incomplete: true`. |
| `execute {"command": "loops", "params": {...}}` | Detecta loops fechados implícitos em cadeias de LINE e ARC. Retorna identificadores dos elementos, perímetro, área e sentido de cada loop; cadeias abertas com distância entre extremidades e junções com três ou mais extremidades. O pareamento de extremidades tem limite e informa quando foi atingido, em vez de inferir resultados incompletos. |
| `query_entities {type?, layer?, selected?, window?, crossing?, limit?, offset?}` | Consulta entidades com identificadores e geometria. `selected: true` limita os resultados à seleção atual no espaço ativo; seleção vazia retorna zero entidades. Filtros são aplicados antes da paginação. |
| `render {width?, height?, fit?, window?}` | Gera uma imagem PNG do desenho ou apenas da área `window` indicada, útil para uma folha ou cômodo de um desenho grande. |
| `find_text {contains?, layer?, window?, limit?, offset?}` | Localiza textos legíveis (TEXT, MTEXT sem códigos de formatação, atributos de bloco e multileaders), retornando conteúdo, ponto de inserção, altura, rotação, estilo, camada e identificador. A consulta não altera o desenho. |
| `new_drawing {metric?}`, `open {path}`, `save {path?}`, `cancel` | Cria, abre, salva ou cancela. Salvar com caminho `.svg`, `.png` ou `.pdf` exporta sem renomear o desenho nem descartar alterações não salvas. |
| `screenshot`, `ui_inspect`, `ui_click {x, y}` | Ferramentas disponíveis somente quando conectado ao aplicativo com interface. |

Recursos MCP: `cadcraft://drawing` (inspeção em JSON) e `cadcraft://commands` (catálogo).

Os argumentos são validados conforme o esquema de entrada de cada ferramenta. Uma ferramenta desconhecida, argumentos que não sejam um objeto, argumento obrigatório ausente ou tipo incorreto geram erro JSON-RPC `-32602` (parâmetros inválidos). Assim, `command_line {}` não envia Enter por acidente. Se a ferramenta executar e falhar (por exemplo, ponto inválido ou comando desconhecido), retorna um resultado com `isError: true` e a mensagem.

## Exemplo de sessão

1. `command_line {"text": "rectang 0,0 100,60"}`
2. `execute {"command": "circle", "params": {"center": [10, 10], "radius": 4}}`
3. `command_line {"text": "dimlinear 0,0 100,0 50,-8"}`
4. `inspect_drawing {}` — confira as contagens e a geometria.
5. `render {}` — visualize o resultado.

O teste de ponta a ponta em `crates/mcp/src/tests.rs` executa uma sessão semelhante e verifica o resultado exclusivamente pelo MCP.
