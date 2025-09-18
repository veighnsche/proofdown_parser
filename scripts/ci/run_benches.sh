#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[benches] cargo bench -p proofdown_parser --no-run"
cargo bench -p proofdown_parser --no-run
