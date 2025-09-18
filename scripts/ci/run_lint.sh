#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[lint] cargo fmt"
cargo fmt --all -- --check

echo "[lint] cargo clippy"
cargo clippy --all-targets -- -D warnings
