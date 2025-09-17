use std::fs;

#[test]
fn minimal_golden_ast() {
    let pml = fs::read_to_string("tests/fixtures/minimal.pml").expect("read fixture");
    let doc = proofdown_parser::parse(&pml).expect("parse ok");
    let actual = serde_json::to_value(&doc).expect("json value");
    let expected_str = fs::read_to_string("tests/golden/minimal.json").expect("read golden");
    let expected: serde_json::Value = serde_json::from_str(&expected_str).expect("parse golden");
    assert_eq!(actual, expected, "AST JSON must match golden (structure)");
}
