# TODO — proofdown_ast

Scope: Data model and error types shared across the workspace. Keep AST JSON stable, ergonomically serializable, and well-documented. Avoid runtime logic here; this crate defines types, serde, and error model.

## Now / Next / Later

- [ ] NOW: Error model polish and ergonomics
  - [ ] Implement `std::fmt::Display` for `ParseError` (human-readable, concise)
  - [ ] Implement `std::error::Error` for `ParseError` (manual or via `thiserror`)
  - [ ] Stabilize `ErrorKind` variants and assign string codes (e.g., `Syntax`, `LimitExceeded`)
  - [ ] Add `From<(line,col,msg)>` helpers if useful for parser construction
- [ ] NEXT: AST JSON stability
  - [ ] Create a JSON Schema for the AST (informative) for downstream consumers
  - [ ] Round-trip tests: serialize → parse (via serde) → equal
  - [ ] Backward-compat guide: how to evolve AST without breaking semver/contracts
- [ ] LATER: Extensibility & performance
  - [ ] Document extension points (new `Block` variants or `Component` fields) and semver process
  - [ ] Bench serde serialize/deserialize for large documents (ensure no pathological slowdowns)
  - [ ] Optional: binary format (e.g., postcard/bincode) benchmarks (no commitment yet)

## Tests & Quality

- [ ] Round-trip serde JSON tests for `Document`, `Block`, `Component`, `Attr`, `ParseError`
- [ ] Ensure stable field ordering and tagged enums remain consistent
- [ ] Negative tests: unknown `Block` tag deserialization fails clearly
- [ ] Optional: proptest for random AST instances to serialize/deserialize

## API & Compatibility

- [ ] Public API docstrings for each type and field (rustdoc)
- [ ] Re-export convenience prelude (optional)
- [ ] SemVer policy documented in this file / CHANGELOG

## Docs

- [ ] Module-level rustdoc: AST overview, invariants, compatibility surface
- [ ] Example snippets showing how to serialize/deserialize AST

## CI

- [ ] Add crate-specific tests to CI (already runs via workspace)
- [ ] Optional: coverage report job (later)

## Security & Safety

- [x] Pure data types only (no IO)
- [x] Serde-driven, no custom unsafe code

## Non-goals

- No parsing or validation logic in `proofdown_ast`.
- No renderer or file IO in this crate.
