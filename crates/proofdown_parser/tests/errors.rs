use proofdown_parser::parse;

#[test]
fn unterminated_component_error() {
    let input = "<grid>"; // missing closing tag
    let err = parse(input).unwrap_err();
    assert!(matches!(err.kind, proofdown_ast::ErrorKind::Syntax));
}

#[test]
fn depth_limit_exceeded() {
    // Build a deeply nested set of components beyond the default limit (16)
    let mut s = String::new();
    for _ in 0..18 {
        s.push_str("<grid>");
    }
    for _ in 0..18 {
        s.push_str("</grid>");
    }
    let err = parse(&s).unwrap_err();
    assert!(matches!(err.kind, proofdown_ast::ErrorKind::LimitExceeded));
}
