use proofdown_ast::{Attr, Block, Component, Document};

#[test]
fn document_serde_roundtrip() {
    let doc = Document {
        blocks: vec![
            Block::Heading { level: 2, text: "Title".into() },
            Block::Paragraph { text: "Hello".into() },
            Block::Component(Component {
                name: "card".into(),
                attrs: vec![Attr { key: "title".into(), value: "T".into() }],
                children: vec![Block::Paragraph { text: "inside".into() }],
                self_closing: false,
            })
        ]
    };
    let json = serde_json::to_string(&doc).expect("serialize");
    let back: Document = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(doc, back);
}

#[test]
fn unknown_block_tag_fails() {
    let bad = r#"{
        "blocks": [ { "type": "Nope", "text": "x" } ]
    }"#;
    let parsed: Result<Document, _> = serde_json::from_str(bad);
    assert!(parsed.is_err(), "unknown block tag must fail deserialization");
}
