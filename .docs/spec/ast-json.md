# AST JSON (Normative)

The AST JSON is the serialized form of `proofdown_ast::Document`.

- Root object: `Document` with field `blocks: Block[]`.
- `Block` is a tagged enum with `type`:
  - `Heading { level: number, inlines: Inline[] }`
  - `Paragraph { inlines: Inline[] }`
  - `BlockQuote { children: Block[] }`
  - `ThematicBreak {}` (encoded as `{ "type": "ThematicBreak" }`)
  - `CodeBlock { info: string, text: string }`
  - `List { kind: "Bullet"|"Ordered", start?: number, tight: boolean, items: ListItem[] }`
  - `Table { align: ("None"|"Left"|"Center"|"Right")[], header?: TableRow, rows: TableRow[] }`
  - `Component { name: string, attrs: Attr[], children: Block[], self_closing: boolean }`

- `Inline` is a tagged enum with `type`:
  - `Text { text: string }`
  - `Emph { children: Inline[] }`
  - `Strong { children: Inline[] }`
  - `Strikethrough { children: Inline[] }`
  - `Code { text: string }`
  - `SoftBreak {}` / `HardBreak {}`
  - `Link { url: string, title?: string, children: Inline[] }`
  - `Image { url: string, title?: string, alt: string }`

- `ListItem { children: Block[], task?: boolean }`
- `TableRow { cells: Inline[][] }`
- `Attr { key: string, value: string }`

Stability rules:
- Only additive changes in minor releases (new enum variants, new optional fields) are allowed.
- Renames/removals are reserved for major releases.
- Error codes are stable: `Syntax`, `LimitExceeded`.
