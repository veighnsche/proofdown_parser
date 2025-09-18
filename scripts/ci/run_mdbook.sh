#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." >/dev/null 2>&1 && pwd)"
cd "$ROOT_DIR"

echo "[mdbook] build book"
if ! command -v mdbook >/dev/null 2>&1; then
  echo "mdbook not found; install with: cargo install mdbook" >&2
  exit 3
fi
mdbook build book
