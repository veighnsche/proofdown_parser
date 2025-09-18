#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

export RUST_BACKTRACE=1

echo "[test] cargo test --workspace --all-features"
cargo test --workspace --all-features
