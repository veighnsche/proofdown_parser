# Grammar (v1)

The v1 grammar is Markdown + Components. The parser recognizes full CommonMark block/inline structure plus a small, HTML-like component system. Semantics (component whitelist, attribute bounds, artifact policies) are enforced downstream by the validator/SSG.

Supported Markdown blocks

- Headings `#..####` (ATX), `level: 1..4`
- Paragraphs
- Block quotes with nested blocks
- Thematic breaks (`---`, `***`, `___`)
- Code blocks (fenced/indented) with optional info string
- Lists (unordered/ordered), with `start`, `tight`, and GFM task items (`[ ]`/`[x]`)
- Tables (GFM): header row optional; alignments per column

Supported Markdown inlines

- Text, emphasis, strong, strikethrough (GFM)
- Inline code
- Soft/hard line breaks
- Links (url + optional title), Images (url + optional title + alt from content)

Components

- Start tag: `<name key="value" key2=value2>`
- End tag: `</name>` (unless self-closing as `<name ... />`)
- Attributes: quoted or bare; preserved exactly as strings in the AST
- Children: zero or more blocks; components may contain full Markdown

Newlines & safety

- UTF-8 input; CRLF/CR are normalized to `\n` during scanning
- Raw HTML blocks/inlines are dropped by the parser for safety (use components instead)

See also: `.specs/01_proofdown_language_v1.md` for extended notes and examples.
