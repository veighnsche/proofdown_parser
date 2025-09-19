#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use proofdown_wasm::wasm_parse;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_node);

#[wasm_bindgen_test]
fn wasm_parse_ok_in_node() {
    let input = "# T\n\n<card title=\"T\" />\n";
    let out = wasm_parse(input);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ok"], true);
    assert!(v["doc"].is_object());
}
