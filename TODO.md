# TODO — proofdown_parser

Last updated: 2025-09-17

This document tracks implementation tasks, milestones, and decisions for the Proofdown parser submodule. It is derived from `.specs/00_proofdown_parser.md`, `.plans/00_workspace_plan.md`, and the current code under `crates/proofdown_parser/`.

## Now / Next / Later

- [ ] NOW: Solidify MVP parser behavior to match v1 scope
  - [ ] Ensure headings `#..####` parse robustly with trailing spaces and empty titles guarded
  - [ ] Improve paragraph accumulation across lines until blank or component start
  - [ ] Harden component parsing of self-closing tags `<name ... />` and closing `</name>` detection (mismatched names)
  - [ ] Attribute parsing: support both quoted and bare values; reject malformed key/value pairs with structured errors
  - [ ] Introduce depth and node counting during parse to prepare for limits enforcement
- [ ] NEXT: Add validation pass and structured error model
  - [ ] Define `ErrorKind` and `ParseError { line, col, kind, msg }`
  - [ ] Enforce component whitelist and attribute schemas from the spec
  - [ ] Enforce configurable limits: depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB (with defaults)
  - [ ] Establish stable error messages and codes for golden tests
- [ ] LATER: Developer ergonomics and platform surfaces (crates scaffolded)
  - [ ] CLI: extend commands and flags; polish UX and errors (`proofdown_cli` exists)
  - [ ] WASM: refine bindings and add example; CI wasm-opt (`proofdown_wasm` exists)
  - [ ] Link macro parser (v1.1) and expanded components; Includes support (v1.2)

## Parser Work (crates/proofdown_parser)

- [ ] Normalize newlines to `\n` (input is UTF-8; guard CRLF gracefully)
- [ ] Whitespace handling around tags and inside text blocks
- [ ] Nested component handling: error on `UnexpectedClose` and `UnterminatedTag` with location
- [ ] Mixed content inside components: paragraphs and headings are supported; consider list/code fences (spec allows, but v1 minimum can defer)
- [ ] Improve scanning to avoid quadratic behavior on large inputs
- [ ] Provide a public, non-panicking API surface; remove ad-hoc `anyhow::bail!` in favor of typed errors
- [ ] Expose configurable parse options (future): max depth, max nodes, input size, component registry gate

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
- [ ] Unknown component/attribute MUST error (validation stage)
- [ ] Limits enforcement with structured errors (`DepthExceeded`, `SizeLimit`)

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

## Examples & Fixtures

- [ ] `examples/minimal.pml` demonstrating grid/card/artifact components
- [ ] `examples/errors/` with curated malformed inputs and expected errors
- [ ] `examples/repo_viewers/` gated by feature flag (optional in v1)

## Documentation

- [ ] Keep `README.md` aligned with spec and current API surface
- [ ] Add module-level rustdoc with grammar highlights and examples
- [ ] Add `docs/` pages if needed for integration notes (parent workspace guidance)

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

