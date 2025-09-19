#![cfg_attr(not(feature = "wasm"), allow(unused))]

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use proofdown_parser::{parse, parse_with_limits, ParserLimits};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn wasm_parse(input: &str) -> String {
    // Return a JSON string: { ok: bool, doc?: Document, err?: string }
    match parse(input) {
        Ok(doc) => serde_json::json!({ "ok": true, "doc": doc }).to_string(),
        Err(e) => serde_json::json!({
            "ok": false,
            "err": {
                "code": e.kind.as_code(),
                "msg": e.msg,
                "line": e.line,
                "col": e.col,
            }
        }).to_string(),
    }
}

/// Parse with explicit resource limits (useful for browsers/Workers)
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn wasm_parse_with_limits(input: &str, max_depth: usize, max_nodes: usize, max_input_bytes: usize) -> String {
    let lims = ParserLimits { max_depth, max_nodes, max_input_bytes };
    match parse_with_limits(input, lims) {
        Ok(doc) => serde_json::json!({ "ok": true, "doc": doc }).to_string(),
        Err(e) => serde_json::json!({
            "ok": false,
            "err": {
                "code": e.kind.as_code(),
                "msg": e.msg,
                "line": e.line,
                "col": e.col,
            }
        }).to_string(),
    }
}
