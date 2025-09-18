#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[schema] cargo run -p schema_check -- .specs/schemas"
cargo run -p schema_check -- .specs/schemas
