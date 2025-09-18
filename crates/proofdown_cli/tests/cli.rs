use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn help_and_version_work() {
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));

    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap());
}

#[test]
fn parse_human_error_exit_code_1() {
    let tmp = NamedTempFile::new().expect("tmp");
    fs::write(tmp.path(), "<card title=\"unterminated>").expect("write");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse").arg(tmp.path());
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Parse error at"));
}

#[test]
fn validate_human_error_exit_code_2() {
    let tmp = NamedTempFile::new().expect("tmp");
    fs::write(tmp.path(), "<foo id=\"x\" />").expect("write");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("validate").arg(tmp.path());
    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Validation error"));
}

#[test]
fn parse_from_stdin_json_ok() {
    let input = "# T\n\n<card title=\"T\" />\n";
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse").arg("-").arg("--json")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"blocks\""));
}

#[test]
fn parse_file_not_found_json() {
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse").arg("/no/such/file.pml").arg("--json");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("\"code\":\"IO\""));
}

#[test]
fn parse_non_utf8() {
    let tmp = NamedTempFile::new().expect("tmp");
    // Write invalid UTF-8 bytes
    let bytes = b"\xff\xfe\xfa";
    fs::write(tmp.path(), bytes).expect("write");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse").arg(tmp.path()).arg("--json");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("\"code\":\"IO\""));
}

#[test]
fn parse_json_error_payload_shape() {
    let tmp = NamedTempFile::new().expect("tmp");
    fs::write(tmp.path(), "<card title=\"unterminated>").expect("write");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse")
        .arg(tmp.path())
        .arg("--json")
        .arg("--pretty");
    cmd.assert().failure().stderr(
        predicate::str::contains("\"ok\": false")
            .and(predicate::str::contains("\"code\": \"Syntax\""))
            .and(predicate::str::contains("\"line\":"))
            .and(predicate::str::contains("\"col\":")),
    );
}

#[test]
fn validate_json_error_payload_shape() {
    let tmp = NamedTempFile::new().expect("tmp");
    fs::write(tmp.path(), "<foo id=\"x\" />").expect("write");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("validate")
        .arg(tmp.path())
        .arg("--json")
        .arg("--pretty");
    cmd.assert().failure().stderr(
        predicate::str::contains("\"ok\": false").and(predicate::str::contains("\"code\":")),
    );
}

#[test]
fn parse_crlf_json_parity() {
    let lf = "# T\n\n<card title=\"T\" />\n";
    let crlf = "# T\r\n\r\n<card title=\"T\" />\r\n";
    let tmp1 = NamedTempFile::new().expect("tmp1");
    fs::write(tmp1.path(), lf).expect("write");
    let tmp2 = NamedTempFile::new().expect("tmp2");
    fs::write(tmp2.path(), crlf).expect("write");
    let out1 = Command::cargo_bin("pml")
        .expect("bin")
        .args(["parse", tmp1.path().to_str().unwrap(), "--json"])
        .output()
        .expect("run");
    let out2 = Command::cargo_bin("pml")
        .expect("bin")
        .args(["parse", tmp2.path().to_str().unwrap(), "--json"])
        .output()
        .expect("run");
    assert!(out1.status.success());
    assert!(out2.status.success());
    let v1: serde_json::Value = serde_json::from_slice(&out1.stdout).expect("json1");
    let v2: serde_json::Value = serde_json::from_slice(&out2.stdout).expect("json2");
    assert_eq!(v1, v2, "JSON AST must be equal for LF vs CRLF inputs");
}
#[test]
fn parse_json_pretty_ok() {
    let fixture = std::path::Path::new("../proofdown_parser/tests/fixtures/minimal.pml");
    assert!(fixture.exists(), "fixture missing: {}", fixture.display());
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse").arg(fixture).arg("--json").arg("--pretty");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"blocks\"").and(predicate::str::contains("\n")));
}

#[test]
fn validate_json_ok() {
    let fixture = std::path::Path::new("../proofdown_parser/tests/fixtures/minimal.pml");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("validate")
        .arg(fixture)
        .arg("--json")
        .arg("--pretty");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"));
}

#[test]
fn validate_limits_depth_enforced() {
    // Build a document with depth 6 (nested grids)
    let mut s = String::new();
    for _ in 0..6 {
        s.push_str("<grid cols=1>");
    }
    for _ in 0..6 {
        s.push_str("</grid>");
    }

    let tmp = NamedTempFile::new().expect("tmp");
    fs::write(tmp.path(), s).expect("write");

    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("validate").arg(tmp.path()).arg("--limits.depth=5");
    // Expect non-zero exit because validator will enforce lower limit than default
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Validation error"));
}

#[test]
fn parse_limits_input_size_enforced() {
    let tmp = NamedTempFile::new().expect("tmp");
    // 50 bytes input
    let s = "# T\n\n<card title=\"T\" />\n".repeat(2);
    fs::write(tmp.path(), s).expect("write");
    let mut cmd = Command::cargo_bin("pml").expect("bin");
    cmd.arg("parse")
        .arg(tmp.path())
        .arg("--json")
        .arg("--limits.input-size=16");
    cmd.assert().failure().stderr(
        predicate::str::contains("LimitExceeded")
            .or(predicate::str::contains("input exceeds max size")),
    );
}
