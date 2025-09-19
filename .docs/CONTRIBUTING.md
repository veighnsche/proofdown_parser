# Contributing Guide

Thank you for your interest in contributing to Proofdown!

- Code of Conduct: be kind, respectful, and constructive.
- Workflow: fork, branch, PR. Keep changes small and focused.
- CI: ensure `cargo fmt`, `cargo clippy`, and `cargo test --workspace` pass.
- Style: Rust 2021 edition, no unsafe; avoid global mutable state.
- Commit messages: concise imperative subject; body describing why.

## Areas

- Parser (`crates/proofdown_parser`)
- AST (`crates/proofdown_ast`)
- CLI (`crates/proofdown_cli`)
- WASM (`crates/proofdown_wasm`)
- Schema checks (`crates/schema_check`)

## Tests

- Unit tests live next to code, integration tests under `tests/`.
- Golden tests: update by setting `UPDATE_GOLDEN=1` and re-running tests; review diffs carefully.
- Fuzzing: run `cargo fuzz run parse` locally for smoke.

## Releasing

- See `.docs/RELEASING.md`.
