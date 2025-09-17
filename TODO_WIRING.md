# Wiring Plan — Proofdown Workspace Integration

Scope: Describe how to wire all crates together (AST, Parser, Validator, CLI, WASM, Schema Check, Integration Example) and the end-to-end flow expected in CI and downstream consumers (e.g., SSG). This plan enumerates integration tasks, interfaces, and acceptance criteria to keep the system deterministic, stable, and spec-compliant.

---

## System Dataflow (high-level)

1) Author writes Proofdown (`.pml`).
2) Parser (`proofdown_parser`) → `Document` (from `proofdown_ast`).
3) Validator (`proofdown_validate`) enforces structural whitelist and attribute bounds on the `Document`.
4) Renderer/SSG (downstream) consumes validated `Document` and artifact Index to produce HTML/JSON.
5) CLI (`proofdown_cli`) orchestrates parse/validate for developers and CI.
6) WASM (`proofdown_wasm`) exposes parse to web UIs with a stable JSON surface.
7) Schema Check (`schema_check`) validates JSON schema files used as advisory tables guidance.

---

## Interfaces (contracts)

- Parser API: `parse(&str) -> Result<Document, ParseError>` and `parse_with_limits(&str, ParserLimits)`
- AST types: `Document`, `Block`, `Component`, `Attr`, `ParseError { line, col, kind, msg }`, `ErrorKind`
- Validator API: `validate(doc: &Document, limits: Option<&Limits>) -> Result<(), ValidateError>`
- Validator helper: `schema_hint_for_kind(kind: &str) -> Option<&'static str>`
- CLI: `pml parse`, `pml validate` with flags (`--json`, `--pretty`, `--limits.*`)
- WASM: `wasm_parse(input: &str) -> String` (JSON payload `{ ok: bool, doc|err }`)
- Schema checker: `schema_check <dir>` validates `.specs/schemas/*.json`

---

## Now — Wiring Tasks

- Parser ↔ AST
  - [ ] Confirm AST and error structs have rustdoc explaining compatibility surface.
  - [ ] Add round-trip serde tests for AST (in `proofdown_ast`) to protect stability.

- Parser ↔ Validator ↔ CLI
  - [ ] Standardize CLI JSON errors: `{ ok: false, err: { code, msg, line?, col? } }` for parse and validate.
  - [ ] Validate via CLI for v2 fixtures in JSON mode and ensure stable success payloads.
  - [ ] Document exit codes (`0 OK`, `1 ParseError`, `2 ValidateError`, `3 IO/usage`).

- Validator ↔ Schemas
  - [ ] Use `schema_hint_for_kind()` when printing validation hints; document mapping.
  - [ ] Ensure `.specs/schemas/` are validated in CI (already), and add strict mode plan in `schema_check`.

- WASM ↔ Parser
  - [ ] Ensure `wasm_parse` outputs match native parse JSON for sample fixtures (parity test).
  - [ ] Document building and loading in README (browser + Node example).

- Integration Example (SSG-style)
  - [ ] Expand example to demonstrate: parse → validate → pretty-print errors with codes.
  - [ ] Add an optional hook to apply `json_pointer` to an example JSON value (mock resolver) for docs only.

- CI Wiring
  - [ ] Parse and validate all `.pml` fixtures via CLI in CI to mimic real usage.
  - [ ] Keep schema checks and benches present; run fuzz smoke on PRs (already wired).

---

## Next — End-to-End Scenarios

- End-to-End test (dev CI job)
  - [ ] Run `pml parse --json` on `v2_minimal.pml` and diff against native `proofdown_parser::parse` JSON to ensure parity.
  - [ ] Run `pml validate --json` on select v2 fixtures; assert `{ "ok": true }` payload.

- Authoring guide + CLI alignment
  - [ ] Ensure every authoring-guide example parses and validates under CLI.
  - [ ] Add a small script to run all examples in `.docs/` through CLI as a smoke test.

- SSG Consumer Notes (downstream)
  - [ ] Document that SSG must: verify artifacts by digest, project JSON via RFC 6901 pointers, enforce viewer bounds at render time.
  - [ ] Provide a stable mapping for `artifact.table kind` to schema filenames (advisory).

---

## Later — Optional Enhancements

- CLI UX
  - [ ] Add `--quiet` and stdin `-` support; snapshot a few human outputs.
- WASM
  - [ ] E2E parity job comparing WASM output vs native for a tiny corpus.
- Schema Check
  - [ ] Implement `--json` and `--strict`; integrate a strict profile in CI.

---

## Acceptance Criteria

- [ ] CLI: `pml parse/validate` returns deterministic JSON payloads and documented exit codes.
- [ ] Parser/Validator/WASM JSON payload parity for at least two fixtures (LF/CRLF parity retained).
- [ ] All `.specs/schemas/*.json` pass schema_check; hints available via `schema_hint_for_kind()` for known table kinds.
- [ ] Integration example builds and demonstrates parse → validate with structured error codes.
- [ ] Workspace CI green with fmt/clippy/tests/schemas/wasm-build/fuzz-smoke.

---

## Runbook (dev quickstart)

- Parse + validate a file via CLI:
  - `cargo run -p proofdown_cli -- parse crates/proofdown_parser/tests/fixtures/minimal.pml --json --pretty`
  - `cargo run -p proofdown_cli -- validate crates/proofdown_parser/tests/fixtures/minimal.pml --json`
- Native vs CLI parity check:
  - Use `proofdown_parser::parse` in a small dev snippet and compare to `pml parse --json` output.
- WASM build:
  - `rustup target add wasm32-unknown-unknown`
  - `cargo build -p proofdown_wasm --no-default-features --features wasm --target wasm32-unknown-unknown`
- Schema check in CI locally:
  - `cargo run -p schema_check -- .specs/schemas`
