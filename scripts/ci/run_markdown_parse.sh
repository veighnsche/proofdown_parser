#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

BIN="./target/debug/pml"

if [[ ! -x "$BIN" ]]; then
  echo "[md-parse] building proofdown_cli once"
  cargo build -q -p proofdown_cli
fi

# Collect markdown files tracked by git (avoid build artifacts)
mapfile -t md_files < <(git ls-files '*.md' | grep -Ev '^(book/book/|target/|node_modules/)')
echo "[md-parse] Found ${#md_files[@]} Markdown files"
for f in "${md_files[@]}"; do
  echo "[md-parse] parsing $f"
  "$BIN" parse "$f" --json > /dev/null
done

# Collect proofdown fixtures
mapfile -t pml_files < <(git ls-files '*.pml')
for f in "${pml_files[@]}"; do
  echo "[md-parse] parsing $f"
  "$BIN" parse "$f" --json > /dev/null
done

echo "[md-parse] OK"
