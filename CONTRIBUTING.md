# Contributing to proofdown_parser

Thanks for your interest in contributing! This repository contains a Rust workspace for the Proofdown parser and related crates.

## Getting started

- Clone and open the repo in your IDE.
- Build the workspace:
  - `cargo build --workspace`
- Run tests (including goldens):
  - `cargo test --workspace`

## Code style & linting

- Format: `cargo fmt --all -- --check`
- Lints: `cargo clippy --all-targets -- -D warnings`

## Tests & goldens

- Parser goldens: update with `env UPDATE_GOLDEN=1 cargo test -p proofdown_parser`.
- Add parse cases under `crates/proofdown_parser/tests/cases/<name>/` with `input.pml` and `expected.json` or `expected_error.json`.
- v2 fixtures live in `crates/proofdown_parser/tests/fixtures/` and goldens in `crates/proofdown_parser/tests/golden/`.

## CI

- GitHub Actions workflow runs fmt, clippy, tests, and schema checks on PRs.

## Release process

- Follow `CHANGELOG.md` and open PRs with the `contract-change` label if the AST JSON or error model changes.
- Tagging/publishing will be coordinated with downstream consumers.

## Conduct

- Be respectful and collaborative. Provide clear reproduction steps for issues and tests for fixes.
