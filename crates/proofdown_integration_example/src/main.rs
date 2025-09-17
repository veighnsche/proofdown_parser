use anyhow::{Context, Result};
use proofdown_parser::parse;
use proofdown_validate::{validate, Limits};
use std::{env, fs};

fn main() -> Result<()> {
    let path = env::args().nth(1).unwrap_or_else(|| "../proofdown_parser/tests/fixtures/minimal.pml".to_string());
    let input = fs::read_to_string(&path).with_context(|| format!("reading {}", path))?;
    let doc = match parse(&input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Parse error at {}:{} [{}]: {}", e.line, e.col, match e.kind { proofdown_ast::ErrorKind::Syntax => "Syntax", proofdown_ast::ErrorKind::LimitExceeded => "LimitExceeded" }, e.msg);
            std::process::exit(1);
        }
    };
    match validate(&doc, Some(&Limits::default())) {
        Ok(()) => println!("Parsed and validated: {} blocks", doc.blocks.len()),
        Err(e) => {
            eprintln!("Validation error [{}]: {}", e.code(), e);
            std::process::exit(2);
        }
    }
    Ok(())
}
