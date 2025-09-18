# Language Inventory

This is a normative inventory of Proofdown constructs, their attributes, and constraints. It aggregates v1 grammar and v2 additive semantics. Validation of bounds happens downstream (validator/SSG). The parser remains syntax-only.

## Blocks

- Markdown (CommonMark + selected GFM)
  - Headings (ATX) `#..####` levels 1..4
  - Paragraphs
  - Block quotes (arbitrary nested blocks)
  - Thematic breaks (`---`, `***`, `___`)
  - Code blocks (fenced/indented) with optional info string
  - Lists (unordered/ordered) with `start` (ordered), `tight`, and GFM task items (`[ ]`/`[x]`)
  - Tables (GFM) with optional header row and per-column alignment
- Components
  - HTML-like tags: `<name key="value" key2=value2>`; `</name>` closes; `<name ... />` self-closing.
  - Attributes: quoted or bare; preserved exactly as strings in AST.
  - Children: zero or more blocks; components MAY contain full Markdown.

## Markdown inlines

- Text
- Emphasis, Strong, Strikethrough (GFM)
- Inline code
- Soft/Hard line breaks
- Links (url + optional title), Images (url + optional title; alt from inline text)

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
- Safety: no I/O in parser; raw HTML blocks/inline are dropped; validator enforces whitelist/attribute bounds; renderer escapes and enforces viewer limits.
