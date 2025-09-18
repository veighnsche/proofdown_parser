#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[fuzz] smoke test"
if ! cargo fuzz --help >/dev/null 2>&1; then
  echo "cargo-fuzz not found; install with: cargo install cargo-fuzz" >&2
  exit 3
fi
cargo fuzz build
RUST_BACKTRACE=1 cargo fuzz run parse -- -runs=1000
