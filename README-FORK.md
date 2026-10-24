# Decode CAD — desenvolvido pela Decode Analytics

O **Decode CAD** é uma edição de CAD mantida pela **Decode Analytics**. Este repositório reúne o código-fonte, instruções de compilação e avisos de licenças aplicáveis.

## Estado do projeto

A compilação multiplataforma e a estabilidade devem ser avaliadas pelas execuções de CI e pelos testes correspondentes. Uma compilação bem-sucedida, por si só, não equivale a uma versão estável nem garante testes manuais em todos os sistemas operacionais.

Consulte [GitHub Actions](https://github.com/Kaiqueaurelio/cadcraft/actions) para acompanhar builds e testes recentes. Artefatos podem ser temporários; confira a execução e o commit antes de usar qualquer binário.

## Compilar localmente

Requer Rust/Cargo compatíveis com a versão declarada em `Cargo.toml`, além das dependências de sistema necessárias à plataforma.

```bash
git clone https://github.com/Kaiqueaurelio/cadcraft.git
cd cadcraft
cargo build --locked --release -p cadcraft
```

## Identidade e créditos

Esta edição é apresentada e mantida pela **Decode Analytics**. O nome Decode CAD identifica o fork e sua distribuição; não implica que todo o código ou todas as dependências tenham sido originalmente escritos pela Decode Analytics.

## Licenças e avisos obrigatórios

O repositório contém as licenças [MIT](LICENSE-MIT) e [Apache-2.0](LICENSE-APACHE). Preserve os avisos de copyright, os arquivos de licença, os avisos e as atribuições exigidos para componentes derivados e dependências. Consulte [NOTICE](NOTICE) e [ATTRIBUTION.md](ATTRIBUTION.md).

## Segurança dos arquivos

O projeto está em desenvolvimento e pode ter limitações de compatibilidade de formatos. Mantenha cópias de segurança dos desenhos importantes e valide os arquivos exportados antes de usá-los em produção.
