# Decode Open Source Hub

Central visual independente, mantida pela Decode Analytics, para explorar ferramentas criativas open source.

## Estrutura

- `index.html`: site estático em HTML, CSS e JavaScript, sem build, backend, autenticação, banco de dados ou serviços pagos.
- Catálogo inicial: 13 aplicações da Craft App Suite e o Decode CAD da Decode Analytics, cada uma com links para o respectivo repositório e página de releases oficiais.
- O site consulta as releases públicas do GitHub para exibir versões e escolher pacotes conforme o sistema operacional selecionado. Se a API estiver indisponível, os links oficiais continuam disponíveis.

## Publicação na Vercel

Configure o projeto para usar a pasta `open-source-hub` como Root Directory. Como é um site estático, não é necessário comando de build nem dependências de instalação.

## Limites importantes

O site não instala programas nem detecta instalações locais; downloads e instalações são feitos pelo navegador e pelo sistema operacional. A central é independente e não é afiliada oficialmente ao ArtCraft/storytold. Consulte cada repositório para confirmar licença, estado de desenvolvimento e compatibilidade.
