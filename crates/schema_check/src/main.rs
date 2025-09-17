use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::{env, fs, path::PathBuf};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let schemas_dir = PathBuf::from(args.next().unwrap_or_else(|| ".specs/schemas".to_string()));
    if !schemas_dir.exists() {
        bail!("schemas dir not found: {}", schemas_dir.display());
    }
    let mut count = 0usize;
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
        let data =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let schema_json: Value =
            serde_json::from_str(&data).with_context(|| format!("parsing {}", path.display()))?;
        // Minimal structural checks
        let obj = schema_json
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("schema root is not an object: {}", path.display()))?;
        let sch = obj
            .get("$schema")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("missing $schema in {}", path.display()))?;
        if !sch.contains("json-schema.org") {
            bail!(
                "$schema does not reference json-schema.org in {}",
                path.display()
            );
        }
        count += 1;
    }
    eprintln!("validated {} schema files (basic checks)", count);
    Ok(())
}
