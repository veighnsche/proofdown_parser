use std::env;
use std::fs;

#[test]
fn minimal_golden_ast() {
    let pml = fs::read_to_string("tests/fixtures/minimal.pml").expect("read fixture");
    let doc = proofdown_parser::parse(&pml).expect("parse ok");
    let actual = serde_json::to_value(&doc).expect("json value");
    let golden_path = "tests/golden/minimal.json";
    if env::var("UPDATE_GOLDEN").ok().as_deref() == Some("1") {
        let pretty = serde_json::to_string_pretty(&actual).expect("pretty json");
        fs::write(golden_path, pretty).expect("write golden");
        return;
    }
    let expected_str = fs::read_to_string(golden_path).expect("read golden");
    let expected: serde_json::Value = serde_json::from_str(&expected_str).expect("parse golden");
    assert_eq!(actual, expected, "AST JSON must match golden (structure)");
}
