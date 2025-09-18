# TODO

These items track gaps, refinements, and follow-ups discovered while modularizing CI and reviewing the parser/CLI. Items <= 50 LOC were addressed immediately.

- [x] Missing `map_table_row()` in `crates/proofdown_parser/src/lib.rs` used by table mapping.
  - Fixed by adding `map_table_row()` to collect inlines for each table cell (handles Paragraph wrappers).

- [x] Add modular CI workflows replacing monolithic `.github/workflows/ci.yml`.
  - New workflows: `lint.yml`, `test.yml`, `schema.yml`, `benches.yml`, `wasm.yml`, `fuzz-smoke.yml`, `cli-smoke.yml`, `mdbook.yml`, `pages.yml`, `markdown-parse.yml`, `grammar.yml`.
  - Legacy `ci.yml` changed to on: workflow_dispatch as a stub.

- [x] Add repository-wide Markdown parsing check in CI.
  - `markdown-parse.yml` builds the CLI once and parses all tracked `*.md` and `*.pml` files to ensure Markdown support coverage.

- [x] Improve error positions for attribute parsing in components.
  - Implemented by threading absolute index/base into `parse_attrs(full, src, abs_base)` in `crates/proofdown_parser/src/lib.rs` so `mk_err`/`pos_to_line_col` can compute accurate positions. Added `attr_error_reports_position` test.

- [x] Extend Markdown coverage tests.
  - Added explicit tests for autolink, GFM task list checkboxes, and table alignments in `tests/markdown_coverage.rs`. Adjusted list mapping to robustly detect task items across comrak versions.

- [x] Validate list start indices for ordered lists.
  - Added `ordered_list_with_non_one_start_has_start_field` test.

- [ ] Consider optional footnotes support (if desired) gated by a feature flag.
  - Out of current scope; document decision in the spec if added.

- [x] CLI UX: add `--version` integration test and a `--help` smoke test.
  - Already present; expanded CLI tests for stdin handling and human/JSON error paths with exit codes.

- [x] CI: consider a matrix (Linux, macOS, Windows) for `test.yml` if cross-platform support is required.
  - Switched `test.yml` to an OS matrix.

