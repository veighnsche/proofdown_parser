# SSG Contract (Parser ↔ SSG)

This chapter summarizes the versioned contract at `.specs/10_contract_with_provenance_ssg.md`. The parser performs syntax recognition only; the SSG (or validator) handles semantics and rendering.

Stable API and types:

```rust
pub struct Document { pub blocks: Vec<Block> }
#[serde(tag = "type")]
pub enum Block {
    Heading { level: u8, inlines: Vec<Inline> },
    Paragraph { inlines: Vec<Inline> },
    BlockQuote { children: Vec<Block> },
    ThematicBreak,
    CodeBlock { info: String, text: String },
    List { kind: ListKind, start: Option<u64>, tight: bool, items: Vec<ListItem> },
    Table { align: Vec<TableAlign>, header: Option<TableRow>, rows: Vec<TableRow> },
    Component(Component)
}

pub enum ListKind { Bullet, Ordered }
pub struct ListItem { pub children: Vec<Block>, pub task: Option<bool> }
pub enum TableAlign { None, Left, Center, Right }
pub struct TableRow { pub cells: Vec<Vec<Inline>> }

#[serde(tag = "type")]
pub enum Inline {
    Text { text: String }, Emph { children: Vec<Inline> }, Strong { children: Vec<Inline> },
    Strikethrough { children: Vec<Inline> }, Code { text: String }, SoftBreak, HardBreak,
    Link { url: String, title: Option<String>, children: Vec<Inline> },
    Image { url: String, title: Option<String>, alt: String }
}

pub struct Component { pub name: String, pub attrs: Vec<Attr>, pub children: Vec<Block>, pub self_closing: bool }
pub struct Attr { pub key: String, pub value: String }

pub fn parse(input: &str) -> Result<Document, ParseError>;
```

Error model and limits:

```rust
pub enum ErrorKind { Syntax, LimitExceeded }
pub struct ParseError { pub line: usize, pub col: usize, pub kind: ErrorKind, pub msg: String }
// Limits: depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB
```

Determinism and golden tests ensure byte-for-byte stability. Raw HTML blocks/inline are dropped for safety; use components or Markdown constructs instead. See the spec file for the canonical text.
