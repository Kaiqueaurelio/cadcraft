# Idiomas da interface

A interface do CADCraft usa **inglês como idioma padrão**, para manter a terminologia e o fluxo de trabalho próximos aos do AutoCAD. O usuário pode selecionar outro idioma em **Edit → Interface Language**. As opções disponíveis são Automático (idioma do sistema), English, Українська, Português (Brasil), Deutsch, Español, Čeština, Русский, Français, 简体中文, Italiano, 한국어, Svenska, العربية, Magyar, Türkçe e Ελληνικά.

A troca de idioma ocorre sem reiniciar o aplicativo, inclusive nos menus nativos do macOS. A preferência é salva junto às demais configurações da interface (`cadcraft.prefs`, ao lado do tema): no armazenamento do eframe em aplicativos desktop e no armazenamento local do navegador na versão web. Se não houver uma preferência válida salva, o aplicativo usa o inglês.

## Seleção automática e configuração

A opção Automático escolhe o primeiro idioma compatível entre os idiomas preferidos do sistema operacional ou, na versão web, entre os valores de `navigator.languages`. Tags BCP 47 e POSIX são reduzidas ao idioma principal: por exemplo, `uk-UA` → `uk`, `de_AT.UTF-8` → `de`, `es-MX` → `es` e `pt_BR.UTF-8` → `pt-BR`. Idiomas sem suporte voltam para o inglês.

No desktop, a variável `CADCRAFT_LOCALE=cs-CZ` pode substituir o idioma do sistema para uma execução. Uma escolha explícita salva nas preferências tem prioridade sobre essa variável.

A automação pode alterar o idioma pelo canal de controle existente:

```json
{"id":1,"method":"engine.execute","params":{"command":"ui.language","params":{"lang":"cs"}}}
```

O método `ui.set` também aceita `{"interfaceLanguage":"cs"}`. Os códigos aceitos são `auto`, `en`, `uk`, `pt-BR`, `de`, `es`, `cs`, `ru`, `fr`, `zh`, `it`, `ko`, `sv`, `ar`, `hu`, `tr` e `el`. Valores inválidos são rejeitados sem alterar a seleção atual. Restaurar as paletas não redefine o idioma.

## Cobertura das traduções

Os catálogos abrangem nomes de comandos e menus, grupos de ferramentas e dicas, página inicial, abas de arquivos e espaço do modelo, controles da barra de status, modos de snap a objetos, instruções de desenho, rótulos de propriedades, diálogos e controles da janela Sobre. A referência de comandos pode ser pesquisada pelo identificador em inglês, pelo rótulo em inglês ou pela tradução.

O inglês continua sendo a fonte original dos rótulos e o idioma de fallback. Isso é intencional para evitar que uma tradução ausente impeça a compilação ou deixe a interface sem texto. A saída do mecanismo, respostas da CLI/MCP/canal de controle, diagnósticos do mecanismo e histórico da linha de comando permanecem em inglês.

Os catálogos são arquivos TSV em UTF-8, com três colunas separadas por tabulação: contexto, texto-fonte em inglês e tradução. A maioria das entradas tem contexto vazio. O contexto opcional `menu` permite distinguir um título de menu do rótulo de um comando ou objeto. As linhas `@plural` definem formas de plural para contagens inteiras.

### Português brasileiro

O catálogo `crates/ui-egui/src/i18n/pt_br.tsv` contém 1.231 linhas e cobre o catálogo de referência e os textos de interface marcados: comandos, menus, grupos e dicas de ferramentas, modos de snap a objetos, instruções de desenho, diálogos e Preferências. A verificação do catálogo não aponta lacunas nessas entradas. A regra de plural é `one|other`, seguindo CLDR para português: zero e um usam a forma `one`.

O catálogo original foi contribuído por @WladmirJunior; linhas adicionais foram concluídas por @RicSchonfelder com auxílio de IA. Uma revisão terminológica por falante nativo (@ffishman, 19/10/2026) corrigiu traduções e alinhou o vocabulário ao uso brasileiro de CAD. Alguns termos técnicos usados no Brasil permanecem como empréstimos, por exemplo *snap*, *viewport*, *grip* e *offset*. Ainda é útil realizar uma revisão final linha a linha e validar a disposição dos textos em cada plataforma.

### Outros catálogos

