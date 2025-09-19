use anyhow::{bail, Context, Result};
use serde_json::{json, Map, Value};
use std::{env, fs, path::PathBuf};
use walkdir::WalkDir;

#[derive(Default)]
struct FileReport {
    errors: Vec<String>,
    warnings: Vec<String>,
}

fn contains_remote_ref(v: &Value) -> bool {
    fn walk(v: &Value) -> bool {
        match v {
            Value::Object(m) => {
                if let Some(Value::String(s)) = m.get("$ref") {
                    if s.starts_with("http://") || s.starts_with("https://") {
                        return true;
                    }
                }
                m.values().any(walk)
            }
            Value::Array(a) => a.iter().any(walk),
            _ => false,
        }
    }
    walk(v)
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut json_mode = false;
    let mut strict = false;
    let mut dir_arg: Option<String> = None;
    let mut full = false;
    for a in args.by_ref() {
        match a.as_str() {
            "--json" => json_mode = true,
            "--strict" => strict = true,
            "--full" => full = true,
            s if dir_arg.is_none() => {
                dir_arg = Some(s.to_string());
            }
            _ => {}
        }
    }
    let schemas_dir = PathBuf::from(dir_arg.unwrap_or_else(|| ".specs/schemas".to_string()));
    if !schemas_dir.exists() {
        bail!("schemas dir not found: {}", schemas_dir.display());
    }
    let mut total = 0usize;
    let mut passed = 0usize;
    let mut warned = 0usize;
    let mut failed = 0usize;
    let mut reports: Vec<(String, FileReport)> = Vec::new();

    const MAX_SCHEMA_BYTES: u64 = 5 * 1024 * 1024; // 5 MiB cap to avoid pathological sizes
    for entry in WalkDir::new(&schemas_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        total += 1;
        let mut rep = FileReport::default();
        // File size limit
        if let Ok(meta) = fs::metadata(path) {
            if meta.len() > MAX_SCHEMA_BYTES {
                rep.errors.push(format!(
                    "schema file too large: {} bytes (max {})",
                    meta.len(), MAX_SCHEMA_BYTES
                ));
                finalize_file(&mut reports, &schemas_dir, path, rep, &mut passed, &mut warned, &mut failed, strict);
                continue;
            }
        }
        let data =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let schema_json: Value = serde_json::from_str(&data)
            .with_context(|| format!("parsing {}", path.display()))?;

        // Forbid remote $ref (http/https)
        if contains_remote_ref(&schema_json) {
            rep.errors.push("remote $ref not allowed (http/https)".into());
        }

        // Root object
        let obj = match schema_json.as_object() {
            Some(o) => o,
            None => {
                rep.errors.push("schema root is not an object".into());
                finalize_file(&mut reports, &schemas_dir, path, rep, &mut passed, &mut warned, &mut failed, strict);
                continue;
            }
        };

        // $schema must exist and reference draft 2020-12 or 2019-09
        match obj.get("$schema").and_then(|v| v.as_str()) {
            Some(s) if s.contains("2020-12") || s.contains("2019-09") => {}
            Some(_) => rep.errors.push("$schema must reference draft 2020-12 or 2019-09".into()),
            None => rep.errors.push("missing $schema".into()),
        }

        // type must be object at root
        match obj.get("type").and_then(|v| v.as_str()) {
            Some("object") => {}
            Some(other) => rep.errors.push(format!("root type must be 'object', found '{}'", other)),
            None => rep.warnings.push("missing root 'type': 'object'".into()),
        }

        // properties shape
        if let Some(props) = obj.get("properties") {
            if !props.is_object() {
                rep.errors.push("'properties' must be an object".into());
            }
        }

        // required subset of properties
        if let (Some(req), Some(props)) = (obj.get("required"), obj.get("properties")) {
            if let (Some(req_arr), Some(props_obj)) = (req.as_array(), props.as_object()) {
                for k in req_arr.iter().filter_map(|v| v.as_str()) {
                    if !props_obj.contains_key(k) {
                        rep.errors.push(format!("'required' key '{}' not present in 'properties'", k));
                    }
                }
            } else {
                rep.errors.push("'required' must be an array and 'properties' must be an object".into());
            }
        }

        // unknown top-level keys (warn)
        let known: &[&str] = &[
            "$schema", "$id", "title", "description", "type", "properties", "required", "$defs",
            "allOf", "anyOf", "oneOf", "additionalProperties", "items", "enum", "const", "patternProperties",
        ];
        for k in obj.keys() {
            if !known.contains(&k.as_str()) {
                rep.warnings.push(format!("unknown top-level key: {}", k));
            }
        }

        // local $ref validation for '#/$defs/...'
        if let Some(defs) = obj.get("$defs") {
            if !defs.is_object() {
                rep.errors.push("'$defs' must be an object".into());
            }
        }
        // Composition keywords structure-only validation
        for key in ["allOf", "anyOf", "oneOf"] {
            if let Some(v) = obj.get(key) {
                if !v.is_array() {
                    rep.errors.push(format!("'{}' must be an array", key));
                }
            }
        }
        validate_local_refs(obj, &mut rep);

        // Optional full validation: attempt to compile schema
        if full && rep.errors.is_empty() {
            if let Err(e) = jsonschema::JSONSchema::compile(&schema_json) {
                rep.errors.push(format!("schema compilation failed: {}", e));
            }
        }

        finalize_file(&mut reports, &schemas_dir, path, rep, &mut passed, &mut warned, &mut failed, strict);
    }

    let summary = json!({
        "files": total,
        "passed": passed,
        "warned": warned,
        "failed": failed,
        "strict": strict,
    });
    if json_mode {
        let files_json: Vec<Value> = reports
            .iter()
            .map(|(p, r)| json!({ "path": p, "errors": r.errors, "warnings": r.warnings }))
            .collect();
        println!("{}", json!({ "summary": summary, "files": files_json }));
    } else {
        eprintln!(
            "validated {} schema files ({} passed, {} warned, {} failed){}",
            total,
            passed,
            warned,
            failed,
            if strict { " [strict]" } else { "" }
        );
        for (p, r) in &reports {
            if r.errors.is_empty() && r.warnings.is_empty() {
                continue;
            }
            eprintln!("- {}:", p);
            for e in &r.errors {
                eprintln!("  error: {}", e);
            }
            for w in &r.warnings {
                eprintln!("  warn:  {}", w);
            }
        }
    }

    if failed > 0 || (strict && warned > 0) {
        std::process::exit(1);
    }
    Ok(())
}

