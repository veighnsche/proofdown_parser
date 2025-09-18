use proofdown_ast::{Attr, Block, Component, Document, Inline};

#[test]
fn document_serde_roundtrip() {
    let doc = Document {
        blocks: vec![
            Block::Heading {
                level: 2,
                inlines: vec![Inline::Text {
                    text: "Title".into(),
                }],
            },
            Block::Paragraph {
                inlines: vec![Inline::Text {
                    text: "Hello".into(),
                }],
            },
            Block::Component(Component {
                name: "card".into(),
                attrs: vec![Attr {
                    key: "title".into(),
                    value: "T".into(),
                }],
                children: vec![Block::Paragraph {
                    inlines: vec![Inline::Text {
                        text: "inside".into(),
                    }],
                }],
                self_closing: false,
            }),
        ],
    };
    let json = serde_json::to_string(&doc).expect("serialize");
    let back: Document = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(doc, back);
}

#[test]
fn list_and_table_roundtrip() {
    use proofdown_ast::{ListItem, ListKind, TableAlign, TableRow};
    let doc = Document {
        blocks: vec![
            Block::List {
                kind: ListKind::Ordered,
                start: Some(3),
                tight: true,
                items: vec![ListItem { children: vec![Block::Paragraph { inlines: vec![Inline::Text { text: "a".into() }] }], task: Some(true) }],
            },
            Block::Table {
                align: vec![TableAlign::Left, TableAlign::Center, TableAlign::Right],
                header: Some(TableRow { cells: vec![vec![Inline::Text { text: "H1".into() }], vec![Inline::Text { text: "H2".into() }], vec![Inline::Text { text: "H3".into() }]] }),
                rows: vec![TableRow { cells: vec![vec![Inline::Text { text: "a".into() }], vec![Inline::Text { text: "b".into() }], vec![Inline::Text { text: "c".into() }]] }],
            },
        ],
    };
    let json = serde_json::to_string(&doc).expect("serialize");
    let back: Document = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(doc, back);
}

#[test]
fn parse_error_serde_roundtrip() {
    use proofdown_ast::{ErrorKind, ParseError};
    let e = ParseError { line: 2, col: 10, kind: ErrorKind::Syntax, msg: "bad".into() };
    let json = serde_json::to_string(&e).unwrap();
    let back: ParseError = serde_json::from_str(&json).unwrap();
    assert_eq!(e, back);
}

#[test]
fn unknown_block_tag_fails() {
    let bad = r#"{
        "blocks": [ { "type": "Nope", "text": "x" } ]
    }"#;
    let parsed: Result<Document, _> = serde_json::from_str(bad);
    assert!(
        parsed.is_err(),
        "unknown block tag must fail deserialization"
    );
}
