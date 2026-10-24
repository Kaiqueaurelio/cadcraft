# AutoLISP

O CADCraft executa rotinas AutoLISP (arquivos `.lsp`). O que já está disponível e o que ainda falta:
[gaps.md](gaps.md) (E1).

## Carregar e executar

- **APPLOAD** (Tools > Load Application…, alias `AP`) abre um seletor de arquivos filtrado para arquivos `.lsp`, começando na pasta usada da última vez (armazenada nas preferências), e executa o arquivo escolhido. Cancelar o seletor não faz nada. Um arquivo selecionado é carregado como se seu nome tivesse sido digitado; assim, se a rotina pedir um ponto, ela aguardará a entrada. Em uma versão desktop sem seletor de arquivos, o nome é solicitado na linha de comando. A versão web não tem sistema de arquivos, então pede para arrastar o arquivo `.lsp` para a janela e o carrega a partir do conteúdo. Somente arquivos `.lsp` são aceitos.
- **`(load "name")`** carrega um arquivo pelo caminho, acrescentando `.lsp` quando a extensão não estiver presente. Um arquivo carregado por APPLOAD também pode ser localizado novamente pelo nome, inclusive na versão web, que não tem sistema de arquivos.
- **`(defun c:NAME …)`** define um comando: digite `NAME` na linha de comando. Os comandos integrados mantêm seus nomes.
- **Na linha de comando**, `(expression)` avalia a expressão e mostra o valor, enquanto `!variable` mostra o valor de uma variável. Quando digitada no prompt de um comando (e não em um campo de texto livre), o valor de uma expressão pode servir como resposta: `CIRCLE 0,0 (* 2 r)`.
- **Por programação** (scripts, canal de controle, MCP e CLI): `lisp {code, inputs?}` e `appload {path} | {name, text | data}` executam até o fim e retornam `{value, output, error?}`. `appload` também retorna `commands`, os comandos `c:` definidos pelo arquivo; uma chamada JSON nunca abre uma caixa de diálogo de arquivos. `inputs` responde às chamadas `get…` na ordem (números, strings, pontos `[x, y]`); quando não há mais entradas, a chamada recebe nil.

### Na inicialização

O CADCraft carrega arquivos LISP da **pasta de suporte**, `support`, dentro da pasta de configurações (no Windows, `%APPDATA%\CADCraft\support`; no macOS, `~/Library/Application Support/CADCraft/support`; no Linux, `~/.config/cadcraft/support`; criada na primeira inicialização):

- `acad.lsp` uma vez por sessão, no primeiro desenho; com `ACADLSPASDOC` definido como 1, em todos os desenhos;
- `acaddoc.lsp` em cada desenho aberto ou criado;
- depois, `S::STARTUP`, caso esteja definida.

`(autoload "file" '("CMD1" "CMD2"))` em um arquivo de inicialização transforma `CMD1` e `CMD2` em comandos que carregam `file` (também a partir da pasta de suporte) na primeira vez em que são digitados. Os arquivos de inicialização são executados até o fim sem solicitar entradas. A pasta do próprio desenho nunca é pesquisada: um arquivo `acaddoc.lsp` que venha junto de um desenho baixado não é executado.

Cada desenho tem suas próprias variáveis e funções. Uma rotina corresponde a uma única etapa de desfazer, independentemente dos comandos que executar. Uma rotina que aguarda entrada (`getpoint`, `pause`) permite que a janela continue funcionando enquanto espera; Esc a interrompe com a mensagem "Function cancelled", recebida por `*error*`. Loops longos executam uma parte por quadro, mantendo a janela responsiva; Esc também os interrompe.

## A linguagem

Inteiros têm 32 bits e sofrem overflow circular como no AutoLISP; a presença de qualquer número real faz a aritmética usar valores reais. As variáveis têm escopo dinâmico: os argumentos e as variáveis locais de uma função (declaradas após `/`) ficam vinculados durante sua execução. Símbolos não diferenciam maiúsculas de minúsculas. Números reais são exibidos com seis algarismos significativos.

