# Developer Guide — proofdown_parser

This guide helps you build, test, lint, fuzz, and release the Proofdown parser workspace.

## Prereqs

- Rust stable toolchain (rustup)
- Optional: wasm32 toolchain if building WASM (`wasm32-unknown-unknown`)

## Build & Test

- Build all crates:
  - `cargo build --workspace`
- Run tests:
  - `cargo test --workspace`
- Update AST JSON goldens for v2 fixtures:
  - `env UPDATE_GOLDEN=1 cargo test -p proofdown_parser`

## Lint & Format

- Format:
  - `cargo fmt --all -- --check`
- Clippy:
  - `cargo clippy --all-targets -- -D warnings`

## CLI

- Parse a file:
  - `cargo run -p proofdown_cli -- parse path/to/file.pml --json --pretty`
- Validate a file with limits:
  - `cargo run -p proofdown_cli -- validate path/to/file.pml --limits.depth=8 --limits.nodes=10000 --json --pretty`

## Validator (v2 features)

- Enforced attributes:
  - `artifact.json`: `json_pointer` (RFC 6901), `collapsed`, `depth=0..8`
  - `artifact.image`: `alt` (required), `max_height=128..2048`, optional `caption`
  - `artifact.table`: optional `caption`, `columns` (keys or JSON Pointers), `kind`
  - `artifact.text`: `max_lines=1..500`, optional `caption`

## Benchmarks

- Run benches (HTML report in `target/criterion`):
  - `cargo bench -p proofdown_parser`

## Fuzzing (optional)

- We scaffolded `fuzz/` for `cargo-fuzz`:
  - Install: `cargo install cargo-fuzz`
  - Run: `cargo fuzz run parse`

## Schemas

- Basic validation of schema files:
  - `cargo run -p schema_check -- .specs/schemas`

## Release notes

- Update `CHANGELOG.md` with entries under Unreleased.
- Tag/versioning will be documented once downstream consumers are ready.
