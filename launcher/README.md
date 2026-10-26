# Decode Open Source Hub

Central web estática e independente para descobrir ferramentas open source da Craft App Suite e o projeto Decode CAD.

## Executar localmente

Abra `launcher/index.html` no navegador. Para testar a consulta às releases via HTTP, sirva a pasta localmente:

```bash
cd launcher
python3 -m http.server 8080
```

Acesse http://localhost:8080.

## Recursos

- Catálogo com 14 projetos, busca e filtros por categoria.
- Seletor de sistema operacional e arquitetura (x86_64/Intel-AMD ou ARM64).
- Consulta à API pública do GitHub somente quando a pessoa clica em **Baixar**, evitando várias chamadas ao carregar a página.
- Prioridade para pacotes compatíveis com sistema e arquitetura; formatos Linux incluem DEB, AppImage, Flatpak, tarball e RPM.
- Links para os repositórios, sites e releases oficiais.
- Ícones apontando para os arquivos de cada repositório.
- Interface responsiva, sem backend, sem autenticação, sem etapa de build e sem dependência de Lovable.
- Verificação estática/funcional básica executável com `node launcher/check.mjs`.

## Limitações honestas

Esta central web não instala nem executa aplicações, não verifica programas instalados e não faz atualizações silenciosas. Os pacotes são baixados diretamente dos projetos originais. Quando uma release não existe, a API está indisponível ou não há pacote compatível, a central abre a página oficial do aplicativo ou as releases do projeto. Se a arquitetura for detectada incorretamente pelo navegador, selecione-a manualmente.

## Marcas e licenças

Projeto independente da Decode Analytics, não afiliado oficialmente à equipe ArtCraft/storytold. As marcas e os ícones pertencem aos respectivos projetos. Consulte cada repositório para os termos atuais de licença e atribuição.
