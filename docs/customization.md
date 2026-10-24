# Personalizando e automatizando o CADCraft

Veja o que já é possível usar para automatizar o CADCraft ou adaptar complementos escritos para outros programas CAD — e o que ainda não está disponível.

| Você tem | No CADCraft | Mais informações |
|---|---|---|
| Rotinas LISP (`.lsp`) | Execute com APPLOAD ou `(load "name")`. A linguagem central, `command`, listas de entidades, conjuntos de seleção, entradas `get…` e comandos `c:` funcionam. | [autolisp.md](autolisp.md) |
| LISP de inicialização (`acad.lsp`, `acaddoc.lsp`) | Carregado da pasta de suporte na inicialização e para cada desenho. | [autolisp.md](autolisp.md#at-startup) |
| Scripts (`.scr`) | Use SCRIPT, `cadcraft-cli run --script-file` ou envie o texto do script por MCP e pelo canal de controle. | Abaixo |
| Aliases de comandos (`.pgp`) | `ALIASEDIT {file}` carrega um arquivo de aliases para a sessão; REINIT o lê novamente. | Abaixo |
| Plug-ins compilados (C++, .NET) | Ainda não são compatíveis. Está planejada uma API de plug-ins escritos em Rust e compilados para WebAssembly. | [gaps.md](gaps.md) (E4) |
| Programas que controlam o CAD externamente | O canal de controle JSON, o servidor MCP e `cadcraft-cli` podem ser usados por qualquer linguagem capaz de abrir um socket TCP ou iniciar um processo. | Abaixo |

## Rotinas LISP

A maioria das rotinas que apenas desenham, editam e consultam objetos funciona como está. Ainda não há suporte a funções ActiveX do Visual LISP (`vlax-…`, `vla-…`), diálogos DCL ou reactors ([gaps.md](gaps.md), E1). Uma rotina que dependa deles precisa ser adaptada — por exemplo, substituindo diálogos por chamadas a `command` e prompts `get…`.

Coloque as rotinas que deseja carregar em todas as sessões na pasta de suporte (`support` dentro da pasta de configurações; caminhos em [autolisp.md](autolisp.md#at-startup)) e carregue-as por `acad.lsp` ou `acaddoc.lsp`. Outra opção é usar `(autoload "file" '("CMD"))` para carregá-las na primeira vez que o comando for digitado.

## Scripts

Um arquivo `.scr` contém o que seria digitado na linha de comando, uma linha por vez. Execute-o com SCRIPT no aplicativo ou sem abrir a interface:

```sh
cadcraft-cli run drawing.dxf --script-file setup.scr --save out.dxf
```

## Aliases de comandos

ALIASEDIT lê um arquivo de aliases `.pgp` (linhas no formato `ALIAS, *COMMAND`) e adiciona os aliases ao restante da sessão. Linhas que iniciariam programas externos são ignoradas. Em um script ou no canal de controle, use `aliasedit {"file": "/caminho/para/aliases.pgp"}`.

## Controlando o CADCraft por outro programa

Todos os comandos têm uma forma JSON que pode ser executada sem diálogos. `cadcraft-cli commands [FILTER]` exibe o catálogo, incluindo o identificador e os parâmetros de cada comando.

- **Tarefas em lote sem interface:** `cadcraft-cli run FILE --cmd 'id {json}' --save OUT --export OUT.png`. Se o valor de um parâmetro `--cmd` for exatamente `{last}`, ele será substituído pelo último identificador de objeto retornado por um `--cmd` anterior na mesma execução. Assim, `viewport.set {"handle":"{last}","scale":1}` pode usar o resultado de `mview`. Todos os outros valores são enviados sem alteração.
- **Aplicativo em execução:** inicie-o com `--control PORT` e envie linhas JSON para `127.0.0.1:PORT`. Consulte [control-protocol.md](control-protocol.md).

  ```sh
  printf '%s\n' '{"id":1,"method":"cmdline.input","params":{"text":"circle 0,0 5"}}' | nc -w 2 127.0.0.1 7979
  ```

- **Agentes de IA:** use `cadcraft-cli mcp`, sem interface ou conectado ao aplicativo. Consulte [mcp.md](mcp.md).
