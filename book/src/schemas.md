# Schemas

Minimal JSON Schemas provide advisory shapes for common artifact `table` row kinds (e.g., `api`, `a11y`, `e2e`, `contracts`). They live under `.specs/schemas/` and are validated during CI with a local, deterministic checker.

- Use `proofdown_validate::schema_hint_for_kind(kind)` to suggest a schema filename.
- Schemas are informative guidance for authors and validators; rendering layers remain strict about unknown or unsafe content.
