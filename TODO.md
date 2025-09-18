# TODO

These items track gaps, refinements, and follow-ups discovered while modularizing CI and reviewing the parser/CLI. Items <= 50 LOC were addressed immediately.

- [x] Missing `map_table_row()` in `crates/proofdown_parser/src/lib.rs` used by table mapping.
  - Fixed by adding `map_table_row()` to collect inlines for each table cell (handles Paragraph wrappers).

- [x] Add modular CI workflows replacing monolithic `.github/workflows/ci.yml`.
  - New workflows: `lint.yml`, `test.yml`, `schema.yml`, `benches.yml`, `wasm.yml`, `fuzz-smoke.yml`, `cli-smoke.yml`, `mdbook.yml`, `pages.yml`, `markdown-parse.yml`, `grammar.yml`.
  - Legacy `ci.yml` changed to on: workflow_dispatch as a stub.

- [x] Add repository-wide Markdown parsing check in CI.
  - `markdown-parse.yml` builds the CLI once and parses all tracked `*.md` and `*.pml` files to ensure Markdown support coverage.

- [ ] Improve error positions for attribute parsing in components.
  - Currently `parse_attrs()` reports line/col as `(1,1)` on errors because it lacks source offset context.
  - Proposal: thread the absolute index/base into `parse_attrs()` so `mk_err`/`pos_to_line_col` can compute accurate positions.

- [ ] Extend Markdown coverage tests.
  - Add explicit tests for: autolink, GFM task list checkboxes, and table alignments in `tests/markdown_coverage.rs`.

- [ ] Validate list start indices for ordered lists.
  - Ensure `start` handling (non-1 values) is covered by a unit test (there is mapping in `map_block`, but no explicit test).

- [ ] Consider optional footnotes support (if desired) gated by a feature flag.
  - Out of current scope; document decision in the spec if added.

- [ ] CLI UX: add `--version` integration test and a `--help` smoke test.
  - Minor improvements to `tests/cli.rs`.

- [ ] CI: consider a matrix (Linux, macOS, Windows) for `test.yml` if cross-platform support is required.

