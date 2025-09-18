//! Typed AST and shared error types for Proofdown.
//!
//! This crate intentionally contains only data structures with `serde` derives
//! and small convenience helpers. No IO or runtime logic lives here.
use serde::{Deserialize, Serialize};

/// Root document consisting of a sequence of blocks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    /// Top-level block sequence
    pub blocks: Vec<Block>,
}

/// Convenience re-exports for downstream consumers.
pub mod prelude {
    pub use crate::{
        find_attr, Attr, Block, Component, Document, ErrorKind, Inline, ListItem, ListKind,
        ParseError, TableAlign, TableRow,
    };
}

 

/// Table column alignment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TableAlign {
    None,
    Left,
    Center,
    Right,
}

/// Table row consisting of inline cell contents per column
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRow {
    /// Per-column inline content for the row
    pub cells: Vec<Vec<Inline>>,
}

/// Block-level nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Block {
    /// ATX-style heading with CommonMark inlines
    Heading {
        level: u8,
        inlines: Vec<Inline>,
    },
    /// Paragraph with CommonMark inlines
    Paragraph {
        inlines: Vec<Inline>,
    },
    /// Block quote containing nested blocks
    BlockQuote {
        children: Vec<Block>,
    },
    /// Thematic break (horizontal rule)
    ThematicBreak,
    /// Fenced/indented code block with optional info string
    CodeBlock {
        info: String,
        text: String,
    },
    /// List of items (ordered or bullet). `tight` indicates tight vs loose formatting.
    List {
        kind: ListKind,
        start: Option<u64>,
        tight: bool,
        items: Vec<ListItem>,
    },
    /// GitHub-flavored Markdown table (inlines only inside cells)
    Table {
        align: Vec<TableAlign>,
        header: Option<TableRow>,
        rows: Vec<TableRow>,
    },
    Component(Component),
}

/// List kind
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListKind {
    Bullet,
    Ordered,
}

/// List item containing nested blocks and optional GFM task marker
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListItem {
    /// Child blocks comprising the list item's content
    pub children: Vec<Block>,
    /// GFM task checkbox state: Some(true)=checked, Some(false)=unchecked, None=not a task item
    pub task: Option<bool>,
}

/// Inline-level CommonMark nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Inline {
    Text {
        text: String,
    },
    Emph {
        children: Vec<Inline>,
    },
    Strong {
        children: Vec<Inline>,
    },
    Strikethrough {
        children: Vec<Inline>,
    },
    Code {
        text: String,
    },
    SoftBreak,
    HardBreak,
    Link {
        url: String,
        title: Option<String>,
        children: Vec<Inline>,
    },
    Image {
        url: String,
        title: Option<String>,
        alt: String,
    },
}

/// Component node with a name, attributes, and optional children.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub attrs: Vec<Attr>,
    pub children: Vec<Block>,
    pub self_closing: bool,
}

/// Component attribute as key=value (values are preserved as strings in the AST).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attr {
    pub key: String,
    pub value: String,
}

/// Parser error kinds (stable string codes used in CLI/WASM).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    LimitExceeded,
}

/// Parse error with 1-based line/col, kind, and human-readable message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize, // 1-based
    pub col: usize,  // 1-based, Unicode scalar values
    pub kind: ErrorKind,
    pub msg: String,
}

impl ErrorKind {
    /// Stable string code for JSON payloads and cross-language use.
    pub fn as_code(&self) -> &'static str {
        match self {
            ErrorKind::Syntax => "Syntax",
            ErrorKind::LimitExceeded => "LimitExceeded",
        }
    }
}

impl ParseError {
    /// Construct a new `ParseError`.
    pub fn new(line: usize, col: usize, kind: ErrorKind, msg: impl Into<String>) -> Self {
        Self { line, col, kind, msg: msg.into() }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self.kind {
            ErrorKind::Syntax => "Syntax",
            ErrorKind::LimitExceeded => "LimitExceeded",
        };
        write!(f, "{}:{} [{}]: {}", self.line, self.col, code, self.msg)
    }
}

impl std::error::Error for ParseError {}

/// Convenience helper to find an attribute value by key.
pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| a.key == key)
        .map(|a| a.value.as_str())
}

impl From<(usize, usize, &'_ str)> for ParseError {
    fn from(t: (usize, usize, &str)) -> Self {
        ParseError::new(t.0, t.1, ErrorKind::Syntax, t.2)
    }
}

impl From<(usize, usize, String)> for ParseError {
    fn from(t: (usize, usize, String)) -> Self {
        ParseError::new(t.0, t.1, ErrorKind::Syntax, t.2)
    }
}
