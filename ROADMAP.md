# Roteiro de desenvolvimento do CADCraft

**Etapa atual: pré-alfa.** Próxima etapa: alfa. Faltam aproximadamente 125–190 horas de trabalho estimadas para cumprir os quatro fluxos centrais que ainda falham no [critério de entrada da alfa](docs/roadmap.md#alpha-gate). A prontidão estimada para uso real está em 38%; a meta da alfa é cerca de 40%.

> **Última revisão:** 10/10/2026 · **Última atualização:** 21/10/2026 · **Alteração mais recente:** objetos proxy preservam sua classe ao salvar (#1812). **Referência de compatibilidade:** Autodesk AutoCAD 2027.

O CADCraft busca alcançar paridade funcional com o AutoCAD, começando por desenho 2D e avançando para anotações, layouts, plotagem, DWG, recursos paramétricos e modelagem 3D. Também pretende oferecer recursos que não existem da mesma forma no AutoCAD: controle por agentes via MCP, CLI programável, versão web, versões para Linux e FreeBSD e licença gratuita.

## Por que ainda está em pré-alfa?

Quatro dos cinco fluxos essenciais avaliados ainda não passam no critério da alfa:

- **Desenho preciso:** ainda faltam recursos de rastreamento de snap a objetos e há opções incompletas em TRIM, EXTEND e OFFSET.
- **Blocos e referências externas:** não existe ainda um editor de blocos completo; REFEDIT permite edição no local, mas referências externas ainda não estão concluídas.
- **Plotagem:** os estilos de plotagem não são aplicados corretamente.
- **Salvamento DWG:** arquivos salvos podem perder elementos que o CADCraft ainda não representa; os arquivos resultantes ainda não foram validados no AutoCAD.
- **Cotagem:** passa na avaliação atual.

A prontidão estimada para trabalho real é de aproximadamente 38%, abaixo do limite de cerca de 40% para a alfa. A estimativa para chegar à beta é de 600–930 horas de trabalho de agentes, sujeita às incertezas do desenvolvimento.

## Indicadores principais

| Indicador | Estimativa atual | Como foi medido |
|---|---:|---|
| Amplitude de recursos (itens de menu do AutoCAD para Mac com comando funcional) | **50%** (245 de 491; menus 2D: cerca de 65%) | Lista de paridade e comandos somente de interface |
| Prontidão para uso real | **≈ 38%** | Estimativa baseada na avaliação de paridade |
| Profissional de desenho 2D | **≈ 28%** | Considera intercâmbio de arquivos, estabilidade e interação |
| Usuário de recursos essenciais | **≈ 46%** | Estimativa para fluxos básicos |
| Trabalho restante até a alfa | **≈ 125–190 h** | Itens do critério de entrada |
| Trabalho restante até a beta | **≈ 600–930 h** | Estimativa com execução paralela |
| Trabalho restante até a paridade completa | **≈ 1.300–2.100 h** | Estimativa global |

As horas são estimativas, não prazos garantidos. Consideram a calibração descrita em [paridade com o aplicativo-alvo](docs/target-app-parity.md#remaining-effort-and-how-it-was-calibrated), incluindo o custo maior de aprofundar comportamentos e corrigir falhas após a primeira implementação.

## Prontidão por perfil de uso

| Perfil | Prontidão estimada | Trabalho estimado até cerca de 95% | Principais gargalos |
|---|---:|---:|---|
| Escopo completo | ≈ 38% | ≈ 1.150–1.850 h de agentes | Núcleo 3D, formatos DWG/DXF, AutoLISP, blocos, referências externas e blocos dinâmicos |
| Profissional de desenho 2D | ≈ 28% | ≈ 570–880 h de agentes | Troca DWG sem perda, entrada precisa, geometria 2D, blocos, referências, anotações e estabilidade |
| Recursos essenciais | ≈ 46% | ≈ 140–225 h de agentes | Opções incompletas de desenho/edição, abertura de DWG, inicialização, diálogo de plotagem, menus de contexto e entrada dinâmica |

Os perfis são cumulativos: recursos essenciais fazem parte do perfil profissional, que por sua vez está incluído no escopo completo.

## Paridade por área

| Área | Paridade estimada | Trabalho restante |
|---|---:|---:|
| Recursos, ponderados pelo uso | 41% | 735–1.115 h |
| Fidelidade da interface e experiência de uso | 43% | 100–160 h |
| Formatos de arquivo | 30% | 170–280 h |
| Hardware | 45% | 15–25 h |
| Localização | 5% | 60–100 h |
| Desempenho | 60% | 20–40 h |
| Estabilidade | 35% | 30–50 h |
| Plataformas | 75% | 15–25 h |
| Ecossistema e automação | 10% | 120–200 h |
| Recursos de IA | 20% | 40–80 h |

Referências detalhadas: [lista de paridade](docs/parity-checklist.md), [lacunas conhecidas](docs/gaps.md), [geometria](docs/geometry-parity.md), [formatos de arquivo](docs/file-format-parity.md), [hardware](docs/hardware-parity.md), [interface](docs/ui-parity.md) e [localização](docs/localization-parity.md).

## Cobertura de idiomas

O idioma de origem da interface é o inglês. O CADCraft também inclui catálogos de tradução para vários idiomas. A existência de um catálogo não significa que todas as mensagens do mecanismo, da CLI ou do MCP estejam traduzidas; mensagens técnicas e textos sem entrada de catálogo podem permanecer em inglês.

| Idioma | Situação resumida |
|---|---|
| Inglês | Idioma de origem, cobertura completa |
| Português (Brasil) | Catálogo atual completo; revisão terminológica realizada, ainda sujeito a revisão adicional |
| Grego | Catálogo atual completo; revisão independente pendente |
| Sueco | Catálogo concluído com revisão terminológica |
| Coreano | Catálogo atual concluído; validação de fontes e revisão adicional pendentes |
| Húngaro e turco | Catálogos atuais concluídos; revisão independente ainda necessária |
| Alemão | Catálogo parcial |
| Ucraniano, espanhol, tcheco, russo, francês, chinês simplificado e italiano | Catálogos disponíveis, com níveis variados de cobertura |

Consulte [paridade de localização](docs/localization-parity.md) e [como funciona a localização](docs/localization.md) para detalhes sobre cobertura, fontes, limitações e como contribuir.

## Próximas prioridades

A ordem de trabalho atual é:

1. Manter o CI verde em pull requests e revisar as contribuições pendentes do projeto de origem.
2. Evitar perda de dados ao salvar; avançar em MULTILEADER nativo e em versões selecionáveis de DWG/DXF.
3. Criar um conjunto de arquivos DWG/DXF reais e validar o resultado no AutoCAD.
4. Concluir entrada precisa: rastreamento de snap a objetos, snaps de extensão/paralelo, FROM/M2P/TK, substituições de snap e entrada dinâmica editável.
5. Completar opções de prompt em TRIM, EXTEND, OFFSET, ROTATE, PLINE, SPLINE, MTEXT e MLEADER.
6. Implementar e validar salvamento automático, recuperação, AUDIT e RECOVER.
7. Melhorar a geometria exata de elipses e splines e finalizar a limpeza de loops de OFFSET em polilinhas.
8. Avançar em referências externas, editor de blocos, grupos e blocos dinâmicos.

A lista detalhada de marcos e os critérios de aceitação estão no [roteiro técnico](docs/roadmap.md).

## Registro de progresso

| Data | Progresso |
|---|---|
| 21/10/2026 | O salvamento DXF preserva a classe de objetos e entidades proxy, atualizando o índice de classe para corresponder à tabela CLASSES reescrita (#1812). |
| 14/10/2026 | HATCHEDIT passou a adicionar contornos a partir de pontos internos ou objetos fechados e remover loops selecionados; testes cobrem associação e desfazer/refazer. |
| 12/10/2026 | TRIM passou a manter apenas o trecho complementar após o último corte em uma polilinha fechada (#726). |
| 12/10/2026 | HATCHEDIT valida escala e ângulo explícitos antes de editar, evitando alterações parciais quando os valores são inválidos (#724). |
| 10/10/2026 | Integrações e correções incluíram catálogo de interface em ucraniano, caixas de entrada dinâmica com Tab e valores relativos, seleção por arrastar, preservação de WIPEOUT e gradientes de hachura em DXF/DWG, opções de DIM* via SETVAR, diálogo UNITS, prompt ao fechar janela e correções de cotagem. |
| 08/10/2026 | Foram publicadas versões com pacotes para Flatpak, AppImage, macOS universal assinado, Windows x64/x86/arm64, Linux, FreeBSD e web. |
| 07/10/2026 | Primeira versão construída em uma sessão, com cotagem, hachura, blocos, ponte DWG, layouts, plotagem PDF, tela GPU, índice espacial R-tree, restrições e pipeline de publicação. |

## Histórico de revisões

| Data | Tipo | Resumo |
|---|---|---|
| 21/10/2026 | Pequena | Objetos proxy preservam a classe ao salvar (#1812) |
| 14/10/2026 | Pequena | Adição e remoção de contornos no HATCHEDIT |
| 12/10/2026 | Pequena | Correção do TRIM em polilinha fechada (#726) |
| 12/10/2026 | Pequena | Validação de parâmetros numéricos do HATCHEDIT (#724) |
| 10/10/2026 | Média | Integração de catálogo ucraniano, entrada dinâmica e correções DXF; indicadores de interface e localização atualizados |
| 10/10/2026 | Média | Estimativas de prontidão e horas por perfil de usuário |
| 10/10/2026 | Grande | Reestruturação do documento, reavaliação de paridade e separação dos detalhes técnicos em documentos específicos |
| 07/10/2026 | Grande | Primeira lista de verificação alfa e estimativa de paridade ponderada |

## Integração do espaço de trabalho compartilhado

Conjuntos de ferramentas, Camadas e Propriedades usam encaixe compartilhado de painéis, layouts persistentes, foco de teclado e barras de ferramentas compactas com itens excedentes. Essa integração não altera os percentuais de marcos do roteiro.