Nenhuma rotina pode fazer o programa travar: recursão profunda gera o erro "internal stack limit reached", código-fonte malformado gera um erro de leitura e todas as funções integradas validam seus argumentos.

## Funções integradas

- **Formas:** `quote` `'` `function` `setq` `set` `if` `cond` `while` `repeat` `foreach` `progn` `and` `or` `defun` `defun-q` `lambda` `eval` `apply` `mapcar` `load` `autoload` `exit` `quit`
- **Números:** `+ - * / rem 1+ 1- abs min max sqrt exp log sin cos atan expt fix float gcd logand logior lsh ~ = /= < > <= >= zerop minusp numberp`
- **Listas:** `car cdr` e combinações de `c…r` de até quatro níveis, `cons list append reverse length nth last member assoc subst acad_strlsort`, `vl-position vl-remove vl-sort vl-remove-if vl-remove-if-not vl-member-if vl-every vl-some vl-list-length vl-consp`
- **Predicados:** `atom listp null not type eq equal boundp vl-symbolp`
- **Strings:** `strcat strlen substr strcase itoa atoi atof chr ascii read wcmatch`, `vl-string-search vl-string-subst vl-string-trim vl-string-left-trim vl-string-right-trim vl-string-translate vl-string->list vl-list->string vl-prin1-to-string vl-princ-to-string vl-symbol-name vl-symbol-value vl-filename-base vl-filename-extension vl-filename-directory`
- **Geometria:** `polar distance angle inters trans` (`trans` retorna o ponto; UCS ainda não é compatível)
- **Saída e arquivos:** `princ prin1 print prompt terpri alert open close read-line write-line findfile`
- **Erros:** `*error*`, `vl-catch-all-apply vl-catch-all-error-p vl-catch-all-error-message`
- **Desenho:** `command` (também `command-s`, `vl-cmdf`) com `pause`, um comando que permanece aberto para a próxima chamada e `(command)` para cancelá-lo; `getvar setvar`; `entget entmake entmod entdel entlast entnext entupd handent`; `ssget` (modos X, L, P, I, W, C, F, WP, CP, um ponto ou seleção pelo usuário; filtros com curingas e operadores `-4`) `ssadd ssdel sslength ssname ssmemb sssetfirst ssgetfirst`; `tblsearch tblnext` (LAYER, LTYPE, STYLE, BLOCK, DIMSTYLE, APPID)
- **Entrada:** `initget getpoint getcorner getdist getangle getorient getreal getint getstring getkword entsel nentsel`
- **Unidades:** `rtos angtos distof angtof`
- **Sem operação:** `vl-load-com gc redraw`; `ver`

As listas de entidades usam os códigos de grupo DXF dos arquivos DXF do CADCraft: o que `entget` retorna corresponde ao conteúdo do objeto no arquivo DXF, acrescido de `-1` (seu nome) e `410` (seu layout). Os ângulos (grupos 50–58) são expressos em radianos, como no AutoCAD, embora o arquivo armazene graus; `entmake`, `entmod` e os filtros de `ssget` também usam radianos.

`entmake` define blocos: uma lista BLOCK (2 = nome, 70 = flags, 10 = ponto-base, 4 = descrição) inicia uma definição; as entidades criadas depois dela são inseridas no bloco, em vez de no desenho. Uma lista ENDBLK finaliza e adiciona o bloco (redefinindo um bloco de mesmo nome, se houver) e retorna seu nome. A flag 1 com um nome `*U` cria um bloco anônimo, que recebe um nome ao ser finalizado. Uma lista BLOCK quando já existe uma definição aberta, uma xref (flag 4), um nome inválido, um bloco que insere a si próprio e um bloco editado no local (REFEDIT) retornam nil. A definição permanece aberta entre rotinas até ENDBLK, mas uma rotina que falha descarta a definição que iniciou.

## Testes com o corpus

O corpus de conformidade do AutoLISP (#648) fica em seu próprio repositório. Defina `CADCRAFT_LISP_CORPUS` apontando para ele e execute `cargo test -p cadcraft-engine --test it lisp_corpus::`; sem essa variável, o teste informa que foi ignorado.
