# Detalhamento do roadmap do CADCraft

> **Última revisão:** 2026-10-24 · **Última atualização:** 2026-10-24 · **Alteração:** pequena (Editor de Blocos para blocos estáticos: evidência de "Use blocks" atualizada) · **Referência:** Autodesk AutoCAD 2027

Plano futuro com marcos, status, foco atual e próximos passos estimados. O resumo está em [ROADMAP.md](../ROADMAP.md); os itens de trabalho que fundamentam cada linha estão em [gaps.md](gaps.md); os números vêm de [target-app-parity.md](target-app-parity.md). As horas são estimativas de trabalho de agente Opus 5.5.

## Foco atual

1. **Fazer a validação passar e integrar as alterações.** (`cargo fmt` na branch main já foi corrigido.) O fork agora tem um fluxo de pull request que executa a validação compartilhada `cargo xtask ci` por meio de `storytold/craft-common-actions`. Falta revisar os PRs upstream abertos e continuar confirmando que a validação funciona neste fork. ≈ 15–25 h.
2. **Parar de perder dados ao salvar** (FF1, FF2, FF7) e permitir salvar DWG/DXF em uma versão escolhida (FF3). ≈ 30–45 h.
3. **Criar um corpus de arquivos reais** e verificar a saída no AutoCAD (FF5; depende do responsável pelo projeto).
4. **Entrada de precisão:** rastreamento de snaps, snaps de extensão/paralelo, FROM/M2P/TK, substituições de snap digitadas e entrada dinâmica editável (U1–U4). ≈ 25–35 h.
5. **Implementar as opções de prompt ainda incompletas** nos comandos mais usados (F1). ≈ 15–25 h.
6. **Salvamento automático, recuperação, AUDIT e RECOVER** (F4). ≈ 10–15 h.
7. **Concluir o restante dos requisitos para alpha:** editor de blocos e REFEDIT, anexação de xrefs (F2, F3) e estilos de plotagem com diálogo e pré-visualização (F14). ≈ 50–80 h.

