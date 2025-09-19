use proofdown_wasm::{wasm_parse, wasm_parse_with_limits};

#[test]
fn wasm_parse_parity_with_native() {
    let input = "# H\n\n<card title=\"T\" />\n";
    let native = proofdown_parser::parse(input).expect("native parse ok");
    let native_json = serde_json::to_value(&native).expect("json");
    let wasm_out = wasm_parse(input);
    let v: serde_json::Value = serde_json::from_str(&wasm_out).expect("wasm json");
    assert_eq!(v["ok"], true);
    assert_eq!(v["doc"], native_json);
}

#[test]
fn wasm_parse_crlf_parity() {
    let lf = "# T\n\n<card title=\"T\" />\n";
    let crlf = "# T\r\n\r\n<card title=\"T\" />\r\n";
    let v1: serde_json::Value = serde_json::from_str(&wasm_parse(lf)).unwrap();
    let v2: serde_json::Value = serde_json::from_str(&wasm_parse(crlf)).unwrap();
    assert!(v1["ok"].as_bool().unwrap());
    assert!(v2["ok"].as_bool().unwrap());
    assert_eq!(v1["doc"], v2["doc"]);
}

#[test]
fn wasm_parse_with_limits_enforces_input_size() {
    let s = "# T\n\n<card title=\"T\" />\n".repeat(100);
    let out = wasm_parse_with_limits(&s, 16, 50_000, 16);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ok"], false);
    assert_eq!(v["err"]["code"], "LimitExceeded");
}

#[test]
fn wasm_parse_never_panics_on_small_inputs() {
    use proptest::prelude::*;
    proptest!(|(s in ".{0,80}")| {
        let out = wasm_parse(&s);
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert!(v.get("ok").is_some());
    });
}