- **Tcheco:** 1.202 linhas; cobre as entradas de referência e os textos marcados. A tradução teve revisão de convenções de CAD, mas ainda não foi revisada por falante nativo independente.
- **Coreano:** 1.235 linhas, incluindo rótulos comuns, títulos de menu e modelos de contagem. A revisão nativa e a validação de layout/IME continuam pendentes.
- **Húngaro:** 1.326 linhas; cobre as entradas de referência e os textos marcados. Revisão independente pendente.
- **Turco:** 1.330 linhas; cobre as entradas de referência e os textos marcados. A interface em execução foi revisada por falante nativo, mas ainda falta revisão linha a linha.
- **Grego:** 1.330 chaves de origem reunidas dos catálogos; revisão nativa independente e validação visual ainda são necessárias.
- **Sueco:** catálogo concluído, com revisão terminológica por falante nativo.
- **Alemão:** catálogo parcial.
- **Ucraniano, espanhol, russo, francês, chinês simplificado e italiano:** catálogos disponíveis, com cobertura e revisão variáveis.

As traduções são trabalho original produzido a partir do significado dos textos em inglês. Não se devem copiar arquivos de localização proprietários de outros produtos.

## Texto de desenho e palavras-chave

Títulos de desenhos, nomes de camadas, layouts, blocos e estilos, nomes de parâmetros e expressões, texto contido no desenho e entrada da linha de comando são dados do usuário e mantêm a grafia original. Palavras-chave clicáveis nos prompts mantêm a forma em inglês que o mecanismo aceita. Isso evita divergências entre o texto exibido e o comando que precisa ser digitado.

As fontes incluídas cobrem os caracteres necessários aos catálogos ucraniano, alemão, tcheco e húngaro. As traduções não adicionam arquivos de fonte e funcionam na web sem depender das fontes do sistema. Para coreano, é necessária uma fonte que cubra todos os caracteres Hangul do catálogo. No desktop, o aplicativo verifica fontes instaladas e, opcionalmente, o diretório indicado por `CRAFT_FONTS_DIR`. Fontes CJK comuns podem já cobrir Hangul.

A versão web não consegue consultar fontes instaladas e não incorpora uma fonte coreana, para respeitar os limites de tamanho da hospedagem. Se não houver uma fonte adequada, a preferência `ko` é mantida, mas a interface usa inglês. Essa limitação não corrige problemas de IME específicos de plataforma nem traduz as mensagens do mecanismo.

## Como adicionar um idioma

A implementação usa um padrão de catálogo aplicado no momento de exibir o texto, seguindo a convenção TSV de outros projetos Craft, mas o código é específico do CADCraft.

1. Adicione um arquivo `crates/ui-egui/src/i18n/<código>.tsv` em UTF-8 com as colunas `context`, `English source` e `translation`.
2. Registre o catálogo em `i18n::LANGUAGES), adicione a variante de preferência e o código/nome do idioma e configure as regras e formas de plural.
3. Use `crate::tl!("English label")` para texto literal, `i18n::t(label)` para um rótulo dinâmico conhecido e `crate::tf!("Version {version}", version = value)` para texto com marcadores nomeados.
4. Traduza apenas no ponto em que o texto é exibido. Não traduza nomes arbitrários de usuários, identificadores ou dados de desenho.
5. Mantenha os identificadores de janelas e as chaves dos grupos de ferramentas baseados no inglês para preservar o estado quando o usuário troca de idioma.
6. Registre a atribuição da tradução em `ATTRIBUTION.md`, atualize a lista de idiomas e inclua testes de localidade, plural, glifos de fonte e troca de idioma.

Os catálogos são incorporados ao aplicativo e analisados uma vez. Entradas ausentes ou malformadas não devem encerrar o programa. Os testes rejeitam linhas inválidas, chaves duplicadas, marcadores alterados e reticências inconsistentes.

Comandos de validação:

```bash
cargo test -p cadcraft-ui-egui i18n
cargo xtask i18n
cargo xtask assets
cargo xtask ci
```

O primeiro comando verifica catálogos, resolução de localidade, preferências, plurais inteiros, glifos incorporados e renderização real do egui durante a troca de idioma. `cargo xtask i18n` lista textos que ainda aparecem em inglês em cada idioma; `cargo xtask assets` verifica a atribuição dos recursos; `cargo xtask ci` executa as verificações de qualidade do workspace.

As traduções são trabalho original sob a licença `MIT OR Apache-2.0`, elaborado a partir do significado dos textos em inglês do CADCraft. Não copie recursos de localização proprietários.
