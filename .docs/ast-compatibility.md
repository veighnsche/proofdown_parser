# Proofdown AST Compatibility Policy

This document describes how the `proofdown_ast` JSON remains stable across versions and how we evolve it without breaking consumers.

- Stability goals
  - The JSON shape (tagged enums, field names) is stable across patch/minor versions.
  - Additive changes are allowed: adding new enum variants or optional fields.
  - Removals or renames are reserved for major versions.

- Versioning
  - We follow SemVer. Any breaking change to the JSON encoding bumps the MAJOR version.

- Guidelines for change
  - Prefer adding optional fields over changing existing ones.
  - When adding a `Block` or `Inline` variant, ensure default deserializers in downstream consumers are resilient (e.g., `#[serde(other)]` patterns if applicable).
  - Keep `ErrorKind` string codes stable; only add new codes.

- Testing
  - Round-trip tests ensure serde stability (`tests/serde_roundtrip.rs`).
  - Golden JSON files in parser tests guard against accidental changes.

- JSON Schema (informative)
  - See `.specs/ast.schema.json` for an informative schema capturing the current shape.
