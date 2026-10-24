# Protocolo de controle do CADCraft

Inicie o aplicativo desktop com `--control PORT` (ou `CADCRAFT_CONTROL_PORT=PORT`). Ele escuta em `127.0.0.1:PORT` linhas JSON: uma solicitação por linha e uma resposta por linha. A porta informada com `--control` tem prioridade sobre a variável; usar `--control` sem valor mantém a porta definida na variável.

```json
{"id": 1, "method": "cmdline.input", "params": {"text": "circle 0,0 5"}, "token": "<session token>"}
{"id": 1, "ok": true, "result": {"prompt": "Command:", "output": ["Command: CIRCLE"], ...}}
```

Toda solicitação precisa conter o `token` da sessão. Como o TCP de loopback não distingue usuários locais do sistema operacional, ao iniciar o servidor de controle o aplicativo grava um token imprevisível por sessão em um arquivo acessível somente pelo proprietário e recusa (`unauthorized`) qualquer solicitação sem o token correspondente. O arquivo se chama `cadcraft-control-<port>.token` e fica no diretório de execução do usuário (`$XDG_RUNTIME_DIR`, ou `~/.cache` como alternativa; `%LOCALAPPDATA%` no Windows); o caminho é registrado no log durante a inicialização. O cliente lê o token desse arquivo — o que exige acesso ao sistema de arquivos do próprio usuário do desktop — e o inclui em cada linha. A ponte MCP (`cadcraft-cli mcp --connect`) faz isso automaticamente.

Erros são retornados como `{"id": …, "ok": false, "error": "message"}`. O erro de um método desconhecido aponta para `control.methods`, que lista os métodos atendidos pelo servidor.

Uma linha que não seja um objeto JSON com um campo `method` do tipo string (uma solicitação HTTP, texto que não seja JSON ou bytes que não sejam UTF-8) recebe uma resposta de erro e faz o servidor fechar a conexão; nada enviado depois dessa linha é executado. Linhas em branco são ignoradas.

O servidor limita os recursos que um cliente pode consumir: no máximo 16 conexões simultâneas (as excedentes recebem uma linha de erro e são fechadas), 256 MiB por linha de solicitação (linhas maiores encerram a conexão), 64 solicitações na fila (as excedentes recebem `{"ok": false, "error": "busy: …", "state": "not-run"}`) e 10 minutos de inatividade ou 30 segundos para uma gravação de resposta bloqueada antes de o socket ser fechado.

O servidor aguarda até 60 s para que o aplicativo responda. Depois disso, retorna timeout, e o campo `state` informa se é seguro reenviar a solicitação:

- `{"id": …, "ok": false, "error": "timeout", "state": "not-run"}`: o aplicativo ainda não havia iniciado a solicitação. Ela é cancelada e nunca será executada, então é seguro enviá-la novamente.
- `{"id": …, "ok": false, "error": "timeout", "state": "may-have-run"}`: o aplicativo já havia começado a executar a solicitação. Ela ainda pode terminar e alterar o desenho, mas o resultado tardio é descartado. Inspecione o desenho (`drawing.inspect`) antes de reenviar.

A tabela abaixo descreve os métodos de nível superior do protocolo de controle: nomes usados no campo `method` da solicitação. `engine.commands` retorna um catálogo separado de comandos, não uma lista de métodos adicionais.

