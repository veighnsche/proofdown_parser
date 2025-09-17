#[test]
fn crlf_and_lf_same_ast() {
    let lf = "# Title\n\n<card title=\"T\" />\n";
    let crlf = "# Title\r\n\r\n<card title=\"T\" />\r\n";
    let a = proofdown_parser::parse(lf).expect("parse lf");
    let b = proofdown_parser::parse(crlf).expect("parse crlf");
    assert_eq!(a, b, "AST must be identical for LF and CRLF inputs");
}

#[test]
fn cr_only_and_lf_same_ast() {
    let lf = "# A\n\n<grid cols=1>\n</grid>\n";
    let cr = "# A\r\r<grid cols=1>\r</grid>\r";
    let a = proofdown_parser::parse(lf).expect("parse lf");
    let b = proofdown_parser::parse(cr).expect("parse cr");
    assert_eq!(a, b, "AST must be identical for LF and CR-only inputs");
}
