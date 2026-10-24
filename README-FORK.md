# CADCraft — fork comunitário de Kaique Aurélio

Este repositório é uma versão derivada do projeto aberto [CADCraft original](https://github.com/storytold/cadcraft), mantida por Kaique Aurélio.

## Estado do projeto

Este fork está em fase de preparação. O workflow em `.github/workflows/build.yml` tenta compilar o aplicativo desktop para Linux, Windows e macOS e disponibiliza os executáveis como artefatos temporários do GitHub Actions quando a compilação conclui com sucesso.

**Importante:** ainda não anuncie os artefatos como versões estáveis antes de conferir os resultados do workflow e testar os executáveis nos sistemas correspondentes. Os artefatos não são instaladores assinados (MSI, AppImage ou DEB).

## Compilar localmente

Requer uma versão do Rust/Cargo compatível com `rust-version` no `Cargo.toml), além das dependências de sistema necessárias ao projeto.

```bash
git clone https://github.com/Kaiqueaurelio/cadcraft.git
cd cadcraft
cargo build --locked --release -p cadcraft
```

## Créditos e licenças

Este projeto deriva de [storytold/cadcraft](https://github.com/storytold/cadcraft). Preserve todos os avisos de copyright, créditos, arquivos de licença e atribuições existentes. O projeto original declara licença `MIT OR Apache-2.0`; leia os arquivos de licença incluídos antes de redistribuir.

## Segurança dos arquivos

O CADCraft está em desenvolvimento inicial e pode ter limitações de compatibilidade de arquivos. Mantenha cópias de segurança dos desenhos importantes e valide os arquivos exportados antes de usá-los em produção.
