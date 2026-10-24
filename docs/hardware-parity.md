# Compatibilidade de hardware

> **Última revisão:** 2026-10-10 · **Última atualização:** 2026-10-13 · **Alteração:** pequena (zoom e pan passam a ser regenerados em segundo plano) · **Referência:** Autodesk AutoCAD 2027 (Mac e Windows)

O hardware usado pelo AutoCAD e como o CADCraft se compara em cada plataforma. Itens de trabalho:
[gaps.md](gaps.md#hardware).

**Resumo (estimativa): compatibilidade de hardware ≈ 45%.** Desenho 2D por GPU, HiDPI e trackpads são compatíveis em todas as plataformas. Ainda faltam: mouses 3D, tablets/digitalizadores, aceleração por GPU para 3D (o CADCraft ainda não tem recursos 3D) e suporte deliberado a vários monitores. Trabalho restante: **≈ 15–25 horas de agente Opus 5.5**, além de uma pessoa com os dispositivos para testar mouses 3D e tablets.

Medição feita a partir do código em 2026-10-10 (`crates/ui-egui/src/gpu.rs`, `canvas.rs`, `apps/cadcraft/src/graphics.rs`, `apps/cadcraft-web`); os dados do AutoCAD vêm dos requisitos de sistema e da documentação de desempenho gráfico da Autodesk. Não foram realizados benchmarks nesta etapa.

| Recurso | AutoCAD 2027 | CADCraft macOS | CADCraft Windows | CADCraft Linux / FreeBSD | CADCraft web |
|---|---|---|---|---|---|
| Canvas 2D com GPU | sim (aceleração por hardware, suavização por GPU, geometria de alta qualidade) | wgpu no Metal | wgpu no DirectX 12 por padrão (`WGPU_BACKEND` substitui; GL é evitado após falhas em GPUs AMD, #31) | wgpu no Vulkan (ou GL) | WebGPU, com fallback para WebGL2 |
| Fallback para CPU | modo de software | sim (rasterizador `tiny-skia`) | sim | sim | — |
| GPU para 3D (estilos visuais sombreados, renderização com ray tracing) | sim | não (sem 3D) | não | não | não |
| Apresentação de baixa latência | sim | vsync ou baixa latência via `CADCRAFT_VSYNC` | igual | sem fila de vsync, para manter o cursor em cruz responsivo (#183) | controlada pelo navegador |
| HiDPI / Retina, escala da tela | sim | sim (egui: pontos por pixel) | sim; há uma correção de DPI para Windows 11 aguardando integração upstream (PR #110) | sim | sim |
| Vários monitores (paletas em outras telas, DPI por monitor) | sim | não tratado de forma específica; uma janela | igual | igual | — |
| Pan no trackpad e zoom por pinça | sim (Mac) | sim, com detecção de trackpad versus roda do mouse (#58) | apenas roda do mouse (touchpad de precisão não testado) | roda do mouse; atraso relatado no Linux Mint (#25) | eventos do navegador |
| Magic Mouse | sim | tratado como trackpad | — | — | — |
| Roda do mouse e pan com botão do meio | sim | sim | sim | sim | sim |
| Mouse 3D (3Dconnexion SpaceMouse) | sim | **não** | **não** | **não** | não |
| Mesa digitalizadora / digitalizador (comando TABLET e calibração) | sim (Windows) | **não** | **não** | **não** | não |
| Lupa de tela que acompanha o cursor | sim | — | apresenta falha (#39; PR #187 adiciona uma opção para usar o cursor do sistema) | — | — |
| Plotters e impressoras (dispositivos PC3, impressoras do sistema) | sim | **não**: somente arquivos PDF/PNG/SVG; Print pergunta onde salvar o PDF (#26) | igual | igual | os bytes do PDF são exibidos em vez de iniciar um download (#134) |

## Desempenho no hardware (números internos)

Dados do roadmap anterior (medidos pelos agentes que implementaram os recursos, em 2026-10-06, usando `cadcraft-cli perf`):
200.000 entidades são selecionadas em 0,001 ms, levam cerca de 4 ms após uma edição (R-tree incremental), e o canvas de GPU desenha um quadro em cerca de 4 ms. Nada disso foi comparado ao AutoCAD na mesma máquina, e desenhos com 1 milhão de entidades ainda não foram testados. Um DWG de 10 MB levou cerca de dois minutos até falhar (#57).

Zoom e pan já não congelam a janela (2026-10-13). Em um DWG arquitetônico hachurado com 12 mil entidades (cerca de 340 mil primitivas de exibição), cada etapa do zoom bloqueava a interface por 2–4,5 s enquanto a lista de exibição, os dados de vértices e os buffers da GPU eram reconstruídos. Agora essas reconstruções são executadas em uma thread de trabalho enquanto a lista anterior continua sendo desenhada; a maior pausa caiu para 26–64 ms (Mac com chip M-series). As edições ainda fazem a reconstrução na thread da interface (cerca de 1,8 s nesse desenho): o próximo passo é implementar cache por entidade.

## Histórico de revisões

| Data | Alteração | Resumo |
|---|---|---|
| 2026-10-13 | pequena | Zoom e pan regenerados em segundo plano: pausas de 2–4,5 s reduzidas para 26–64 ms em um DWG hachurado |
| 2026-10-10 | grande | Primeiro documento de compatibilidade de hardware: backends de GPU por plataforma, dispositivos de entrada, telas e plotters |
