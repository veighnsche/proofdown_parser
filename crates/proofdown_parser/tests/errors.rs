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

#[test]
fn attr_error_reports_position() {
    // Leading newlines to shift absolute base; unterminated quoted attribute
    let input = "\n\n<card title=\"unterminated>\n";
    let err = parse(input).unwrap_err();
    assert!(matches!(err.kind, proofdown_ast::ErrorKind::Syntax));
    // Positions should be computed from the absolute index, not default to (1,1)
    assert!(err.line >= 1);
    assert!(err.col >= 1);
    assert!(err.line != 1 || err.col != 1, "line/col should reflect absolute position");
}
