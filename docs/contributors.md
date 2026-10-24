# Colaboradores na janela Sobre

A seção **About ▸ Contributors** credita quem contribuiu com o CADCraft; **About ▸ Models** credita os modelos de IA citados nos rodapés `Co-Authored-By` dos commits. O procedimento segue o padrão compartilhado do craftrules em [standards/contributors.md](https://github.com/storytold/craftrules/blob/main/standards/contributors.md). Esta página registra a decisão específica do repositório.

## Decisão e arquivos

Dois arquivos são incorporados ao binário pela função `build.rs` do crate de interface. Nada é lido do disco ou da rede durante a execução; a versão web também inclui esses dados.

- [`contributors/contributors.json`](../contributors/contributors.json): estatísticas de commits por nome de usuário do GitHub. É **gerado** por `craftrules/scripts/contributors.py`; nunca edite manualmente.
- [`contributors/people.toml`](../contributors/people.toml): nomes escolhidos pelas próprias pessoas. **Cada colaborador adiciona apenas sua própria entrada**, em seu próprio pull request.

Cada colaborador recebe uma linha com nome de usuário do GitHub (sempre), nome de exibição (opcional ou sincronizado com o perfil público), nome real (somente se informado pela pessoa), pull requests integrados, commits, linhas adicionadas e removidas, recursos binários adicionados e removidos e datas do primeiro e último commit.

A lista pode ser exibida como uma lista de nomes ou uma tabela, mantendo a mesma ordem. É possível ordenar por nome, pull requests, commits, linhas adicionadas/removidas, saldo de linhas, recursos binários e data do primeiro ou último commit. O seletor de nome alterna entre **Username → Display name → Real name**; quando não há nome, usa `@username`. A ordenação alfabética ignora maiúsculas/minúsculas e o caractere `@`.

## Adicione seus créditos

Inclua sua entrada em `contributors/people.toml` por meio de um pull request enviado por você. Todos os campos são opcionais:

```toml
[people.seu-usuario-github]
real_name = "Seu nome"
display_name = "Apelido"
sync_github_name = true   # usa como nome de exibição o nome do perfil público do GitHub
```

Ou, com o repositório craftrules clonado ao lado de `craft-apps/`:

```sh
python3 ../../craftrules/scripts/contributors.py --add-me . --real-name "Seu nome" --sync-github-name
```

Os nomes aparecem após a próxima atualização. O script verifica se a entrada foi enviada pela própria pessoa ou por um mantenedor com `consent_source`. Remova sua entrada para retirar seus nomes da lista.

## Atualização por mantenedores

```sh
python3 ../../craftrules/scripts/contributors.py .          # gera novamente e verifica; requer git, Python 3.11+ e gh
python3 ../../craftrules/scripts/contributors.py --check .  # verifica apenas people.toml
```

Faça commit do arquivo atualizado `contributors/contributors.json`. Os nomes e e-mails de autoria registrados pelo Git nunca são exibidos.
