# Language Inventory

This is a normative inventory of Proofdown constructs, their attributes, and constraints. It aggregates v1 grammar and v2 additive semantics. Validation of bounds happens downstream (validator/SSG). The parser remains syntax-only.

## Blocks

- Heading
  - Form: `#`..`####` followed by a single space and the title.
  - Levels: 1..4 (higher counts are treated as paragraphs by parsers aligned to current spec).
  - Title: trimmed of surrounding spaces; empty titles are treated as paragraphs.
- Paragraph
  - Consecutive non-empty lines until a blank line or a heading/component begins.
- Component
  - HTML-like tags: `<name key="value" key2=value2>`; `</name>` closes; `<name ... />` self-closing.
  - Attributes: quoted or bare; preserved exactly as strings in AST.

## Structural Components

- `grid(cols, gap?)`
  - Purpose: layout container for cards/sections.
  - Attributes (validator bounds): `cols` in 1..6 required; `gap` in 0..64 optional.
- `section(title)`
  - Title required.
- `card(title)`
  - Title required; contains child blocks.

## Artifact Viewers (syntax recognized; validation enforces bounds)

- `artifact.summary(id)`
- `artifact.table(id, caption?, columns?, kind?)`
  - `columns`: comma-separated keys or RFC 6901 JSON Pointers; validated for syntax.
  - `kind`: optional hint for schema; advisory.
- `artifact.json(id, collapsed?, depth=0..8, json_pointer?)`
  - `json_pointer`: RFC 6901 string; static projection.
- `artifact.markdown(id)`
- `artifact.image(id, alt, max_height=128..2048, caption?)`
- `artifact.link(id, download?, title?)`
- `artifact.text(id, max_lines=1..500, caption?)` (v2)

## Limits (enforced)

- Default: `max_depth=16`, `max_nodes=50k`, `max_input_bytes=1MiB`.
- Determinism: same input ⇒ same AST byte-for-byte under serde JSON.
- Safety: no I/O in parser; validator enforces whitelist/attribute bounds; renderer escapes and enforces viewer limits.
