# TODO — proofdown_parser

Last updated: 2025-09-17

This document tracks implementation tasks, milestones, and decisions for the Proofdown parser submodule. It is derived from `.specs/00_proofdown_parser.md`, `.plans/00_workspace_plan.md`, and the current code under `crates/proofdown_parser/`.

## Now / Next / Later

- [ ] NOW: Solidify MVP parser behavior to match v1 scope
  - [x] Ensure headings `#..####` parse robustly with trailing spaces and empty titles guarded
  - [x] Improve paragraph accumulation across lines until blank or a component/heading start
  - [x] Harden component parsing of self-closing tags `<name ... />` and closing `</name>` detection (mismatched names)
  - [x] Attribute parsing: support both quoted and bare values exactly
  - [ ] Reject malformed key/value pairs with structured errors
  - [x] Introduce depth and node counting during parse to prepare for limits enforcement
  - [x] Enforce default parser limits: depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB
- [ ] NEXT: Add validation pass and structured error model
  - [x] Define `ErrorKind` and `ParseError { line, col, kind, msg }`
  - [x] Enforce component whitelist and attribute schemas from the spec
  - [x] Expose configurable limits in API (defaults enforced in parser): depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB
  - [ ] Establish stable error messages and codes for golden tests
- [ ] LATER: Developer ergonomics and platform surfaces (crates scaffolded)
  - [ ] CLI: extend commands and flags; polish UX and errors (`proofdown_cli` exists)
  - [ ] WASM: refine bindings and add example; CI wasm-opt (`proofdown_wasm` exists)
  - [ ] Link macro parser (v1.1) and expanded components; Includes support (v1.2)

## Milestones & Acceptance Criteria

- [x] M1: Workspace bootstrap
  - [x] Root workspace `Cargo.toml` with resolver = 2 and shared deps
  - [x] `cargo build --workspace` succeeds

- [x] M2: AST crate extracted (`proofdown_ast`)
  - [x] Types `Document`, `Block`, `Component`, `Attr` live in AST crate with serde
  - [x] Parser depends on AST crate

- [x] M3: Minimal parser crate (`proofdown_parser`)
  - [x] `parse(&str) -> Result<Document>` implemented
  - [x] Paragraph accumulation across lines; attributes preserved
  - [x] Default limits enforced (depth, nodes, input size)
  - [x] Smoke test passes on a minimal example

- [x] M4: CLI for local dev (`proofdown_cli`)
  - [x] `pml parse <file> [--json] [--pretty]` prints AST JSON
  - [x] `pml validate <file>` calls validator stub and prints OK on success

- [x] M5: WASM bindings scaffold (`proofdown_wasm`)
  - [x] `wasm_parse(input: &str) -> String` returns `{ ok, doc|err }` JSON
  - [ ] Example page and CI build job

- [ ] M6: Structured error model (contract §5)
  - [ ] `ParseError` + `ErrorKind` defined and used in parser
  - [ ] CLI/WASM surfaces show structured errors (line, col, kind, msg)

- [ ] M7: Golden tests & determinism gate
  - [ ] Add golden fixtures and byte-for-byte JSON comparisons
  - [ ] Error-path fixtures (unterminated/mismatched/over-depth)

## Parser Work (crates/proofdown_parser)

- [ ] Normalize newlines to `\n` (input is UTF-8; treat `\r\n` as `\n` consistently)
- [x] Headings:
  - [x] Require a single space after `#..####` per grammar; otherwise treat as paragraph
  - [x] Trim trailing spaces; disallow empty titles
- [ ] Whitespace handling:
  - [ ] Preserve attribute values exactly (done for bare/quoted)
  - [ ] Trim block text lines only at edges; avoid collapsing internal spaces
- [x] Components:
  - [x] Detect mismatched close tags as `Syntax` with best-effort line/col
  - [x] Detect unterminated components with best-effort line/col
  - [x] Support self-closing `<name ... />` accurately
- [ ] Mixed content inside components: paragraphs and headings supported; consider lists/code fences later
- [ ] Performance: avoid quadratic behavior during scans; prefer single-pass with bounded lookahead
- [x] API: non-panicking typed errors (replace `anyhow::bail!`) and expose `parse_with_limits` variant
- [x] Configurable options: `max_depth`, `max_nodes`, `max_input_bytes` via builder or fn args

## Validation Pass (planned `proofdown_validate` or inside parser initially)

