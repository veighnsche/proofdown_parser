#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

SKIP_WASM=0
SKIP_FUZZ=0
SKIP_MDBOOK=0
SKIP_MD_PARSE=0

usage() {
  cat <<'EOF'
Usage: scripts/ci/run_all.sh [options]
  --skip-wasm            Skip wasm build
  --skip-fuzz            Skip fuzz smoke
  --skip-mdbook          Skip mdBook build
  --skip-markdown-parse  Skip parsing all Markdown/PML
EOF
}

for arg in "$@"; do
  case "$arg" in
    --skip-wasm) SKIP_WASM=1 ;;
    --skip-fuzz) SKIP_FUZZ=1 ;;
    --skip-mdbook) SKIP_MDBOOK=1 ;;
    --skip-markdown-parse) SKIP_MD_PARSE=1 ;;
    --help|-h) usage; exit 0 ;;
    *) echo "Unknown flag: $arg" >&2; usage; exit 3 ;;
  esac
done

bash scripts/ci/run_lint.sh
bash scripts/ci/run_test.sh
bash scripts/ci/run_schema.sh
bash scripts/ci/run_benches.sh
bash scripts/ci/run_cli_smoke.sh
bash scripts/ci/run_grammar.sh

if [[ "$SKIP_MD_PARSE" -eq 0 ]]; then
  bash scripts/ci/run_markdown_parse.sh
else
  echo "[all] markdown parsing skipped"
fi

if [[ "$SKIP_WASM" -eq 0 ]]; then
  bash scripts/ci/run_wasm.sh
else
  echo "[all] wasm build skipped"
fi

if [[ "$SKIP_MDBOOK" -eq 0 ]]; then
  bash scripts/ci/run_mdbook.sh
else
  echo "[all] mdBook build skipped"
fi

if [[ "$SKIP_FUZZ" -eq 0 ]]; then
  bash scripts/ci/run_fuzz_smoke.sh
else
  echo "[all] fuzz smoke skipped"
fi

echo "[all] DONE: all checks passed"
