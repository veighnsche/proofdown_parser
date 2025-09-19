# Behavior & Safety (Normative)

- Markdown engine
  - Based on comrak `=0.21.0` (pinned). GFM features enabled: tables, autolink, strikethrough, task lists.
  - Footnotes are feature-gated via `proofdown_parser` Cargo feature `footnotes`.

- Safety
  - Raw HTML blocks are dropped. No HTML passthrough.
  - Smart punctuation is disabled by default.

- Limits (resource bounding)
  - Defaults (may change between minor versions): `max_depth=64`, `max_nodes=50_000`, `max_input_bytes=1 MiB`.
  - Limits are enforced in native and exposed to WASM via `wasm_parse_with_limits`.
  - On limit exceed, the parser returns `ErrorKind::LimitExceeded`.

- Error model
  - Parser errors contain `line`, `col` (1-based), `kind` with stable codes: `Syntax`, `LimitExceeded`.
  - Error formatting and JSON mapping are stable.

- Determinism
  - Given identical input and options, the AST JSON is deterministic.
  - Line ending normalization (LF vs CRLF) yields identical JSON outputs.
