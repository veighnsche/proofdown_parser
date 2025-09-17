# TODO — proofdown_wasm

Scope: WebAssembly surface for the Proofdown parser (and optionally validator later). Deliver a minimal, safe, and small WASM module with stable JS APIs, a demo page, and CI build.

## Now / Next / Later

- [ ] NOW: Solidify WASM surface and demo
  - [ ] Expose `wasm_parse(input: &str) -> String` via `wasm-bindgen` returning `{ ok, doc|err }` JSON
  - [ ] JS types: define a small TS interface for `{ ok: boolean, doc?: Document, err?: ParseError }`
  - [ ] Demo page: wire up real `wasm_bindgen` glue instead of placeholder; load `.wasm` and call `wasm_parse`
  - [ ] Node test: verify `wasm_parse` works under Node (no DOM), if practical
  - [ ] Error mapping: ensure `ParseError { line, col, kind, msg }` maps to a JS object (stable keys)
  - [ ] Build script: `wasm-pack build` or `cargo build --target wasm32-unknown-unknown` + glue, document both
  - [ ] CI: build wasm with `--features wasm` and upload artifact (already partially in place)
- [ ] NEXT: Size and performance
  - [ ] Feature gates on dependencies; avoid pulling in non-essential crates
  - [ ] Link-time size opts: `opt-level = 's'` and `lto = true` in a `Cargo.toml` profile (wasm profile)
  - [ ] Integrate `wasm-opt` (binaryen) in CI for release builds (document how to install)
  - [ ] Memory behavior: avoid large allocations; document expected peak sizes
- [ ] LATER: Packaging and ecosystem
  - [ ] Publish npm package via `wasm-pack` (ESM + types)
  - [ ] Bundler guides: Vite/Rollup/Webpack examples; dynamic import and caching best practices
  - [ ] Web Worker example: run parse off main thread (postMessage contract)
  - [ ] Optional validator-in-WASM (future): small rule subset; keep separate crate if size grows

## API & Compatibility

- [ ] Stable API: keep `wasm_parse` signature and JSON fields stable
- [ ] SemVer policy: document what constitutes a breaking change in WASM JS API
- [ ] Error codes: mirror `ErrorKind` string codes for front-end routing

## Tests

- [ ] wasm-bindgen test (headless) for `wasm_parse` success and failure cases
- [ ] Golden parity: compare `wasm_parse` JSON with native `parse` JSON for shared fixtures
- [ ] CRLF normalization parity tests
- [ ] Property-like test (small corpus) to ensure no panics on odd inputs

## Demo

- [ ] Replace placeholder `examples/wasm/index.html` with working wasm-bindgen glue
- [ ] Add a small UI: textarea input, parse button, JSON output pretty-printed
- [ ] Add example dropdown with known fixtures

## CI

- [x] Build wasm in CI (baseline)
- [ ] Add wasm-bindgen-based build or wasm-pack job; publish artifact (PR preview)
- [ ] Optional: compare wasm vs native JSON outputs for a sample fixture in CI

## Docs

- [ ] README section: how to build, load, and call `wasm_parse`
- [ ] TS typings example; browser and Node usage snippets
- [ ] Size/performance notes and recommendations

## Security & Safety

- [x] No network calls; pure compute
- [x] No code execution; result is JSON data only
- [ ] Enforce input size limits even in WASM (document defaults and overrides if added)

## Non-goals

- No rendering logic in this crate (UI demo is for testing only)
- No artifact fetching/resolution in WASM (caller provides content)
