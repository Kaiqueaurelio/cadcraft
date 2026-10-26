#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "$BASH_SOURCE")/.." && pwd)"
MANIFEST="$ROOT/open-source-hub/decode-suite-manifest.json"
REBRAND="$ROOT/scripts/rebrand_decode_suite.py"
DESTINATION="$PWD/decode-studio-source"
if [[ $# -gt 0 ]]; then DESTINATION="$1"; fi

if [[ -e "$DESTINATION" ]] && [[ -n "$(find "$DESTINATION" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null)" ]]; then
  echo "Erro: o destino não está vazio: $DESTINATION" >&2
  echo "Escolha uma pasta nova para evitar sobrescrever trabalho existente." >&2
  exit 2
fi

mkdir -p "$DESTINATION"
python3 -m venv "$DESTINATION/.venv"
PYTHON="$DESTINATION/.venv/bin/python"
"$PYTHON" -m pip install --upgrade pip
"$PYTHON" -m pip install cairosvg Pillow

echo "Clonando os 13 projetos públicos com seus submódulos..."
while IFS=$'\t' read -r SLUG PRODUCT_ID; do
  [[ -n "$SLUG" ]] || continue
  TARGET="$DESTINATION/$PRODUCT_ID"
  echo "  -> $PRODUCT_ID (storytold/$SLUG)"
  git clone --recurse-submodules --depth 1 "https://github.com/storytold/$SLUG.git" "$TARGET"
done < <(python3 -c 'import json,sys; d=json.load(open(sys.argv[1],encoding="utf-8")); [print(p["upstream_slug"]+"\t"+p["product_id"]) for p in d["products"]]' "$MANIFEST")

echo "Aplicando nomes, IDs, ícones e tema Decode..."
"$PYTHON" "$REBRAND" --workspace "$DESTINATION" --manifest "$MANIFEST" --icons "$ROOT/open-source-hub/assets/icons"

echo
echo "Concluído. Os 13 clones downstream estão em: $DESTINATION"
echo "Os remotes originais foram renomeados para 'upstream'; nenhum push foi feito."
echo "As licenças e os avisos de atribuição foram preservados."
echo "Próximo passo: revisar os diffs e compilar/testar cada workspace antes de distribuir builds."
