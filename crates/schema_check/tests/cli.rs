use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;
use std::process::Command;

fn write_schema(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let p = dir.join(name);
    fs::write(&p, content).expect("write schema");
    p
}

#[test]
fn ok_schema_passes_and_json_summary() {
    let td = tempdir().unwrap();
    let _p = write_schema(td.path(), "ok.json", r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {"a": {"type": "string"}},
        "required": ["a"]
    }"#);
    let mut cmd = Command::cargo_bin("schema_check").expect("bin");
    cmd.arg(td.path()).arg("--json");
    let out = cmd.assert().success().get_output().stdout.clone();
    let v: serde_json::Value = serde_json::from_slice(&out).expect("json");
    assert_eq!(v["summary"]["files"].as_u64(), Some(1));
    assert_eq!(v["summary"]["failed"].as_u64(), Some(0));
}

#[test]
fn missing_schema_is_error() {
    let td = tempdir().unwrap();
    let _p = write_schema(td.path(), "bad.json", r#"{
        "type": "object"
    }"#);
    let mut cmd = Command::cargo_bin("schema_check").expect("bin");
    cmd.arg(td.path());
    cmd.assert().failure().stderr(predicate::str::contains("missing $schema"));
}

#[test]
fn required_not_in_properties_is_error() {
    let td = tempdir().unwrap();
    let _p = write_schema(td.path(), "bad.json", r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {"a": {"type": "string"}},
        "required": ["b"]
    }"#);
    let mut cmd = Command::cargo_bin("schema_check").expect("bin");
    cmd.arg(td.path()).arg("--json");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("validated"))
        .stdout(predicate::str::contains("$ref").not());
}

#[test]
fn unknown_top_level_key_warns_and_strict_fails() {
    let td = tempdir().unwrap();
    let _p = write_schema(td.path(), "warn.json", r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {},
        "foo": 1
    }"#);
    // Non-strict passes with warning
    Command::cargo_bin("schema_check")
        .expect("bin")
        .args([td.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("warn:"));
    // Strict fails
    Command::cargo_bin("schema_check")
        .expect("bin")
        .args([td.path().to_str().unwrap(), "--strict"])
        .assert()
        .failure();
}
