use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    Component(Component),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub attrs: Vec<Attr>,
    pub children: Vec<Block>,
    pub self_closing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attr { pub key: String, pub value: String }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    LimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize, // 1-based
    pub col: usize,  // 1-based, Unicode scalar values
    pub kind: ErrorKind,
    pub msg: String,
}
