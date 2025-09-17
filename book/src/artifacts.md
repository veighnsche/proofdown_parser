# Artifact Viewers

Artifact viewers present CI/testing evidence from content-addressed artifacts. The parser is syntax-only; bounds and integrity checks are downstream.

- `artifact.summary(id)` — Show rollups or key counts.
- `artifact.table(id, caption?, columns?, kind?)`
  - `columns`: comma-separated; tokens are simple keys or RFC 6901 JSON Pointers (e.g., `/a/b`, `foo.bar`).
  - `kind`: advisory mapping to schema filename; use validator helper.
- `artifact.json(id, collapsed?, depth=0..8, json_pointer?)`
  - `json_pointer`: RFC 6901 (syntax validated); static projection.
- `artifact.markdown(id)` — Rendered markdown snapshot (safe subset downstream).
- `artifact.image(id, alt, max_height=128..2048, caption?)` — Always include `alt`.
- `artifact.link(id, download?, title?)` — Links to heavy/interactive assets.
- `artifact.text(id, max_lines=1..500, caption?)` — Short plaintext snippets.

Accessibility and performance:
- Large JSON collapsed by default (use small `depth`).
- Always provide `alt` text and captions when useful.
