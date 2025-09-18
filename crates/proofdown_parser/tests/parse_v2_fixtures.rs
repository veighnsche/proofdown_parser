use proofdown_parser::parse;
use std::{fs, path::Path};

#[test]
fn parse_all_v2_fixtures() {
    let dir = Path::new("tests/fixtures");
    let entries = fs::read_dir(dir).expect("read fixtures dir");
    let mut found = false;

    for entry in entries {
        let entry = entry.expect("read dir entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("pml") {
            found = true;
            let text = fs::read_to_string(&path).expect("read fixture file");
            parse(&text).unwrap_or_else(|_| panic!("parse ok for {}", path.display()));
        }
    }

    assert!(found, "no .pml fixtures found under tests/fixtures");
}
