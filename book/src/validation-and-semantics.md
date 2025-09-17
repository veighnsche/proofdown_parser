# Validation and Semantics

Validation enforces a structural whitelist and attribute bounds over the parsed AST. Unknown components/attributes are errors at validation time. Examples:

- Structural: `grid(cols=1..6, gap=0..64)`, `section(title)`, `card(title)`
- Artifacts: 
  - `artifact.json(id, collapsed?, depth=0..8, json_pointer?)`
  - `artifact.table(id, caption?, columns?, kind?)`
  - `artifact.image(id, alt, max_height=128..2048, caption?)`
  - `artifact.text(id, max_lines=1..500, caption?)`
- Advisory schema hints for `artifact.table.kind` map to filenames (see schemas chapter).
- Errors are structured with codes: `UnknownComponent`, `UnknownAttribute`, `MissingAttribute`, `AttrType`, `AttrBound`, `DepthExceeded`, `NodeLimit`.
