# Hospedando o CADCraft na web

O arquivo `cadcraft-web-<versão>.zip` (obtido nas versões do GitHub ou por meio de `packaging/web/package.sh`) contém um site estático na pasta `cadcraft-web-<versão>/`:

| Arquivo | Descrição |
|---|---|
| `index.html` | Página que carrega todos os recursos por URLs relativas. |
| `cadcraft-web-<hash>.js` | Código de integração gerado pelo wasm-bindgen (módulo ES). |
| `cadcraft-web-<hash>_bg.wasm` | Aplicativo: cerca de 13 MB, ou 5 MB com compactação. |
| `_headers`, `.htaccess` | Exemplos de regras de cabeçalho para Netlify/Cloudflare Pages e Apache. |

Não há código executado no servidor. Envie o conteúdo da pasta para qualquer serviço que hospede arquivos estáticos.

## Funciona em qualquer caminho

Todas as URLs em `index.html` são relativas (`public_url = "./"` em `apps/cadcraft-web/Trunk.toml`). Por isso, o site funciona na raiz de um domínio (`https://exemplo.com/`), em um caminho com prefixo (`https://exemplo.com/ferramentas/cadcraft/`) ou em um bucket de CDN.

Os nomes dos recursos incluem um hash do conteúdo e podem ser armazenados em cache indefinidamente. Somente o `index.html` precisa ser revalidado.

## Configurações necessárias no servidor

- **Tipo MIME:** sirva arquivos `.wasm` como `application/wasm`. Os navegadores não conseguem compilá-los em fluxo com outro tipo, e o aplicativo pode carregar lentamente ou nem carregar. Sirva arquivos `.js` como `text/javascript`. A maioria dos provedores já faz isso. No nginx, confira se `mime.types` contém `application/wasm wasm;`.
- **Compactação:** habilite gzip ou Brotli para `.wasm`, `.js` e `.html`. Isso reduz o download de cerca de 13 MB para aproximadamente 5 MB. Também é possível pré-compactar os arquivos (por exemplo, `brotli -k *.wasm`) e configurar o servidor para enviar `Content-Encoding: br`.
- **Cache:** use `Cache-Control: public, max-age=31536000, immutable` nos arquivos `.wasm` e `.js` com hash, e `no-cache` no `index.html`.
- **HTTPS:** WebGPU e a área de transferência só funcionam em um contexto seguro: `https://` ou `http://localhost`. Em HTTP simples, fora do localhost, o aplicativo usa WebGL2 como alternativa.
- **Sem cabeçalhos especiais de isolamento:** o CADCraft não usa `SharedArrayBuffer`, portanto não precisa de `Cross-Origin-Opener-Policy` nem de `Cross-Origin-Embedder-Policy`. Se o site já enviar COEP `require-corp`, envie também `Cross-Origin-Resource-Policy: same-origin` (ou `cross-origin` quando os arquivos estiverem em uma CDN) nos arquivos do aplicativo.

Exemplo para nginx:

```nginx
location /cadcraft/ {
    types { application/wasm wasm; text/javascript js; text/html html; }
    gzip on;
    gzip_types application/wasm text/javascript text/html;
    location ~* \\.(wasm|js)$ { add_header Cache-Control "public, max-age=31536000, immutable"; }
    location ~* index\\.html$ { add_header Cache-Control "no-cache"; }
}
```

## Solução de problemas

- Se o aplicativo não iniciar, confirme que o arquivo `.wasm` é servido com o tipo MIME correto.
- Se o navegador continuar usando uma versão antiga após uma atualização, verifique a política de cache do `index.html`.
- Para testar localmente, use um servidor HTTP local em vez de abrir o arquivo `index.html` diretamente.
