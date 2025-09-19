# Components & Attributes (Normative)

Components are XML-like tags allowed in block context:

- Opening tag: `<name key=value key2="value two">`
- Self-closing: `<name ... />`
- Closing tag: `</name>`

Parsing rules:

- `name`: `[A-Za-z][A-Za-z0-9_-]*`
- Attributes are `key=value` with:
  - key: `[A-Za-z][A-Za-z0-9_-]*`
  - value: either bare (no spaces/quotes/`>`), or double-quoted.
- Whitespace is allowed between attributes.
- Self-closing requires a single space before `/>` if any attributes are present.

AST mapping:

- Becomes `Block::Component(Component)` with fields:
  - `name: String`
  - `attrs: Vec<Attr>` each `{ key: String, value: String }`
  - `children: Vec<Block>`
  - `self_closing: bool`

Validation:

- Unknown components and attributes are rejected by `proofdown_validate`.
- Component-specific attribute types/bounds are enforced by the validator, not the parser.
