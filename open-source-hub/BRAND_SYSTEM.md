# Decode Studio — identidade visual dos aplicativos

## Direção da marca
Todos os produtos adotam uma família visual própria da Decode Analytics: superfícies azul-marinho profundas, azul elétrico e ciano como acentos, geometria limpa, contraste alto e ícones de ferramenta com traço consistente. O nome público de cada produto começa com **Decode**.

## Nomes dos produtos
- Decode Photo — edição de imagem (referência upstream: PhotoCraft)
- Decode Light — revelação RAW e fotografia (LightCraft)
- Decode Film — vídeo e cor (FilmCraft)
- Decode Design — diagramação (DesignCraft)
- Decode Vector — ilustração vetorial (VectorCraft)
- Decode PDF — documentos PDF (PdfCraft)
- Decode Effects — efeitos visuais (EffectCraft)
- Decode Writer — texto e documentos (WordCraft)
- Decode Slides — apresentações (DeckCraft)
- Decode Sheets — planilhas (GridCraft)
- Decode Draft — CAD (CADCraft)
- Decode Audio — áudio (SoundCraft)
- Decode UI — interfaces e protótipos (FigCraft)
- Decode CAD — projeto próprio da Decode Analytics

## Tokens de design
- Deep navy: #07101E
- Navy surface: #0D1B34
- Decode blue: #4C85F7
- Decode sky: #7CA8FF
- Decode cyan: #55DDEB
- Text: #F4F8FF
- Secondary text: #9FB1CC
- Border: #263B5B

## Regras de aplicação
- Cada aplicativo recebe um símbolo de função distinto dentro do mesmo sistema gráfico.
- Títulos, menus, telas About e instaladores devem utilizar o nome Decode.
- Nunca substituir nem remover LICENSE, NOTICE, arquivos .attribution ou atribuições de terceiros.
- Repositório, pacote ou release upstream deve continuar identificado como fonte até a existência de builds independentes com ID de aplicativo Decode.
- Não usar ícones/nomes Craft como identidade visual primária nos produtos rebranded. A atribuição upstream deve ficar em créditos, documentação e avisos legais.
- Uma troca de ícone por si só não equivale a um fork rebranded: os próximos passos são atualizar metadados, identificadores de app, strings de interface, packaging, integração com deep links e testes, mantendo bibliotecas e dependências internas coerentes.

## Estado deste repositório
Os ícones vetoriais e a central visual já usam a identidade Decode. O código-fonte funcional dos 13 aplicativos upstream ainda precisa ser adaptado e compilado como builds downstream independentes; os nomes comerciais do launcher não significam que os binários upstream já foram rebatizados.
