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

## v2 adoption plan (additive over v1)

Goals:

- Keep parser grammar unchanged while supporting v2 viewers/attributes in validation and SSG.
- Harden interoperability via minimal JSON Schemas for common `artifact.table kind`s.
- Expand the normative test corpus with v2 conformance fixtures.

Tasks:

1) Validator updates (proofdown_validate or SSG validator)
   - Enforce new bounded attributes: `artifact.json.json_pointer`, `artifact.image.caption`, `artifact.table.caption`, `artifact.text.max_lines`.
   - Recognize `artifact.text` as a valid viewer; validate `max_lines` bounds (1..500) and UTF-8 text assumption.
   - Support `artifact.table` column selectors that are simple keys or RFC 6901 JSON Pointers (static projection only).
   - Optionally validate `kind`-specific row shapes using `.specs/schemas/*.schema.json`.

2) Parser notes
   - Grammar unchanged; component syntax continues as-is.
   - If the parser enforces a whitelist, add `artifact.text` to the allowed set (gated by a feature or version flag as needed).

3) Renderer/SSG
   - Add captions rendering for images/tables/links.
   - Implement JSON Pointer projection for `artifact.json` and `artifact.table` (static, no query semantics).
   - Respect bounds with clear truncation/elision indicators.
   - Fail-closed on digest mismatch/missing artifacts.

4) Test corpus
   - Add v2 fixtures under `crates/proofdown_parser/tests/fixtures/` (examples across categories).
   - Ensure parser can parse all fixtures (`parse_v2_fixtures.rs` test added).
   - Add renderer/validator tests in downstream repos to assert v2 behaviors and failure modes.

5) Docs
   - Link `.specs/03_proofdown_language_v2.md` from README and related docs.
   - Maintain `.specs/schemas/` with a `README.md` summarizing usage.
   - Keep the authoring guide aligned with v2 patterns (captions, pointers, `artifact.text`).

Rollout:

- Default renderers remain compatible with v1 documents.
- Capabilities flag may advertise v2 support; documents can opt to v2 patterns incrementally.
