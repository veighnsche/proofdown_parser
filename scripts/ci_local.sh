#!/usr/bin/env bash
set -euo pipefail

# Local CI runner mirroring .github/workflows/ci.yml
# Usage: scripts/ci_local.sh [--skip-wasm] [--skip-fuzz] [--skip-mdbook]

SKIP_WASM=0
SKIP_FUZZ=0
SKIP_MDBOOK=0

for arg in "$@"; do
  case "$arg" in
    --skip-wasm) SKIP_WASM=1 ;;
    --skip-fuzz) SKIP_FUZZ=1 ;;
    --skip-mdbook) SKIP_MDBOOK=1 ;;
    *) echo "Unknown flag: $arg" >&2; exit 3 ;;
  esac
done

echo "[ci] fmt"
cargo fmt --all -- --check

echo "[ci] clippy"
cargo clippy --all-targets -- -D warnings

echo "[ci] tests (workspace)"
cargo test --workspace --all-features

echo "[ci] schema check"
cargo run -p schema_check -- .specs/schemas

echo "[ci] benches build"
cargo bench -p proofdown_parser --no-run

echo "[ci] cli smoke (parse fixtures)"
for f in crates/proofdown_parser/tests/fixtures/*.pml; do
  echo "    parsing $f"
  cargo run -q -p proofdown_cli -- parse "$f" --json >/dev/null
done

echo "[ci] cli smoke (validate minimal)"
cargo run -q -p proofdown_cli -- validate crates/proofdown_parser/tests/fixtures/minimal.pml --json | grep '"ok": true' >/dev/null

if [[ "$SKIP_WASM" -eq 0 ]]; then
  echo "[ci] wasm build"
  if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo "  adding wasm32-unknown-unknown target"
    rustup target add wasm32-unknown-unknown
  fi
  cargo build -p proofdown_wasm --no-default-features --features wasm --target wasm32-unknown-unknown
else
  echo "[ci] wasm build skipped"
fi

if [[ "$SKIP_MDBOOK" -eq 0 ]]; then
  echo "[ci] mdBook build"
  if ! command -v mdbook >/dev/null 2>&1; then
    echo "  mdbook not found; install with: cargo install mdbook" >&2
    exit 3
  fi
  mdbook build book
else
  echo "[ci] mdBook build skipped"
fi

if [[ "$SKIP_FUZZ" -eq 0 ]]; then
  echo "[ci] fuzz smoke"
  if ! cargo fuzz --help >/dev/null 2>&1; then
    echo "  cargo-fuzz not found; install with: cargo install cargo-fuzz" >&2
    exit 3
  fi
  cargo fuzz build
  RUST_BACKTRACE=1 cargo fuzz run parse -- -runs=1000
else
  echo "[ci] fuzz smoke skipped"
fi

echo "[ci] DONE: all checks passed"
