# TODO — proofdown_wasm

Scope: WebAssembly surface for the Proofdown parser (and optionally validator later). Deliver a minimal, safe, and small WASM module with stable JS APIs, a demo page, and CI build.

## Now / Next / Later

- [x] NOW: Solidify WASM surface and demo
  - [x] Expose `wasm_parse(input: &str) -> String` via `wasm-bindgen` returning `{ ok, doc|err }` JSON
  - [x] JS types: implicit via JSON shape `{ ok, err: { code, msg, line, col } }`
  - [x] Demo page: wired up real `wasm_bindgen` glue via `wasm-pack` output in `examples/wasm/pkg/`
  - [ ] Node test: verify `wasm_parse` works under Node (no DOM), if practical
  - [x] Error mapping: ensure `ParseError { line, col, kind, msg }` maps to a JS object (stable keys)
  - [x] Build script: `wasm-pack build` documented in demo; `wasm.yml` builds and uploads artifact
  - [x] CI: build wasm with `--features wasm` and upload artifact
- [ ] NEXT: Size and performance
  - [x] Feature gates on dependencies; avoid pulling in non-essential crates (deps minimal; no additional gating required)
  - [x] Link-time size opts: `opt-level = 's'` and `lto = true` at workspace root profile
  - [x] Integrate `wasm-opt` (binaryen) in CI for release builds
  - [x] Memory behavior: document expected peak sizes and recommend limits (see README)
- [ ] LATER: Packaging and ecosystem
  - [ ] Publish npm package via `wasm-pack` (ESM + types)
  - [ ] Bundler guides: Vite/Rollup/Webpack examples; dynamic import and caching best practices
  - [ ] Web Worker example: run parse off main thread (postMessage contract)
  - [ ] Optional validator-in-WASM (future): small rule subset; keep separate crate if size grows

## API & Compatibility

- [x] Stable API: keep `wasm_parse` signature and JSON fields stable
- [x] SemVer policy: document what constitutes a breaking change in WASM JS API (`.docs/wasm-api-compatibility.md`)
- [x] Error codes: mirror `ErrorKind` string codes for front-end routing

## Tests

- [x] wasm-bindgen test (headless) for `wasm_parse` success cases (browser)
- [x] Golden parity: compare `wasm_parse` JSON with native `parse` JSON for shared fixtures (native test)
- [x] CRLF normalization parity tests (native test)
- [x] Property-like test (small corpus) to ensure no panics on odd inputs (native proptest)

## Demo

- [x] Replace placeholder `examples/wasm/index.html` with working wasm-bindgen glue
- [x] Add a small UI: textarea input, parse button, JSON output pretty-printed
- [x] Add example dropdown with known fixtures

## CI

- [x] Build wasm in CI (baseline)
- [x] Add wasm-pack job; publish artifact (PR preview)
- [ ] Optional: compare wasm vs native JSON outputs for a sample fixture in CI

## Docs

- [x] README section: how to build, load, and call `wasm_parse`
- [ ] TS typings example; browser and Node usage snippets
- [x] Size/performance notes and recommendations

## Security & Safety

- [x] No network calls; pure compute
- [x] No code execution; result is JSON data only
- [x] Enforce input size limits even in WASM (document defaults; provide `wasm_parse_with_limits`)

## Non-goals

- No rendering logic in this crate (UI demo is for testing only)
- No artifact fetching/resolution in WASM (caller provides content)
