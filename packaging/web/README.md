# Hospedando o CADCraft na web

O arquivo `cadcraft-web-<versão>.zip` (obtido na página de versões do GitHub ou gerado por `packaging/web/package.sh`) contém um site estático na pasta `cadcraft-web-<versão>/`:

| Arquivo | Descrição |
|---|---|
| `index.html` | Página que carrega todos os recursos por URLs relativas. |
| `cadcraft-web-<hash>.js` | Código de integração gerado pelo wasm-bindgen (módulo ES). |
| `cadcraft-web-<hash>_bg.wasm` | Aplicativo: cerca de 13 MB, ou 5 MB com compactação. |
| `_headers`, `.htaccess` | Exemplos de regras de cabeçalho para Netlify/Cloudflare Pages e Apache. |

Não há código executado no servidor. Envie o conteúdo da pasta para qualquer serviço que hospede arquivos estáticos.

## Funciona em qualquer caminho

Todas as URLs em `index.html` são relativas (`public_url = "./"` em `apps/cadcraft-web/Trunk.toml`). Assim, o site funciona na raiz de um domínio (`https://example.com/`), em um caminho com prefixo (`https://example.com/tools/cadcraft/`) ou em um bucket de CDN. Os nomes dos recursos incluem um hash do conteúdo e podem ser armazenados em cache indefinidamente. Somente o `index.html` precisa ser revalidado.

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

Para testar localmente, execute `python3 -m http.server 8765` dentro da pasta e abra http://localhost:8765/.

## Incorporando o aplicativo em uma página (iframe)

```html
<iframe
  src="https://example.com/cadcraft/"
  title="Editor de imagens CADCraft"
  style="width: 100%; height: 720px; border: 0;"
  allow="fullscreen; clipboard-read; clipboard-write"
  allowfullscreen>
</iframe>
```

- O aplicativo preenche o iframe e acompanha seu tamanho; portanto, ajuste o tamanho do iframe, não o do aplicativo.
- Os atalhos de teclado são enviados ao iframe depois que o usuário clica nele, como acontece com qualquer aplicativo incorporado.
- **Incorporação entre origens diferentes:** funciona. As preferências ficam no `localStorage` do iframe. Navegadores que particionam ou bloqueiam armazenamento de terceiros podem esquecê-las entre visitas; nesse caso, o aplicativo inicia com as configurações padrão.
- **Iframes em sandbox:** precisam, no mínimo, de `sandbox="allow-scripts allow-same-origin allow-downloads allow-popups"`. Sem `allow-same-origin`, não há armazenamento. Sem `allow-downloads`, as ações Salvar e Exportar (downloads do navegador) são bloqueadas.
- Não envie `X-Frame-Options: DENY` nem uma política CSP `frame-ancestors` que exclua a página que incorpora o aplicativo.

## Seleção do renderizador e opções alternativas

O CADCraft renderiza usando wgpu. Quando disponível no navegador, usa **WebGPU**; caso contrário, alterna automaticamente para **WebGL2**. Parâmetros na URL podem substituir essa escolha e também funcionam no atributo `src` do iframe:

| Parâmetro | Efeito |
|---|---|
| *(nenhum)* | Usa WebGPU, se disponível; caso contrário, WebGL2. |
| `?webgl` | Força o mecanismo WebGL2 (útil quando um driver WebGPU apresenta problemas). |
| `?cpu` | Força o caminho de renderização em canvas pela CPU (mais lento, porém mais compatível). |

Exemplo: `<iframe src="https://example.com/cadcraft/?webgl" ...>`.

Se o navegador não oferecer WebGPU nem WebGL2, será exibida uma mensagem no lugar do aplicativo.
