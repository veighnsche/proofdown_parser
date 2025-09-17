use proofdown_parser::parse;

#[test]
fn parse_minimal_document() {
    let input = r#"
# Title

<grid cols=3>
  <card title="Tests">Hello</card>
</grid>
"#;
    let doc = parse(input).expect("parse ok");
    assert!(!doc.blocks.is_empty());
}
