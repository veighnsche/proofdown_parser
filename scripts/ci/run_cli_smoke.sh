#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[cli-smoke] parse fixtures (*.pml)"
for f in crates/proofdown_parser/tests/fixtures/*.pml; do
  echo "  parsing $f"
  cargo run -q -p proofdown_cli -- parse "$f" --json >/dev/null
done

echo "[cli-smoke] validate minimal.pml"
cargo run -q -p proofdown_cli -- validate crates/proofdown_parser/tests/fixtures/minimal.pml --json | grep -E '"ok":[[:space:]]*true' >/dev/null
