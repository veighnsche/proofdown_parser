# 4-Week Execution Plan — proofdown_parser

Scope: proofdown_parser repo (parser/AST/CLI/WASM), plus validator/SSG touchpoints and docs/specs that live here. v2 is additive; grammar remains stable. We will focus on validation, tests, CI, robustness, and release-readiness.

Notes

- Parser stays syntax-only. v2 features (captions, json_pointer, artifact.text, table kind schemas) are validated/rendered downstream.
- Use the normative test corpus and goldens to guard compatibility. Prefer deterministic AST JSON for comparisons.
- All artifacts and docs remain script-free, deterministic, and content-addressed.

---

## Week 1 — Validation + CI baseline

Goals

- Implement v2 validator behavior in `proofdown_validate` (or SSG-side), without changing grammar.
- Add CI gates for build/test/clippy/fmt and run parser tests + goldens.
- Validate `artifact.table` row shapes against minimal JSON Schemas (informative but useful).

Deliverables

- Validator
  - Enforce: `artifact.json.json_pointer` (RFC 6901 static projection), `artifact.image.caption`, `artifact.table.caption`.
  - Add `artifact.text(max_lines=1..500, caption?)` to whitelist and bounds.
  - Support `artifact.table` column selectors as JSON Pointer or simple keys; static projection only.
  - Optional: `kind` row validation against `.specs/schemas/*.schema.json`.
- Tests
  - Unit tests for each bound; error messages stable and human-readable.
  - Golden tests for v2 fixtures parse (already added) continue to pass.
- CI
  - Workflow: build, test, clippy, fmt; artifact caching (optional).
  - Job to run schema validation on JSON examples where present.

Acceptance criteria

- CI is green on a PR that includes validator unit tests and existing parser goldens.
- Invalid pointers/attribute bounds yield structured, stable errors.

---

## Week 2 — Integration & fixtures expansion

Goals

- Add a minimal integration example or doc snippet showing SSG consumption (path dep) and validator usage.
- Expand conformance fixtures and goldens to cover more v2 patterns and edge cases.

Deliverables

- Examples/Docs
  - Minimal SSG/validator wiring example (code or doc) using `parse` + `validate`.
  - Authoring guide updates where needed (short examples for captions/pointers/text viewer).
- Fixtures
  - Add more v2 `.pml` fixtures for: mutation testing, chaos/resiliency, snapshots with mixed media, large JSON collapsed-by-default.
  - Add negative fixtures for validator (out-of-bounds, bad pointer, unknown attribute) with expected errors.

Acceptance criteria

- Integration snippet/example compiles and runs locally.
- New fixtures parse; validator catches negatives; goldens updated under `UPDATE_GOLDEN=1` as needed.

---

## Week 3 — Robustness (fuzzing, performance)

Goals

- Ensure parser and validator behave well on adversarial inputs and large docs; no panics, predictable limits.

Deliverables

- Fuzzing
  - Set up `cargo-fuzz` target for the parser; basic corpus and crash repro flow.
- Performance
  - Micro-bench harness for common documents (headings, nested components, attributes) and large-but-valid inputs.
  - Guardrails: Assert limits (depth/nodes/input bytes) are enforced quickly and with clear errors.

Acceptance criteria

- Fuzzing runs locally and in CI (nightly or scheduled) without panics; any findings triaged.
- Benchmarks show linear or near-linear behavior; regressions are detectable.

---

## Week 4 — Release readiness & docs polish

Goals

- Prepare for internal release/tag with clear docs, CLI affordances, and changelog.

Deliverables

- CLI
  - Add flags: `--json`, `--pretty`, and limits overrides (`--limits.depth`, `--limits.nodes`, `--limits.input-size`).
  - Snapshot tests for CLI output (goldens).
- Docs
  - CONTRIBUTING/Developer Guide: build/test/fuzz/CI; how to update goldens; release checklist.
  - Module-level rustdoc with grammar highlights and examples.
- Release & Versioning
  - CHANGELOG.md with rules for patch/minor/major.
  - Tag + version bump plan documented (actual tagging can follow once downstream is ready).

Acceptance criteria

- CLI flags work and are covered by tests.
- Docs updated and discoverable from README and specs.
- Changelog initialized and kept up-to-date.

---

## Cross-cutting checks (all weeks)

- Determinism: AST JSON goldens remain stable across runs/platforms.
- Safety: No IO in parser; validator rejects unknown components/attributes and enforces bounds.
- Accessibility: Encourage captions and alt text; large JSON collapsed by default in examples.
- Backward compatibility: v1 documents continue to parse; v2 is additive.

---

## Tracking checklist

- Parser
  - [ ] Keep grammar stable; improve whitespace and paragraph handling as needed
  - [ ] Depth/node/input limits thoroughly exercised
- Validator
  - [ ] v2 attributes and viewers enforced (`artifact.text`, captions, json_pointer)
  - [ ] `artifact.table` column selectors and `kind` row validation
- Tests/Goldens
  - [ ] Expand v2 fixtures and negatives; keep goldens current
  - [ ] Fuzzing harness and corpora
  - [ ] Performance micro-benchmarks
- CI
  - [ ] Build/test/clippy/fmt pipelines
  - [ ] Optional: scheduled fuzzing and benchmark checks
- Docs
  - [ ] Authoring guide and specs cross-linked
  - [ ] CONTRIBUTING + Developer Guide
  - [ ] CHANGELOG and release checklist

---

## How to run (dev quickstart)

- Update v2 goldens when fixtures change:
  - `env UPDATE_GOLDEN=1 cargo test -p proofdown_parser`
- Run tests normally:
  - `cargo test -p proofdown_parser`
- Validate style/lints locally (once CI is in place):
  - `cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings`
