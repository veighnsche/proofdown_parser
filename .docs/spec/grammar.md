# Grammar (Normative)

- Block-level constructs
  - Headings: ATX (`#` .. `######`) with inline content.
  - Paragraphs: sequences of inline elements.
  - BlockQuote: `>`-prefixed blocks.
  - ThematicBreak: `---`, `***`, `___`.
  - CodeBlock: fenced (``` or ~~~) with optional info string.
  - List: bullet (`-`, `*`, `+`) and ordered (`1.` etc.), optional `start`, `tight` flag.
  - Table: GFM tables; alignments `Left|Center|Right`.
  - Component: `<name key=value ...> ... </name>` or self-closing `<name ... />`.

- Inline constructs
  - Text, Emph (`*`/`_`), Strong (`**`/`__`), Strikethrough (`~~`), Code (`` `code` ``), Soft/Hard breaks, Links, Images.

- Safety & exclusions
  - Raw HTML blocks are dropped.
  - Smart punctuation disabled by default.
  - GFM extensions enabled: tables, autolink, strikethrough, task lists.
  - Footnotes are feature-gated.