| Método | Parâmetros | Função |
|---|---|---|
| `cmdline.input` | `{text}` | Digita uma linha na linha de comando exatamente como um usuário. Inicia comandos pelo nome/alias e responde a prompts (pontos `x,y`, `@dx,dy`, `@d<a`, distâncias, palavras-chave, vazio = Enter). Espaços funcionam como Enter, exceto em prompts de texto. |
| `cmdline.script` | `{text}` | Executa um script de várias linhas (como arquivos `.scr`). |
| `cmdline.key` | `{key: "enter"|"escape"}` | Pressiona Enter ou Escape. Responde como `cmdline.input`: estado, `output` e `error` quando Enter é recusado. |
| `cmdline.state` | — | Prompt atual, palavras-chave, tipos de entrada aceitos e trecho final do histórico. |
| `engine.execute` | `{command, params}` | Executa qualquer comando com parâmetros JSON. Nunca abre um diálogo. `command` é um alias deste método, com os mesmos parâmetros e resultado. |
| `engine.commands` | — | Todos os comandos: id, label, caminho do menu, atalho, aliases, documentação dos parâmetros e estado enabled. |
| `drawing.inspect` | `{entities?, limit?}` | Resumo do desenho (contagens, limites, camadas, estilos, blocos, layouts, seleção e desfazer) e entidades com handles e geometria. |
| `ui.inspect` | — | Estado da interface, retângulo do canvas, vista, cursor e snap, contadores de desempenho; `closePrompt`: desenho ao qual se refere um prompt "Save changes?" (null quando não há nenhum); e foco real do teclado egui (`focus.id`, `focus.label`, `focus.role`). |
| `ui.elements` | `{label?, role?, focused?}` | Controles observados na barra de ferramentas, abas, desenho e paleta de camadas do quadro mais recente. Os filtros exigem correspondência exata. Cada item contém o `id` egui real, label, role, retângulo `[x,y,width,height]`, estados enabled/visible/selected e foco atual. |
| `ui.menu.list` / `ui.menu.invoke` | `{command}` | Árvore de menus; aciona um item como um clique (interativo). |
| `ui.pointer` | `{x, y, space?: "world"|"screen", button?, action?: "click"|"move"}` | Clica na área de desenho (coordenadas do mundo por padrão). O botão direito equivale a Enter. |
| `ui.click` / `ui.move` | `{x, y, button?, shift?, ctrl?}` | Eventos reais de ponteiro egui em pontos de tela (alcançam todos os widgets). |
| `ui.drag` | `{x, y, to: [x, y], button?}` | Executa pressionar, mover e soltar de verdade (por exemplo, arrastar a borda superior da linha de comando ou o anel do ViewCube). |
| `ui.key` / `ui.text` | `{key, cmd?, shift?, alt?}` / `{text}` | Entrada sintética de teclado. |
| `ui.set` | Campos de UiState | Mostra/oculta paletas, barras de ferramentas, linha de comando, ViewCube etc.; `theme: "system"|"light"|"dark"` (também `engine.execute` com `ui.theme {theme}`); `ui.inspect` informa o tema exibido (`theme`: light/dark). `animations: "system"|"on"|"off"` (também `ui.animations {animations}`) exibe ou oculta efeitos de animação (transições de diálogos e menus, rolagem suavizada); `system` segue a configuração do sistema operacional. `ui.inspect` informa se as animações estão ativas (`animations`: true/false). `saveFormat: "dxf"|"dwg"` (também `ui.saveformat {format}`) define o formato sugerido por Save As para novos desenhos; `saveVersion: "2000"`…`"2018"` (também `ui.saveversion {version}`) define a versão usada ao salvar (SAVEAS também aceita `version` e `binary` diretamente). |
| `ui.resize` | `{width, height}` | Redimensiona a janela. |
| `ui.focus` | — | Ativa a janela nativa. Não transfere o foco para um widget. |
| `ui.screenshot` | `{path?}` | Captura PNG da janela (exige que um quadro tenha sido apresentado). |
| `ui.render` | `{path?, width?, height?, fit?}` | Renderização headless. `fit` (true por padrão) enquadra o desenho inteiro; `false` usa a vista atual. Retorna `{pngBase64, width, height, bytes, path?}`. |
| `app.open` / `app.save` / `app.quit` | `{path}` | Operações com arquivos. |
| `control.methods` | — | Todos os métodos atendidos pelo servidor: `{method, params, description}`. O modo headless (`cadcraft-cli mcp`) lista apenas os métodos que disponibiliza. |
| `ping` | — | Retorna `"pong"`. Verificação simples de integridade. |

