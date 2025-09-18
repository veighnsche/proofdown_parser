use std::{env, fs, path::Path};

#[test]
fn v2_fixtures_match_golden() {
    let fixtures_dir = Path::new("tests/fixtures");
    if !fixtures_dir.exists() {
        return;
    }
    let update = env::var("UPDATE_GOLDEN").ok().as_deref() == Some("1");

    for entry in fs::read_dir(fixtures_dir).expect("read fixtures") {
        let entry = entry.expect("dirent");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("pml") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if !stem.starts_with("v2_") {
            continue;
        }
        let pml = fs::read_to_string(&path).expect("read v2 fixture");
        let doc = proofdown_parser::parse(&pml).expect("parse ok");
        let actual = serde_json::to_value(&doc).expect("json value");
        let golden_path = Path::new("tests/golden").join(format!("{}.json", stem));
        if update {
            let pretty = serde_json::to_string_pretty(&actual).expect("pretty json");
            fs::write(&golden_path, pretty).expect("write golden");
            continue;
        }
        let expected_str = fs::read_to_string(&golden_path).expect("read golden");
        let expected: serde_json::Value =
            serde_json::from_str(&expected_str).expect("parse golden");
        assert_eq!(actual, expected, "AST JSON must match golden for {}", stem);
    }
}
