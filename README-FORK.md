# Decode CAD — fork comunitário de Kaique Aurélio

Este repositório contém o Decode CAD, uma versão derivada do projeto aberto [CADCraft original](https://github.com/storytold/cadcraft), mantida por Kaique Aurélio.

## Estado do projeto

**Primeiro build multiplataforma concluído com sucesso em 24/10/2026:** o GitHub Actions compilou executáveis para Linux x86-64, Windows x86-64 e macOS. Os arquivos estão disponíveis temporariamente na [execução de build bem-sucedida](https://github.com/Kaiqueaurelio/cadcraft/actions/runs/40096986796), na seção **Artifacts**. O workflow `.github/workflows/build.yml` disponibiliza esses artefatos por 14 dias.

**Importante:** a compilação passou, mas isso não substitui testes manuais do aplicativo em cada sistema operacional. Os arquivos são executáveis compactados, não instaladores assinados (MSI, AppImage ou DEB); ainda não devem ser tratados como versão estável.

## Compilar localmente

Requer uma versão do Rust/Cargo compatível com a versão indicada por `rust-version` no `Cargo.toml`, além das dependências de sistema necessárias ao projeto.

```bash
git clone https://github.com/Kaiqueaurelio/cadcraft.git
cd cadcraft
cargo build --locked --release -p cadcraft
```

## Créditos e licenças

Este projeto deriva de [storytold/cadcraft](https://github.com/storytold/cadcraft). Preserve todos os avisos de copyright, créditos, arquivos de licença e atribuições existentes. O projeto original declara licença `MIT OR Apache-2.0`; leia os arquivos de licença incluídos antes de redistribuir.

## Segurança dos arquivos

O Decode CAD está em desenvolvimento inicial e pode ter limitações de compatibilidade de arquivos. Mantenha cópias de segurança dos desenhos importantes e valide os arquivos exportados antes de usá-los em produção.
