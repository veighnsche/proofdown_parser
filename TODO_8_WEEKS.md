# Weeks 5–8 Execution Plan — proofdown_parser

Scope: Continue additive improvements without breaking grammar/AST stability. Focus on robustness, ergonomics, and release maturity across parser, validator, CLI, WASM, CI, and docs.

---

## Week 5 — Precision, whitespace, and schemas

Goals

- Improve error location precision and whitespace handling without altering AST shape.
- Strengthen JSON Pointer and column selector validation logic.
- Map `artifact.table.kind` → minimal schema filename and add a validator helper to resolve hints.

Deliverables

- Parser
  - Accurate line/column tracking in scanners (propagate position into nested component parsing).
  - Whitespace: trim block text at edges only; do not collapse internal spaces.
- Validator
  - JSON Pointer validation: stricter checks (array indices, escape correctness); document limitations.
  - Columns: robust parsing with quoted tokens and spaces handling; retain current CSV-like surface.
  - Kind-to-schema map helper (no I/O); return clear hint messages to SSG on which schema to use.
- Tests
  - Negative tests for pointer/columns (bad escapes, out-of-range indices).
  - Golden stability checks for whitespace changes.

Acceptance criteria

- Error positions are consistent and stable in tests.
- All existing goldens unchanged; whitespace improvements don’t break AST shape.

---

## Week 6 — Integration example and WASM build path

Goals

- Provide a minimal SSG integration example and a WASM demo build path.

Deliverables

- Integration example
  - Expand `crates/proofdown_integration_example` to run `parse` + `validate` + print actionable errors with codes.
  - Example of applying `json_pointer` to a JSON value (mocked resolver) and selecting table columns.
- WASM
  - `examples/wasm/` minimal page with a textarea and parse/validate buttons (no bundler required; document `wasm-pack` flow).
  - CI job to compile `proofdown_wasm` for wasm32 with `--features wasm` and upload artifact.
- CLI
  - Add `--validate-only` short path and `--no-validate` control on `parse` (optional; document only if implemented).

Acceptance criteria

- Integration example runs locally (cargo run) and prints structured errors (with `ValidateError::code()`).
- WASM build artifacts produced in CI; demo page loads a “hello world” parse result.

---

## Week 7 — Fuzzing and performance guardrails

Goals

- Exercise parser with fuzzing and extend micro-benchmarks.

Deliverables

- Fuzzing
  - `cargo-fuzz` target `parse` with a small seed corpus (valid/invalid snippets).
  - CI scheduled job (cron) to build fuzz target (not execute long runs in PRs) and run a short smoke for crashes.
- Performance
  - Benchmarks for nested components and attributes-heavy documents; HTML report retained as CI artifact on PRs.

Acceptance criteria

- No panics from fuzzing smoke; regressions triaged.
- Bench results recorded for minimal and v2 fixtures; deltas monitored informally.

---

## Week 8 — Release Candidate polish

Goals

- Finalize RC-quality ergonomics, docs, and policies.

Deliverables

- CLI
  - Consistent exit codes and JSON outputs documented; add `--version` and `--help` refinements.
- Docs
  - Authoring-guide updates for `artifact.text`, `json_pointer`, and table `columns` best practices.
  - Security policy (no scriptable content, content-addressing), Code of Conduct.
  - Expand `CHANGELOG.md` with an RC entry; document SemVer/contract expectations.
- CI
  - Optional cache for Rust and wasm toolchains; workflow matrix to test MSRV if pinned.

Acceptance criteria

- RC checklist complete in repo; basic policies present; CLI behavior documented.

---

## Tracking checklist (Weeks 5–8)

- Parser
  - [ ] Precise line/col tracking in nested scans
  - [ ] Whitespace trimming at edges only
- Validator
  - [ ] Strict JSON Pointer checks; columns parser robustness
  - [ ] `kind` → schema filename helper (advisory)
- Integration
  - [ ] Integration example applies pointers/columns in a mock flow
  - [ ] WASM demo page and CI build artifacts
- Quality
  - [ ] Fuzz target and scheduled CI smoke
  - [ ] Bench coverage for nested and attribute-heavy docs
- Docs & Policy
  - [ ] Authoring-guide updates for v2 features
  - [ ] Security policy and Code of Conduct
  - [ ] RC changelog entry
