#![cfg_attr(not(feature = "wasm"), allow(unused))]

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use proofdown_ast::Document;
use proofdown_parser::parse;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn wasm_parse(input: &str) -> String {
    // Return a JSON string: { ok: bool, doc?: Document, err?: string }
    match parse(input) {
        Ok(doc) => serde_json::json!({ "ok": true, "doc": doc }).to_string(),
        Err(e) => serde_json::json!({ "ok": false, "err": e }).to_string(),
    }
}
