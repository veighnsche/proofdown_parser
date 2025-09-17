use std::{env, fs, path::{Path, PathBuf}};

fn cases_root() -> PathBuf { PathBuf::from("tests/cases") }

#[test]
fn run_all_parse_cases() {
    let root = cases_root();
    if !root.exists() { return; }
    let mut failures = Vec::new();
    for entry in fs::read_dir(&root).expect("read cases root") {
        let entry = entry.expect("dirent");
        if !entry.file_type().expect("ft").is_dir() { continue; }
        let case_dir = entry.path();
        if let Err(err) = run_case(&case_dir) {
            failures.push(format!("{} => {}", case_dir.display(), err));
        }
    }
    if !failures.is_empty() {
        panic!("Parse case failures ({}):\n{}", failures.len(), failures.join("\n"));
    }
}

fn run_case(dir: &Path) -> Result<(), String> {
    let input_path = dir.join("input.pml");
    let expected_ok = dir.join("expected.json");
    let expected_err = dir.join("expected_error.json");
    let input = fs::read_to_string(&input_path).map_err(|e| format!("read {}: {}", input_path.display(), e))?;

    let update = env::var("UPDATE_GOLDEN").ok().as_deref() == Some("1");

    match proofdown_parser::parse(&input) {
        Ok(doc) => {
            // If both files exist, prefer success golden (expected.json)
            if expected_err.exists() && !expected_ok.exists() {
                return Err(format!("expected error but parse succeeded for {}", dir.display()));
            }
            let actual = serde_json::to_value(&doc).map_err(|e| e.to_string())?;
            if update || !expected_ok.exists() {
                let pretty = serde_json::to_string_pretty(&actual).map_err(|e| e.to_string())?;
                fs::write(&expected_ok, pretty).map_err(|e| e.to_string())?;
                return Ok(());
            }
            let expected_str = fs::read_to_string(&expected_ok).map_err(|e| e.to_string())?;
            let expected: serde_json::Value = serde_json::from_str(&expected_str).map_err(|e| e.to_string())?;
            if actual != expected {
                return Err(format!("AST did not match golden for {}", dir.display()));
            }
            Ok(())
        }
        Err(err) => {
            // If both files exist, prefer error golden (expected_error.json)
            if expected_ok.exists() && !expected_err.exists() {
                return Err(format!("expected success but got error for {}: {:?}", dir.display(), err));
            }
            let actual = serde_json::to_value(&err).map_err(|e| e.to_string())?;
            if update || !expected_err.exists() {
                let pretty = serde_json::to_string_pretty(&actual).map_err(|e| e.to_string())?;
                fs::write(&expected_err, pretty).map_err(|e| e.to_string())?;
                return Ok(());
            }
            let expected_str = fs::read_to_string(&expected_err).map_err(|e| e.to_string())?;
            let expected: serde_json::Value = serde_json::from_str(&expected_str).map_err(|e| e.to_string())?;
            if actual != expected {
                return Err(format!("Error did not match golden for {}", dir.display()));
            }
            Ok(())
        }
    }
}
