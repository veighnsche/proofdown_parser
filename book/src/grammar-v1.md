# Grammar (v1)

The v1 grammar is intentionally small and LL-friendly. The parser recognizes blocks and components syntactically and leaves semantics to the validator.

- Headings
  - Lines beginning with `#`..`####` followed by a single space are headings (levels 1–4).
  - Heading text is trimmed; empty titles fallback to paragraphs.
- Paragraphs
  - Non-empty lines accumulated until a blank or a new heading/component start (`<`).
- Components
  - Start tag: `<name (key=("value"|bare))* (/>|>)` where `name` is a case-sensitive string.
  - End tag: `</name>` closes a component.
  - Self-closing: `<name ... />` has no children.
  - Attributes preserve original string values; quoting or bare tokens.
- Whitespace & Newlines
  - Input encoding is UTF-8; CRLF and CR are treated as `\n` for scanning consistency.
  - Parsers SHOULD avoid collapsing internal spaces within paragraph text.

See also: `.specs/01_proofdown_language_v1.md` for extended notes and examples.
