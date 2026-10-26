# Decode Open Source Hub

Central visual independente, mantida pela Decode Analytics, para explorar ferramentas criativas open source.

## Estrutura e recursos

- `index.html`: site estático em HTML, CSS e JavaScript, sem build, backend, autenticação ou banco de dados.
- Catálogo com 13 aplicações da Craft App Suite e o Decode CAD da Decode Analytics.
- Busca, filtros por categoria, ordenação e interface responsiva.
- Seletor de sistema operacional e arquitetura do processador (automática, x86_64, ARM64 e Windows x86 de 32 bits).
- Links para as páginas oficiais, repositórios e downloads.
- Consulta à API pública do GitHub somente quando alguém aciona **Baixar** — não dispara uma rajada de requisições ao abrir a página.
- Se uma release não estiver disponível ou não houver pacote compatível, a central abre a página oficial do aplicativo; a escolha da arquitetura pode ser ajustada manualmente.

## Executar e validar localmente

Para servir a pasta localmente:

```bash
cd open-source-hub
python3 -m http.server 8080
```

Acesse http://localhost:8080. A verificação automatizada pode ser executada com Node.js 22 ou superior:

```bash
node open-source-hub/check.mjs
```

O mesmo teste é executado pelo GitHub Actions em pull requests que alteram esta pasta.

## Publicação na Vercel

A configuração do projeto Vercel utiliza a pasta `open-source-hub` como Root Directory. É um site estático, sem comando de build nem dependências de instalação.

## Limites importantes

O site não instala nem executa programas, não detecta instalações locais e não faz atualizações silenciosas. Os downloads vêm dos projetos originais. A identificação automática da arquitetura depende das informações fornecidas pelo navegador; selecione manualmente a arquitetura se necessário. A central rejeita pacotes identificados como pertencentes a outro sistema operacional e não escolhe arquivos com arquitetura conhecida incompatível.

A central é independente e não é afiliada oficialmente ao ArtCraft/storytold. As marcas e os ícones pertencem aos respetivos projetos; consulte cada repositório para confirmar licenças, estado de desenvolvimento e compatibilidade.
