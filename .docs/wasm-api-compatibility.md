# WASM API Compatibility Policy

This document captures the stability guarantees for the Proofdown WASM surface (`proofdown_wasm`).

- Stable surface
  - `wasm_parse(input: string) -> string` returns a JSON string with shape `{ ok: boolean, doc?: Document, err?: { code: string, msg: string, line?: number, col?: number } }`.
  - `wasm_parse_with_limits(input: string, maxDepth: number, maxNodes: number, maxInputBytes: number) -> string` is provided for environments that require explicit resource bounding.
  - JSON field names are stable across patch/minor. New fields may be added additively.

- Error codes
  - Error codes mirror the Rust `ErrorKind` string codes (e.g., `Syntax`, `LimitExceeded`). New codes may be added additively.

- Versioning
  - We follow SemVer for the NPM package and crate. Breaking changes to the JS JSON shape are reserved for major versions.

- Testing
  - Native tests compare WASM JSON output vs. native parser for parity.
  - Browser tests ensure the function executes under `wasm-bindgen-test`.

- Notes
  - For Node usage, build with `wasm-pack --target nodejs` and import the generated JS glue.
