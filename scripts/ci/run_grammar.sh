#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[grammar] parser tests"
cargo test -p proofdown_parser

echo "[grammar] validator tests"
cargo test -p proofdown_validate

echo "[grammar] CLI tests"
cargo test -p proofdown_cli