- [ ] Encode component registry and attribute schemas:
  - Structural: `grid(cols=1..6, gap=0..64)`, `section(title)`, `card(title)`
  - Artifacts: `artifact.summary(id)`, `artifact.table(id)`, `artifact.json(id, collapsed, depth=0..8)`, `artifact.markdown(id)`, `artifact.image(id, alt, max_height=128..2048)`, `artifact.link(id, download, title)`
  - Optional repo viewers (if artifacts resolvers exist): `repo.code`, `repo.link`, `repo.tree`, `repo.diff`, `repo.symbol`
- [ ] Attribute types and bounds:
  - Integers: decimal with inclusive ranges
  - Booleans: `true|false`
  - Enums: fixed sets where applicable
  - Strings: quoted or safe bare `[A-Za-z0-9._\-/:]+`
  - Glob patterns for `include`/`exclude`: safe subset; no `..`; anchored under repo root
- [x] Unknown component/attribute MUST error (validation stage)
- [x] Limits enforcement with structured errors (`DepthExceeded`, `SizeLimit`)

## Error Model Implementation (contract §5)

- [x] Define in `proofdown_ast` the following types:
  - [x] `ErrorKind` = `Syntax | LimitExceeded`
  - [x] `ParseError { line: usize, col: usize, kind: ErrorKind, msg: String }`
- [x] Replace all `anyhow::Error` in `proofdown_parser` with `ParseError`
- [ ] Line/Column tracking:
  - [ ] Maintain a `(line, col)` cursor while scanning input (treat `\n` as line increment, reset col)
  - [ ] For nested components, propagate current position to child parsers
  - [ ] On errors, return nearest known position
- [x] Update CLI to print structured errors with location and kind
- [x] Update WASM to return `{ ok: false, err: { line, col, kind, msg } }`

## Link Macro (v1.1)

- [ ] Implement ABNF from spec for targets and labels
- [ ] Normalize semantics: treat `path_shorthand` as `repo:<path>`
- [ ] Define AST representation for links (inline) or surface through a token stream; decide how inline constructs integrate with current block-level AST
- [ ] Validate targets per repository rules (no traversal; commit-pinned)

## AST and Error Model (`proofdown_ast`)

- [x] Extract `Document`, `Block`, `Component`, `Attr` to `crates/proofdown_ast`
- [ ] Define `ErrorKind` and `ParseError` with `serde` for stable JSON error reporting
- [ ] Provide stable `serde` for AST round-tripping in tests
- [ ] Implement `Display` and error codes for all `ErrorKind` variants

## WASM Surface (`proofdown_wasm`)

- [x] Create `wasm-bindgen` wrappers: `wasm_parse(input: &str) -> String`
- [ ] Map `ParseError` to a JS object `{ line, col, kind, msg }`
- [ ] Optimize build size (feature gates, `wasm-opt` in CI)
- [ ] Provide a small example in `examples/wasm` with bundler config

## CLI (`proofdown_cli`)

- [x] Command: `pml parse <file>` → print AST JSON; exit non-zero on error
- [x] Command: `pml validate <file>` → perform whitelist and limits checks
- [ ] Flags: `--json`, `--pretty`, `--limits.depth`, `--limits.nodes`, `--limits.input-size`
- [ ] Snapshot tests for CLI output

## Testing & Quality

- [ ] Golden tests from `.specs/00_proofdown_parser.md` examples
- [ ] Determinism tests: parse → serialize JSON → compare
- [ ] Error-path tests: unknown component, bad nesting, bad attributes, depth exceeded
- [ ] Fuzzing (later): feed random inputs; assert no panics and reasonable error coverage
- [ ] Benchmarks: large documents; ensure linear or near-linear time behavior

## CI & Quality Gates

- [ ] Add CI workflow: build, test, clippy, fmt check, wasm build (feature)
- [ ] Add `cargo fmt` and `cargo clippy` to Developer Guide
- [ ] Optional: cache cargo and wasm toolchains for faster CI

## Release & Versioning (contract §7)

- [ ] Add CHANGELOG.md with rules: patch/minor/major per contract
- [ ] Tag releases and publish crates (internal or crates.io as applicable)
- [ ] Document the `contract-change` label and PR process

## Integration with `provenance_ssg`

- [ ] Provide an integration example crate or doc snippet using `external_pml` feature
- [ ] Add an integration test (optional) gated by a workspace feature that compiles a minimal SSG consumer

