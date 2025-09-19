# TODO — proofdown_ast

Scope: Data model and error types shared across the workspace. Keep AST JSON stable, ergonomically serializable, and well-documented. Avoid runtime logic here; this crate defines types, serde, and error model.

## Now / Next / Later

- [x] NOW: Error model polish and ergonomics
  - [x] Implement `std::fmt::Display` for `ParseError` (human-readable, concise)
  - [x] Implement `std::error::Error` for `ParseError` (manual)
  - [x] Stabilize `ErrorKind` variants and assign string codes via `ErrorKind::as_code()`
  - [x] Add `From<(line,col,msg)>` helpers for ergonomic construction
- [x] NEXT: AST JSON stability
  - [x] Create a JSON Schema for the AST (informative) for downstream consumers (`.specs/ast.schema.json`)
  - [x] Round-trip tests: serialize → parse (via serde) → equal (see `tests/serde_roundtrip.rs`)
  - [x] Backward-compat guide: documented SemVer/compatibility policy in `.docs/ast-compatibility.md`
- [ ] LATER: Extensibility & performance
  - [ ] Document extension points (new `Block` variants or `Component` fields) and semver process
  - [ ] Bench serde serialize/deserialize for large documents (ensure no pathological slowdowns)
  - [ ] Optional: binary format (e.g., postcard/bincode) benchmarks (no commitment yet)

## Tests & Quality

- [x] Round-trip serde JSON tests for `Document`, `Block`, `Component`, `Attr`, `ParseError`
- [x] Ensure stable field ordering and tagged enums remain consistent (serde-driven)
- [x] Negative tests: unknown `Block` tag deserialization fails clearly
- [ ] Optional: proptest for random AST instances to serialize/deserialize

## API & Compatibility

- [x] Public API docstrings for each type and field (rustdoc added)
- [x] Re-export convenience prelude (`proofdown_ast::prelude`)
- [x] SemVer policy documented in `.docs/ast-compatibility.md`

## Docs

- [x] Module-level rustdoc: AST overview, invariants, compatibility surface
- [x] Example coverage via tests; README points to types and JSON schema

## CI

- [x] Add crate-specific tests to CI (runs via workspace test job)
- [ ] Optional: coverage report job (later)

## Security & Safety

- [x] Pure data types only (no IO)
- [x] Serde-driven, no custom unsafe code

## Non-goals

- No parsing or validation logic in `proofdown_ast`.
- No renderer or file IO in this crate.