fn finalize_file(
    reports: &mut Vec<(String, FileReport)>,
    root: &PathBuf,
    path: &std::path::Path,
    rep: FileReport,
    passed: &mut usize,
    warned: &mut usize,
    failed: &mut usize,
    strict: bool,
) {
    let rel = path.strip_prefix(root).unwrap_or(path).display().to_string();
    if !rep.errors.is_empty() {
        *failed += 1;
    } else if !rep.warnings.is_empty() {
        *warned += 1;
    } else {
        *passed += 1;
    }
    // In strict mode, warnings count towards non-zero exit, but still recorded as warnings in summary.
    reports.push((rel, rep));
}

fn validate_local_refs(root: &Map<String, Value>, rep: &mut FileReport) {
    fn check_ref(path: &str, root: &Map<String, Value>, rep: &mut FileReport) {
        if !path.starts_with("#/") {
            return; // only local refs validated here
        }
        let mut cur: &Value = &Value::Object(root.clone());
        for seg in path.trim_start_matches('#').trim_start_matches('/').split('/') {
            let key = seg.replace("~1", "/").replace("~0", "~");
            match cur {
                Value::Object(map) => {
                    if let Some(next) = map.get(&key) {
                        cur = next;
                    } else {
                        rep.errors.push(format!("$ref target not found: {}", path));
                        return;
                    }
                }
                _ => {
                    rep.errors.push(format!("$ref path traversed non-object at segment '{}': {}", seg, path));
                    return;
                }
            }
        }
    }

    fn walk(v: &Value, root: &Map<String, Value>, rep: &mut FileReport) {
        match v {
            Value::Object(map) => {
                if let Some(Value::String(s)) = map.get("$ref") {
                    if s.starts_with("#/") {
                        check_ref(s, root, rep);
                    }
                }
                for (_k, vv) in map {
                    walk(vv, root, rep);
                }
            }
            Value::Array(arr) => {
                for vv in arr {
                    walk(vv, root, rep);
                }
            }
            _ => {}
        }
    }
    let v = Value::Object(root.clone());
    walk(&v, root, rep);
}
