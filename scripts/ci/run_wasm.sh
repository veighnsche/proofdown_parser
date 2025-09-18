#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[wasm] build proofdown_wasm for wasm32-unknown-unknown"
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
  echo "  adding wasm32-unknown-unknown target"
  rustup target add wasm32-unknown-unknown
fi
cargo build -p proofdown_wasm --no-default-features --features wasm --target wasm32-unknown-unknown
