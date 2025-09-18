# Additive Semantics (v2)

v2 is additive and backward compatible with v1: it introduces semantics and viewer attributes on top of the Markdown + Components grammar. No grammar changes in v2. New attributes/viewers are validated downstream. Key additions:

- `artifact.json`: `json_pointer` (RFC 6901) for static projection; `depth` bound; `collapsed` flag.
- `artifact.table`: `columns` (keys or JSON Pointers); optional `caption`; optional `kind` hint for schema.
- `artifact.image`: optional `caption`; `alt` remains required; `max_height` bounds.
- `artifact.text`: new viewer with `max_lines=1..500` and optional `caption`.

Validation and rendering layers enforce bounds, unknown attrs, and advisory schema hints.