Para executar um comando listado por `engine.commands`, informe seu id em `params.command` e seus argumentos no objeto aninhado `params.params`, usando `engine.execute` como método de nível superior. Isso inclui `open` e `document.bytes), além dos comandos de interface no catálogo do aplicativo desktop. `drawing.inspect` também é um comando do engine, mas possui um método explícito de nível superior; um id de comando com ponto no nome não se torna automaticamente um método do protocolo.

Por exemplo, inspecione o desenho e liste os comandos diretamente; depois, abra um desenho e obtenha seus bytes DXF via `engine.execute` (uma solicitação por linha):

```json
{"id":2,"method":"drawing.inspect","params":{}}
{"id":3,"method":"engine.commands","params":{}}
{"id":4,"method":"engine.execute","params":{"command":"open","params":{"path":"examples/apartment.dxf"}}}
{"id":5,"method":"engine.execute","params":{"command":"document.bytes","params":{}}}
```

O exemplo de `open` pressupõe que o aplicativo foi iniciado na raiz do repositório; caso contrário, use o caminho de um desenho existente e acessível. `document.bytes` usa DXF por padrão e retorna bytes codificados em base64 no campo `data` do resultado. O alias `command` pode ser usado da mesma forma:

```json
{"id":6,"method":"command","params":{"command":"document.bytes","params":{}}}
```

Enviar `open` ou `document.bytes` como `method` de nível superior retorna erro `unknown method`: o protocolo despacha explicitamente os métodos e não recorre ao registro de comandos como alternativa. `app.open` é um método conveniente de nível superior para abrir um desenho com `{path}`.

Para ler objetos selecionados como geometria estruturada em uma única consulta, execute `entities` com `selected: true` (booleano; omitido ou false mantém o comportamento de seleção sem filtro):

```json
{"id":7,"method":"engine.execute","params":{"command":"entities","params":{"selected":true,"limit":1000}}}
```

Somente entidades selecionadas no espaço ativo são retornadas; uma seleção vazia retorna uma página vazia. O filtro de seleção é combinado com `type`, `layer` e `window` antes de `offset`/`limit`. A resposta existente não muda: `count` é o total de correspondências, `returned` é o tamanho da página e `entities` contém handles, camadas e geometria. Essa consulta não altera a seleção, o conteúdo do desenho nem o histórico de desfazer. `entities` é um comando, não um método de nível superior do protocolo.

O servidor MCP (`cadcraft-cli mcp --connect 127.0.0.1:PORT`) encapsula esse protocolo; no modo headless (`cadcraft-cli mcp`), ele oferece os mesmos métodos por meio de uma sessão em processo. Os nomes das ferramentas MCP são outra interface: `execute` corresponde a `engine.execute`, `list_commands` a `engine.commands` e a ferramenta MCP `open` a `app.open`. Consulte [mcp.md](mcp.md).

`ui.key` enfileira eventos reais de tecla egui; a resposta confirma o enfileiramento. Consulte `ui.inspect` ou `ui.elements` para verificar o foco ou estado resultante antes de enviar a próxima ação. Use Tab e Shift-Tab para percorrer os controles e Enter/Space para ativá-los. O registro observado não é a árvore de acessibilidade do sistema operacional e não comprova aceitação de entrada física ou por leitor de tela. Controles não registrados ainda informam seu id real de foco, com label/role null.

O host nativo restaura escolhas de layout a partir de `workspace.json` e salva alterações atomicamente. Ele armazena visibilidade de paletas/conjuntos de ferramentas, conjunto selecionado, grupos recolhidos, altura do histórico da linha de comando e outras preferências da interface. Não armazena desenhos, buffers de comando, diálogos nem a aba Start atual. O diretório padrão é `~/Library/Application Support/CADCraft` no macOS, `%APPDATA%/CADCraft` no Windows e `$XDG_CONFIG_HOME/cadcraft` ou `~/.config/cadcraft` nos demais sistemas. Defina `CADCRAFT_CONFIG_DIR` para usar um perfil isolado; um valor explicitamente vazio desativa leitura e gravação do workspace. Arquivos inválidos, grandes demais ou de versão mais nova são reportados e mantidos intactos, com a persistência desativada nessa inicialização. Corrija ou mova o arquivo antes de reiniciar para voltar a salvar as preferências do workspace.

## Encaixe de painéis

Tool Sets, Layers e Properties podem ser reorganizados em grupos, divididos, flutuados dentro da janela do aplicativo, fechados e reabertos. O canvas de desenho permanece visível e não pode ser fechado nem transformado em aba. A visualização do documento/sessão é protegida. Layouts, abas ativas, visibilidade dos painéis e retângulos flutuantes são salvos junto das preferências existentes da interface. Essas operações não editam o conteúdo dos documentos.

IDs de painel: `toolSets`, `layers`, `properties`, `externalReferences`. A área central protegida é `canvas`.

`ui.dock` aceita `operation`; operações básicas por painel também exigem `panel`. A operação avançada `resizeSplit` usa um caminho de divisão (`path`) em vez de um painel. As operações básicas são `open`, `close`, `activate`, `float` (com `rect: [x, y, width, height]` finito e positivo) e `move` (com `target` e `zone: center|left|right|top|bottom`). A zona central agrupa com o alvo; as zonas laterais criam uma divisão ao lado dele. `ui.dock.reset` restaura a organização padrão. Movimentos inválidos retornam um erro sem alterar o layout.

A paleta QuickCalc (`quickCalc`) não faz parte da organização padrão. Os comandos digitados `QUICKCALC`/`QC` e `ui.panel.quickcalc` abrem e ativam explicitamente a paleta: ela flutua na primeira vez e depois reabre no local salvo. **Window > QuickCalc** e **Ctrl/Cmd+8** acionam `ui.toggle.quickcalc`, fechando o painel se estiver aberto e reabrindo-o caso contrário. O atalho também funciona quando o foco está em um campo de texto. Alternar a paleta preserva o comando de desenho em execução e a entrada pendente da linha de comando. A expressão, o histórico e as variáveis da calculadora permanecem durante a sessão, mas não são salvos entre reinicializações.

Acione esses comandos de interface por meio de `engine.execute`, por exemplo: `{"command":"ui.dock","params":{"operation":"float","panel":"layers","rect":[100,100,320,440]}}`.

As operações avançadas de `ui.dock` usam a mesma transação validada dos gestos do ponteiro: `moveFloating` atualiza `rect`; `resizeSplit` aceita um caminho booleano de filho (`path`) e `size` (`{"Ratio":0.5}`, `{"FixedFirst":280}` ou `{"FixedSecond":280}`); `setStackOpen` recebe `open`; e `resizeStack` recebe uma altura em pontos ou null. Um `move` central pode incluir `before` (id de painel ou null para acrescentar ao final) para definir a ordem das abas. A forma equivalente `{"action": ...}` aceita uma ação compartilhada de encaixe serializada; essa também é a forma emitida pelo renderizador. As regras do canvas protegido e dos painéis específicos do aplicativo se aplicam às duas formas.

## Respostas da linha de comando

O aplicativo e a sessão headless retornam os mesmos campos.

`cmdline.state`, `cmdline.key` e `cmdline.input` retornam o estado da linha de comando:

| Campo | Valor |
|---|---|
| `prompt` | Texto do prompt (`"Command:"` quando ocioso). |
| `running` | Id do comando em execução ou `null`. |
| `keywords` | Opções do prompt atual. |
| `accept` | Tipos de entrada aceitos pelo prompt atual ou `null` quando ocioso. |
| `buffer` | Texto digitado na linha de comando, mas ainda não enviado (sempre `""` no modo headless). |
| `history` | As últimas 20 linhas do histórico. |
| `historyExpanded` | Indica se o histórico está expandido (sempre `false` no modo headless). |

`cmdline.input` acrescenta:

- `output`: linhas de histórico adicionadas pela entrada, incluindo a mensagem de erro quando a linha é recusada.
- `error`: mensagem de erro, somente quando a linha é recusada (comando desconhecido, entrada inválida etc.).

Uma linha recusada ainda retorna `"ok": true`, porque o texto foi digitado. Verifique o campo `error`.

`app.open` sem um `path` do tipo string falha com `"missing path"` antes da execução de qualquer comando.

## Idioma da interface

Selecione um idioma com `engine.execute` e o comando de interface `ui.language`:

```json
{"id":1,"method":"engine.execute","params":{"command":"ui.language","params":{"lang":"uk"}}}
```

Os valores aceitos são `auto`, `en`, `uk`, `pt-BR`, `de`, `es`, `cs`, `ru`, `fr`, `zh`, `it`, `ko`, `sv`, `ar` e `hu`. Para chinês, é necessária uma fonte CJK instalada; para árabe, uma fonte árabe instalada; para coreano, uma fonte que cubra a interface em coreano, fornecida pelo pacote opcional craft-fonts ou pelo sistema operacional. Sem uma fonte adequada, a preferência salva é mantida, mas o idioma exibido passa a ser `en`. `ui.set` também aceita `{"interfaceLanguage":"uk"}`; `ui.inspect` informa a preferência salva no estado da interface. Escolhas inválidas não alteram o idioma atual. Os menus, rótulos de comandos e textos de prompt retornados pelo canal de controle mantêm o texto-fonte em inglês. Consulte [localization.md](localization.md).
