use serde::{Deserialize, Serialize};

/// Root document consisting of a sequence of blocks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

/// Block-level nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    Component(Component),
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
