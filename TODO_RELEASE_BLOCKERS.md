# Production Readiness — Release Blockers (Parser/Validator/CLI)

This checklist enumerates gaps to close before calling the workspace production ready. Items are either already implemented now or addressed below.

- [x] Parser newline normalization test (CRLF/CR -> LF) with AST equality
- [x] CLI `--help` and `--version` behavior covered by tests
- [x] CLI validation error JSON path and exit code semantics covered by tests
- [x] Validator: JSON Pointer, columns, captions, and `artifact.text` bounds tests
- [x] Validator: `schema_hint_for_kind` mapping covered by tests
- [x] Property test: parser never panics on arbitrary UTF-8 inputs (bounded)
- [x] Fuzz harness present and CI smoke job added
- [x] Benchmarks compile and run (not in PR CI)
- [x] Integration example compiles and exits with clear codes on errors
- [x] CI: fmt/clippy/workspace tests/schemas, wasm build, fuzz smoke

Notes

- WASM demo page is a minimal placeholder; wasm-bindgen glue for direct `wasm_parse` is intentionally out-of-scope for now and documented in examples.
- Schema checks are basic sanity; full JSON Schema validation is deferred until runtime resolvers are available.
