# Decode Open Source Launcher

Aplicativo desktop nativo da Decode Analytics para explorar e gerenciar ferramentas open source com a identidade visual Decode Suite.

## Engenharia reversa de referência

A implementação foi planejada após análise do código-fonte público de https://github.com/storytold/craft-launcher e de suas camadas principais:

- crates/core/src/catalog.rs: catálogo fixo dos treze programas, nomes, repositórios, ícones e identificadores.
- crates/core/src/release.rs e crates/core/src/platform.rs: leitura de releases, validação de URL, identificação de formatos e escolha do artefato pela plataforma/arquitetura.
- crates/native/src/http.rs: cliente HTTPS, limites de tamanho, downloads e verificação SHA-256.
- crates/native/src/install.rs, detect.rs, manifest.rs e paths.rs: instalação/atualização, detecção local, registros de propriedade e diretórios gerenciados.
- crates/ui-egui e apps/artcraft-launcher: interface egui/eframe e aplicativo desktop.

Este repositório contém uma implementação independente para a Decode Analytics; não copia nem pretende ser o produto oficial da ArtCraft/storytold. A suíte de referência é consultada por APIs e releases públicas. O Decode CAD aparece como projeto independente da Decode Analytics.

## Estado da implementação

- [x] Interface desktop em Rust + egui/eframe, busca, categorias e estado por aplicativo.
- [x] Catálogo com 13 nomes de produto Decode e o Decode CAD, usando uma família de ícones própria Decode.
- [x] Consulta de releases públicas do GitHub, cache de respostas e links de fallback para os repositórios upstream.
- [x] Seleção de pacotes por sistema/arquitetura e formato, com distinção entre instaladores e pacotes portáteis.
- [x] Verificação SHA-256 por digest da API ou SHA256SUMS.txt, com recusa de pacotes sem checksum.
- [x] Instalação local de AppImage/TAR.GZ no Linux e pacote ZIP portátil no Windows, com extração limitada e rejeição de caminhos inseguros/links simbólicos.
- [x] Abre o MSI oficial no assistente do Windows quando esse é o artefato preferido.
- [x] Instala DMG do macOS em ~/Applications com hdiutil/ditto.
- [x] Registro local de instalações gerenciadas e opção de remover somente arquivos que pertencem à central.
- [x] Detecção básica de aplicativos externos em caminhos comuns e Flatpak.
- [x] Workflow de compilação/testes para Linux, Windows e macOS.

### Limitações conhecidas desta versão inicial

Esta ainda não é paridade completa com o launcher de referência nem uma migração completa dos 13 aplicativos. A identidade Decode atualmente cobre o catálogo/launcher e os ícones do hub; cada aplicativo original ainda precisa ser bifurcado, retematizado, recompilado e validado antes de poder ser distribuído como versão Decode. A bandeja do sistema, notificações em segundo plano, single-instance, pausa/cancelamento de transferências, atualização automática do próprio launcher, detecção exaustiva de instalações de todos os gestores de pacotes e integração completa com MSI/DEB/RPM/Flatpak/Arch ainda precisam ser concluídas. A detecção de aplicativos instalados fora da central é intencionalmente conservadora; ela não deve prometer uma versão se não conseguir lê-la.

No Windows, o MSI abre o instalador oficial com interface própria e a instalação passa a ser gerida pelo Windows Installer; esta central atualiza o status, mas não assume propriedade sobre esses arquivos. Não há execução silenciosa nem elevação invisível de privilégios.

## Compilar no Linux Mint / Ubuntu

Dependências para compilar o renderer desktop:

    sudo apt update
    sudo apt install -y build-essential pkg-config libxkbcommon-dev libwayland-dev \
      libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libasound2-dev

Instale uma versão estável recente do Rust e execute:

    cd launcher-desktop
    cargo run --release

## Segurança e dados locais

A central usa somente releases oficiais de cada repositório. Os downloads são gravados sob a pasta de cache do usuário e conferidos antes da instalação. Pacotes que não tenham checksum verificável são recusados. O launcher não executa os aplicativos baixados durante a etapa de download/verificação.

Os registros ficam em Decode Open Source Hub/installed.json dentro da pasta local de dados do usuário. A remoção limita-se a arquivos/pastas identificados como gerenciados; não remove instalações do sistema ou de terceiros.

## Licenças e identidade

A central é um projeto independente da Decode Analytics, licenciada MIT/Apache-2.0 conforme os arquivos de licença do repositório. Os aplicativos, nomes, ícones e marcas dos projetos Craft pertencem a seus respectivos titulares; confira seus repositórios para avisos e condições de redistribuição. O launcher não é afiliado oficialmente à ArtCraft/storytold.

Repositórios de referência: https://github.com/storytold/craft-launcher e https://github.com/Kaiqueaurelio/decodecad.


## Downstream Decode Studio rebranding of the 13 apps

The catalog names and original Decode SVG icon family are defined in `open-source-hub/decode-suite-manifest.json`. The source-migration tool is `scripts/rebrand_decode_suite.py`; it changes only local clones, creates/uses a `decode-branding` branch, renames matching icon resources and application IDs together, updates visible display strings and theme accents, and preserves `LICENSE-APACHE`, `LICENSE-MIT`, `NOTICE` and upstream attribution sidecars.

To prepare all 13 projects locally (this clones roughly 13 Rust workspaces, so choose a location with enough disk space):

```bash
sudo apt update
sudo apt install -y git python3-cairosvg python3-pil build-essential pkg-config \
  libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev \
  libgl1-mesa-dev libasound2-dev
python3 scripts/prepare_decode_suite.py --workspace ~/decode-suite
```

To prepare only one project and test/build it on Linux:

```bash
python3 scripts/prepare_decode_suite.py --workspace ~/decode-suite --only decode-photo --build
```

Use the `product_id` values from the manifest for `--only`: `decode-photo`, `decode-light`, `decode-film`, `decode-design`, `decode-vector`, `decode-pdf`, `decode-effects`, `decode-writer`, `decode-slides`, `decode-sheets`, `decode-draft`, `decode-audio`, and `decode-ui`.

Alternatively, GitHub Actions has a manual workflow named **Decode Suite Downstream Build**. Run it to prepare a source ZIP for one app or all 13; choose `build-linux` to additionally run the app's Rust tests and produce a Linux executable artifact. It does not push to the upstream repositories.

**Important:** changing source identity is not the same as publishing a verified release. Until a downstream build passes tests and is released, the launcher intentionally does not install an upstream Craft binary under a Decode name. The original repository, upstream release and retained legal notices remain visible for traceability.
