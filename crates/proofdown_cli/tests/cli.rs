use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn parse_json_pretty_ok() {
    let fixture = std::path::Path::new("../proofdown_parser/tests/fixtures/minimal.pml");
    assert!(fixture.exists(), "fixture missing: {}", fixture.display());
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse").arg(fixture).arg("--json").arg("--pretty");
    cmd.assert().success().stdout(predicate::str::contains("\"blocks\"").and(predicate::str::contains("\n")));
}

#[test]
fn validate_json_ok() {
    let fixture = std::path::Path::new("../proofdown_parser/tests/fixtures/minimal.pml");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("validate").arg(fixture).arg("--json").arg("--pretty");
    cmd.assert().success().stdout(predicate::str::contains("\"ok\": true"));
}

#[test]
fn validate_limits_depth_enforced() {
    // Build a document with depth 6 (nested grids)
    let mut s = String::new();
    for _ in 0..6 { s.push_str("<grid cols=1>"); }
    for _ in 0..6 { s.push_str("</grid>"); }

    let mut tmp = NamedTempFile::new().expect("tmp");
    fs::write(tmp.path(), s).expect("write");

    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("validate")
        .arg(tmp.path())
        .arg("--limits.depth=5");
    // Expect non-zero exit because validator will enforce lower limit than default
    cmd.assert().failure().stderr(predicate::str::contains("Validation error"));
}
