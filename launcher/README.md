# Decode Open Source Hub

Central web estática e independente para descobrir ferramentas open source da Craft App Suite e o projeto Decode CAD.

## Testar localmente

Abra `launcher/index.html` em um navegador. Para testar a consulta à API do GitHub de forma mais confiável, sirva a pasta por HTTP local:

```bash
cd launcher
python3 -m http.server 8080
```

Acesse http://localhost:8080.

## Recursos

- Catálogo com 14 ferramentas, busca e filtros por categoria.
- Seletor de sistema operacional e priorização dos formatos de pacote publicados nas releases.
- Consulta de versão mais recente via API pública do GitHub.
- Links diretos para repositórios e páginas oficiais.
- Interface responsiva, sem backend, sem autenticação e sem dependência de Lovable.

## Limitações honestas

A central web não instala nem executa aplicações, não verifica programas instalados no computador e não faz atualizações silenciosas. Quando não há release ou pacote compatível, oferece a página oficial de releases. Os downloads vêm diretamente dos projetos originais.

## Marcas e licenças

Projeto independente da Decode Analytics, não afiliado oficialmente à equipe ArtCraft/storytold. As marcas e ícones pertencem aos respectivos projetos. Consulte cada repositório para os termos atuais de licença e atribuição.