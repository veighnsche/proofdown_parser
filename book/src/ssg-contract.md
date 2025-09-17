# SSG Contract (Parser ↔ SSG)

This chapter summarizes the versioned contract at `.specs/10_contract_with_provenance_ssg.md`. The parser performs syntax recognition only; the SSG (or validator) handles semantics and rendering.

Stable API and types:

```rust
pub struct Document { pub blocks: Vec<Block> }
#[serde(tag = "type")]
pub enum Block { Heading { level: u8, text: String }, Paragraph { text: String }, Component(Component) }
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

Determinism and golden tests ensure byte-for-byte stability. See the spec file for the canonical text.