## Ownership & Contacts

- Owner: @veighnsche
- Parser crate maintainer(s): TBD
- Validator crate maintainer(s): TBD

## Dependencies & Tooling

- Rust toolchain: 1.75+ (or repository `rust-toolchain.toml` pin)
- Targets:
  - Native: default `std`
  - WASM: `wasm32-unknown-unknown` (feature `wasm` + `wasm-bindgen`)
- CI: build, test, clippy, fmt, optional wasm build

## Risks & Mitigations

- Nested workspace integration issues
  - Mitigation: parent workspace does not list this nested workspace as a member; use `path` deps only; feature-gate in parent
- API/JSON churn across teams
  - Mitigation: contract spec, SemVer, golden tests, `contract-change` PR label
- WASM size/perf concerns
  - Mitigation: minimal deps, feature gates, `wasm-opt` in CI (later)
- Catastrophic parse behavior on adversarial inputs
  - Mitigation: depth/node/size limits; single-pass scans; fuzz tests later

## Decision Log (contract alignment)

- Parser performs syntax recognition only; semantic validation is downstream (SSG/validator)
- Parser enforces limits (depth, nodes, input size); no partial AST on error
- AST JSON layout is part of compatibility surface; `#[serde(tag = "type")]` for `Block`
- Feature-gated integration in parent via `external_pml`

## Examples & Fixtures

- [ ] `examples/minimal.pml` demonstrating grid/card/artifact components
- [ ] `examples/errors/` with curated malformed inputs and expected errors
- [ ] `examples/repo_viewers/` gated by feature flag (optional in v1)
- [ ] `examples/errors/unterminated_component.pml`, `mismatched_close.pml`, `over_depth.pml`

## Documentation

- [ ] Keep `README.md` aligned with spec and current API surface
- [ ] Add module-level rustdoc with grammar highlights and examples
- [ ] Add `docs/` pages if needed for integration notes (parent workspace guidance)
- [ ] Add CONTRIBUTING.md and Developer Guide (build, test, run CLI/WASM)

## Workspace & Packaging

- [x] Introduce `crates/proofdown_ast` as first split-out crate
- [ ] Later: split `proofdown_validate` if code size/concerns warrant
- [ ] Maintain `[workspace.dependencies]` for shared versions (`anyhow`, `serde`, `thiserror`)
- [ ] Ensure the parent workspace does not list this nested workspace as a member; use `path` deps only

## Security & Safety

- [ ] No IO in parser; callers verify referenced resources via Index
- [ ] Strict whitelist; fail on unknowns
- [ ] Enforce safe globs; prohibit path traversal
- [ ] Bound resource usage with configurable limits

## Performance

- [ ] Avoid repeated substring allocations (prefer slicing)
- [ ] Minimize backtracking; prefer single-pass or bounded lookahead
- [ ] Track complexity hotspots (component scanning, attribute parsing)

## Integration with Parent Workspace

- [ ] Provide a minimal integration example for `provenance_ssg` using a `path` dependency
- [ ] Gate features in the parent until parser API stabilizes
- [ ] Provide a stable JSON AST schema for downstream consumers

## Open Questions / Decisions to Make

- [ ] Inline constructs (link macro, emphasis) representation in AST: separate inline AST vs. delegated to renderers?
- [ ] Should lists and code fences be parsed as first-class blocks in v1 or deferred?
- [ ] Attribute value normalization (e.g., trimming, case) policy per component
- [ ] Error message phrasing and error codes: stability contract

## Done (to date)

- [x] Nested workspace scaffold with root `Cargo.toml` and `resolver = "2"`
- [x] `crates/proofdown_parser` created with MVP `parse(&str) -> Result<Document>`
- [x] README expanded with spec overview, API, registry, and roadmap
- [x] `crates/proofdown_ast` created and wired into parser
- [x] `crates/proofdown_validate` scaffolded with `Limits` and `validate()`
- [x] `crates/proofdown_cli` scaffolded with `pml parse` and `pml validate`
- [x] `crates/proofdown_wasm` scaffolded with `wasm_parse()` behind feature flag
- [x] Example `examples/minimal.pml` and smoke test `crates/proofdown_parser/tests/parse_smoke.rs`
- [x] Default limit enforcement in parser (depth/nodes/input size)
- [x] Paragraph accumulation across non-empty lines
- [x] Attribute parsing preserves bare values exactly

