# Changelog — proofdown_parser

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Validator (v2) checks: `artifact.json.json_pointer`, `artifact.image.caption`, `artifact.table.caption`, `artifact.table.columns` (keys/JSON Pointers), `artifact.text.max_lines` (1..500).
- CLI flags: `--json`, `--pretty`, and limits flags for parse/validate (`--limits.depth`, `--limits.nodes`, `--limits.input-size`).
- v2 fixtures and goldens expanded (visual+a11y, performance, API, contracts, data_quality, fuzzing, iac, mobile, snapshots, db_migrations, mutation).
- Golden tests for all `v2_*.pml` fixtures; case tests for mixed content and attribute parsing; limits tests.
- Benchmarks with Criterion for minimal and v2 fixtures.
- CI workflow: fmt, clippy, workspace tests, schema checks.
- Developer Guide with commands and workflow; schemas README.

### Changed

- Parser module docs added to `crates/proofdown_parser/src/lib.rs`.
- README updated with quick links to specs, authoring guide, artifacts, and schemas.

### Fixed

- Basic schema sanity checks script (`schema_check` crate) to catch malformed files early.
