# proofdown_parser — Workspace Organization Plan

Status: Draft (2025-09-17)

This document describes how we will organize the Rust workspace inside the `proofdown_parser` submodule so the team can iterate independently while integrating cleanly with the parent Provenance repository.

## Objectives

- Provide a nested Cargo workspace in the submodule with clear crate boundaries.
- Keep the core parser pure (no IO) and WASM-ready.
- Expose a minimal CLI and WASM package for local testing and Worker integration.
- Make it trivial for the outer repo to depend on inner crates via `path`.

## Workspace Layout (proposed)

```
proofdown_parser/                 # submodule root (this repo)
├─ Cargo.toml                     # [workspace] root for nested workspace
├─ crates/
│  ├─ proofdown_ast/              # AST types, error types, stable serde
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ proofdown_lexer/            # (optional) tokenizer if separated from parser
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ proofdown_parser/           # parser crate (depends on ast[/lexer])
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ proofdown_validate/         # whitelist + attribute validation + limits
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ proofdown_wasm/             # wasm-bindgen bindings (feature-gated)
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  └─ proofdown_cli/              # small CLI: parse, validate, dump AST JSON
│     ├─ Cargo.toml
│     └─ src/main.rs
├─ .specs/
│  └─ 00_proofdown_parser.md      # standalone spec (grammar, ABNF, registry)
├─ .plans/
│  └─ 00_workspace_plan.md        # this document
└─ README.md
```

Notes:

- We may skip `proofdown_lexer/` initially and fold tokenization into `proofdown_parser/` for simplicity.
- `proofdown_validate/` can start inside `proofdown_parser` and split out later if useful.

## Workspace Root `Cargo.toml` (draft)

```toml
[workspace]
members = [
  "crates/proofdown_ast",
  # "crates/proofdown_lexer",  # enable if split out
  "crates/proofdown_parser",
  # "crates/proofdown_validate", # enable when split out
  # "crates/proofdown_wasm",     # gated by wasm feature/tooling
  # "crates/proofdown_cli",      # optional binary for local testing
]
resolver = "2"

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"

# Optional: unify common deps to keep versions in sync
[workspace.dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
```

## Crate Responsibilities

- `proofdown_ast`
  - Types: `Document`, `Block`, `Component`, `Attr`, `ErrorKind`, `ParseError`.
  - Serde support for round-tripping AST as JSON for golden tests.
  - No parsing logic.

- `proofdown_parser`
  - Public API: `parse(&str) -> Result<Document, ParseError>`.
  - Deterministic grammar; no IO; limit accounting.
  - Feature flags for extended grammar if needed.

- `proofdown_validate` (optional initial split)
  - Whitelist enforcement and attribute schema checks.
  - Bound checking (depth, nodes, attributes) and helpful errors.

- `proofdown_wasm`
  - `wasm-bindgen` wrappers: `parse(input: &str) -> JsValue` (stable JSON AST).
  - JS-facing errors with line/column and kind codes.

- `proofdown_cli`
  - `pml parse <file>` → exit non-zero on error; `--json` dumps AST JSON.
  - `pml validate <file>` → runs attribute whitelist/limits checks.

## Integration with Parent Workspace

- The parent `provenance` workspace MUST NOT list the submodule path as a member (no nested workspace overlap).
- Parent crates (e.g., `provenance_ssg`) depend on inner crates via `path`:
  - Example once available: `proofdown_parser = { path = "crates/proofdown_parser/crates/proofdown_parser" }`.
- For now (until crates exist), parent code SHOULD gate usage via a feature flag, or temporarily stub the parser.

## Milestones

1) Bootstrap workspace: add root `Cargo.toml` with `[workspace]` and `resolver = "2"`.
2) Create `proofdown_ast` with minimal types and tests.
3) Create `proofdown_parser` with minimal parser to support `examples/minimal` front page.
4) Optional: add `proofdown_cli` for local dev ergonomics.
5) Optional: add `proofdown_wasm` bindings for Worker integration.
6) Wire parent `provenance_ssg` to depend on `proofdown_parser` via `path` and gate features.
7) Add golden tests for AST and determinism.

## Risks & Mitigations

- Nested workspace build integration:
  - Mitigation: keep outer workspace members explicit and do not include the submodule; use `path` deps to target inner crates.
- API churn across teams:
  - Mitigation: keep AST stable and versioned; expose only necessary functions; use feature flags.
- WASM size/perf:
  - Mitigation: pure Rust, small dep footprint; consider `wasm-opt` in CI.