Os itens 2–5 e 7 fazem parte dos requisitos para [alpha](#requisitos-para-alpha).

## Requisitos para alpha

Fluxos essenciais realizados diariamente por um profissional típico do AutoCAD (padrão craftrules progress-docs). O CADCraft está em **pré-alpha**, pois quatro dos cinco fluxos ainda não passam pelos requisitos; os itens pendentes formam a lista de verificação da alpha. Plataforma principal: macOS.

| Fluxo de trabalho essencial | Funciona de ponta a ponta? | Evidências | Horas para aprovação |
|---|---|---|---|
| Desenhar e modificar com precisão usando snaps e rastreamento | **parcial, bloqueador** | Snaps em execução, polar, ortho e entrada de coordenadas funcionam; rastreamento de snap, snaps de extensão e paralelo são opções sem comportamento implementado; não há FROM/M2P/TK nem substituições de snap digitadas; os modos padrão de TRIM/EXTEND, Erase/Layer/Multiple de OFFSET e Reference de ROTATE exibem "not available yet"; interseções de elipses e splines são aproximadas ([ui-parity.md](ui-parity.md), [gaps.md](gaps.md), U1, U3, U4, F1) | 35–50 |
| Cotar e anotar | parcial, não bloqueador | Todos os tipos principais de DIM, associatividade, variáveis DIMSTYLE (43 de cerca de 80), TEXT/MTEXT, MLEADER e tabelas funcionam; faltam a caixa de diálogo DIMSTYLE, o editor MTEXT no local e recursos avançados de MLEADER (F7, F9, F10). É possível cotar uma folha de ponta a ponta | 0 para alpha (trabalho de beta) |
| Usar blocos (inserir blocos da biblioteca, editá-los, atributos e xrefs) | **parcial, bloqueador** | BLOCK/INSERT estáticos, atributos, edição por ATTSYNC e BATTMAN, xrefs (#525) e o Editor de Blocos (BEDIT/BSAVE/BCLOSE: todas as referências são atualizadas) funcionam; falta REFEDIT, portanto não é possível editar um bloco diretamente no desenho; blocos dinâmicos vindos de outros desenhos perdem seu comportamento (F2, F3) | 35–55 |
| Camadas, layouts e plotagem de uma folha | **parcial, bloqueador** | Camadas, layouts, viewports, configurações de página, saída PDF, estilos de plotagem (CTB/STB, #410), diálogo Plot (#160), Plot Preview, PUBLISH (#186) e dispositivos plotter (#564) funcionam; impressoras são atendidas somente por arquivos PDF/PNG/SVG (F14, H3) | 15–25 |
| Salvar um DWG que o AutoCAD abra sem perdas e reabrir DWGs de terceiros | **não** | O DXF preserva dados que o CADCraft não modela (#528) e grava MULTILEADER nativo (#459, ainda não testado em fluxo DWG); pelo caminho DWG, acadrust descarta entidades e objetos de classes personalizadas e alguns dados externos; nunca houve verificação no AutoCAD nem testes com arquivos reais; um DWG de 10 MB falhou após cerca de 2 minutos (#57) (FF1–FF6) | 40–60 + responsável (corpus de arquivos reais e verificação no AutoCAD) |
| **Total para alpha** | | | **≈ 125–190** |

A prontidão para uso real é de aproximadamente 38%, também abaixo do limite de cerca de 40% para alpha; concluir os itens bloqueadores eleva o resultado acima de 40%.

## Lista de verificação para beta

Beta significa estar perto da completude de recursos para trabalho 2D convencional, abrir e salvar DWG de forma confiável e manter apenas bugs conhecidos e recursos de borda pendentes (≈ 75% de prontidão para uso real).

| Item | Estado | Estimativa |
|---|---|---|
| DWG/DXF: nenhuma perda ao salvar; MULTILEADER nativo; escolha de versão; páginas de código; corpus de arquivos reais aprovado; verificado no AutoCAD | não iniciado | 50–75 |
| Geometria 2D exata para elipses e splines; offset de polilinha correto; tolerâncias relativas | não iniciado | 50–80 |
| Entrada de precisão no nível do AutoCAD (rastreamento, entrada dinâmica, substituições, modificadores de ponto, grips, palavras-chave de seleção e menus de contexto) | parcial | 90–140 |
| Todas as opções de prompt de Draw/Modify funcionam; matrizes associativas | parcial | 40–60 |
| Profundidade de anotação: gerenciador e variáveis DIMSTYLE, comandos DIM ausentes, MTEXT no local, campos, edição de MLEADER, escala anotativa e tabelas | parcial | 90–130 |
| Xrefs, editor de blocos, blocos dinâmicos, grupos e ferramentas de atributos | não iniciado | 90–135 |
| Estilos de plotagem, pré-visualização, PUBLISH e viewports não retangulares | parcial | 30–45 |
| Salvamento automático, recuperação, AUDIT/RECOVER, diálogo Options, modelos, SHX e arquivos `.lin`/`.pat` do usuário | parcial | 40–60 |
| Subconjunto principal de AutoLISP e SCRIPT | concluído (AutoLISP #758, SCRIPT #693); Visual LISP, DCL e reactors ainda virão | 30–50 |
| Catálogo de idiomas e idiomas do AutoCAD para Mac | não iniciado | 30–45 |
| Estabilidade: validação de CI, backlog de problemas, verificação rápida de comandos e fuzzing geométrico | parcial | 30–50 |
| Plataformas: abrir/salvar na web, verificações de execução em Windows/Linux; desempenho com 1 milhão de entidades e DWGs grandes | parcial | 20–35 |
| **Total até beta** | | **≈ 600–930** |

Cerca de 70% desse trabalho pode ser executado em paralelo por 4–6 agentes (um por linha), portanto o tempo decorrido estimado é de aproximadamente 150–250 horas.

## Marcos

Status reavaliado em 2026-10-10 com base na profundidade, e não apenas na presença do recurso (os percentuais de 2026-10-07, informados pelos agentes que implementaram cada marco, estão no registro de progresso de [ROADMAP.md](../ROADMAP.md)).

| # | Marco | Status | Restante (h) |
|---|---|---|---|
| M0 | Estrutura básica e fatia vertical: workspace, interface no estilo AutoCAD, linha de comando, desenho/edição básicos, DXF, CLI, MCP e web | concluído | — |
| M1 | Núcleo de desenho: todos os comandos e opções 2D de Draw, entrada dinâmica, rastreamento de snap e substituições de snap | ≈ 55% (boa cobertura dos comandos Draw; faltam rastreamento, entrada dinâmica e substituições) | 35–55 |
| M2 | Modificação: grips, PEDIT, SPLINEDIT, LENGTHEN, BLEND, ALIGN, matrizes associativas, trim/extend para todas as curvas e interface MATCHPROP | ≈ 50% | 45–70 |
| M3 | Camadas e propriedades | ≈ 70% (bugs de camadas em aberto #71–#75, #87) | 10–15 |
| M4 | Anotações: DIM*, gerenciador DIMSTYLE, MLEADER, MTEXT no local, campos, tabelas, escala anotativa e SHX | ≈ 40% | 90–130 |
| M5 | Hachuras e blocos: recursos avançados de hachura, editor de blocos, blocos dinâmicos, xrefs, grupos e atributos | ≈ 30% | 105–160 |
| M6 | Layouts e plotagem: viewports, configurações de página, estilos de plotagem, pré-visualização e PUBLISH | ≈ 35% | 35–50 |
| M7 | Arquivos: fidelidade e versões DWG/DXF, RECOVER/AUDIT, modelos, salvamento automático, ETRANSMIT e referências externas | ≈ 30% | 170–280 (inclui outros formatos) |
| M8 | Consultas e utilitários, OPTIONS e personalização | ≈ 25% | 45–70 |
| M9 | Paramétrico | ≈ 50% | 20–30 |
| M10 | Desempenho: 1 milhão de entidades a 60 fps e carregamento de DWGs grandes | ≈ 60% (somente números internos) | 20–40 |
| M11 | 3D: UCS, órbita, estilos visuais, sólidos, superfícies, malhas, seções e renderização | ≈ 2% | 300–450 |
| M12 | Automação: AutoLISP, SCRIPT, gravador de ações, conjuntos de folhas e Express Tools | ≈ 15% (MCP, CLI e canal de controle concluídos) | 130–220 |
| M13 | Acabamento da versão 1.0: preferências, espaços de trabalho, localização, acessibilidade, pacotes assinados e documentação | ≈ 15% (pacotes assinados para todas as plataformas existem; temas concluídos; catálogo de strings com ucraniano e alemão e troca de idioma em tempo real, #36) | 110–170 |
| — | Precisão geométrica (transversal; ver [geometry-parity.md](geometry-parity.md)) | ≈ 40% | 70–110 |
| — | Recursos de IA | ≈ 0% no aplicativo | 40–80 |
| | **Restante para paridade completa** | | **≈ 1.300–2.100** |

Os marcos se sobrepõem um pouco às linhas transversais; o total corresponde à soma das dimensões em [target-app-parity.md](target-app-parity.md), e não à soma da coluna.

## Depois da beta

Fundamentos 3D (UCS, vistas 3D e órbita, depois sólidos conforme `plan/adr/0001`), conjuntos de folhas, suporte completo a AutoLISP, recursos de IA com modelos locais de licença aberta e todos os idiomas restantes.

## Histórico de revisões

| Data | Alteração | Resumo |
|---|---|---|
| 2026-10-12 | pequena | Editor de Blocos para blocos estáticos integrado: evidência de "Use blocks" atualizada; status e estimativas inalterados (REFEDIT e blocos dinâmicos continuam pendentes) |
| 2026-10-11 | pequena | Evidências dos requisitos para alpha atualizadas após integração de trabalho (#459, #525, #528, #160, #186, #564); status e estimativas inalterados |
| 2026-10-10 | pequena | Main integrada: formatação corrigida, catálogo ucraniano registrado em M13, prontidão para uso real em 38% |
| 2026-10-10 | pequena | Requisitos para alpha adicionados (5 fluxos essenciais, 4 pendentes); estágio definido como pré-alpha |
| 2026-10-10 | grande | Criado a partir da tabela de marcos do ROADMAP antigo e do foco atual; marcos reavaliados pela profundidade; lista de verificação da beta adicionada |
