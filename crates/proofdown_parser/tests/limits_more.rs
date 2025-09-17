use proofdown_parser::{parse_with_limits, ParserLimits};

#[test]
fn input_size_limit_exceeded() {
    let big = "a".repeat(1024);
    // Set a very small input limit to avoid large test files
    let limits = ParserLimits {
        max_input_bytes: 512,
        ..Default::default()
    };
    let err = parse_with_limits(&big, limits).unwrap_err();
    assert!(
        matches!(err.kind, proofdown_ast::ErrorKind::LimitExceeded),
        "expected LimitExceeded, got: {:?}",
        err
    );
}

#[test]
fn node_count_limit_exceeded() {
    // Build a document with many self-closing components to exceed node count
    let mut s = String::from("# Title\n\n");
    for _ in 0..200 {
        s.push_str("<artifact.json id=\"x\" />\n");
    }
    let limits = ParserLimits {
        max_nodes: 50,
        ..Default::default()
    };
    let err = parse_with_limits(&s, limits).unwrap_err();
    assert!(matches!(err.kind, proofdown_ast::ErrorKind::LimitExceeded));
}
