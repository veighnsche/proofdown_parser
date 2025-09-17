use proofdown_ast::{Block, Document};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub max_input_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_depth: 16, max_nodes: 50_000, max_input_bytes: 1 << 20 }
    }
}

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("depth exceeded: {depth} > {max_depth}")]
    DepthExceeded { depth: usize, max_depth: usize },

    #[error("node limit exceeded: {nodes} > {max_nodes}")]
    NodeLimit { nodes: usize, max_nodes: usize },

    #[error("unknown component: {name}")]
    UnknownComponent { name: String },

    #[error("unknown attribute: {name}.{key}")]
    UnknownAttribute { name: String, key: String },

    #[error("missing attribute: {name}.{key}")]
    MissingAttribute { name: String, key: String },

    #[error("attribute type error: {name}.{key} expected {expected}")]
    AttrType { name: String, key: String, expected: &'static str },

    #[error("attribute out of bounds: {name}.{key}={value} not in {range}")]
    AttrBound { name: String, key: String, value: String, range: &'static str },
}

pub fn validate(doc: &Document, limits: Option<&Limits>) -> Result<(), ValidateError> {
    let limits = limits.copied().unwrap_or_default();
    let (depth, nodes) = analyze(doc);
    if depth > limits.max_depth {
        return Err(ValidateError::DepthExceeded { depth, max_depth: limits.max_depth });
    }
    if nodes > limits.max_nodes {
        return Err(ValidateError::NodeLimit { nodes, max_nodes: limits.max_nodes });
    }
    // Whitelist & attribute schema checks
    for b in &doc.blocks {
        validate_block(b)?;
    }
    Ok(())
}

fn analyze(doc: &Document) -> (usize, usize) {
    fn walk(blocks: &[Block], cur_depth: usize, out_depth: &mut usize, out_nodes: &mut usize) {
        *out_depth = (*out_depth).max(cur_depth);
        for b in blocks {
            *out_nodes += 1;
            match b {
                Block::Component(c) => {
                    if !c.children.is_empty() {
                        walk(&c.children, cur_depth + 1, out_depth, out_nodes);
                    }
                }
                _ => {}
            }
        }
    }
    let mut depth = 0;
    let mut nodes = 0;
    walk(&doc.blocks, 1, &mut depth, &mut nodes);
    (depth, nodes)
}

fn validate_block(b: &Block) -> Result<(), ValidateError> {
    match b {
        Block::Component(c) => {
            validate_component(c)?;
            for ch in &c.children { validate_block(ch)?; }
        }
        _ => {}
    }
    Ok(())
}

fn validate_component(c: &proofdown_ast::Component) -> Result<(), ValidateError> {
    let name = c.name.as_str();
    match name {
        // Structural
        "grid" => {
            // attrs: cols=1..6 (required), gap=0..64 (optional)
            let mut seen = std::collections::HashSet::new();
            for a in &c.attrs { seen.insert(a.key.as_str()); }
            if !seen.contains("cols") {
                return Err(ValidateError::MissingAttribute { name: c.name.clone(), key: "cols".into() });
            }
            let cols = parse_int_attr(c, "cols")?;
            if cols < 1 || cols > 6 {
                return Err(ValidateError::AttrBound { name: c.name.clone(), key: "cols".into(), value: cols.to_string(), range: "1..=6" });
            }
            if seen.contains("gap") {
                let gap = parse_int_attr(c, "gap")?;
                if gap < 0 || gap > 64 { return Err(ValidateError::AttrBound { name: c.name.clone(), key: "gap".into(), value: gap.to_string(), range: "0..=64" }); }
            }
            // unknown attributes
            for a in &c.attrs {
                if a.key != "cols" && a.key != "gap" {
                    return Err(ValidateError::UnknownAttribute { name: c.name.clone(), key: a.key.clone() });
                }
            }
        }
        "section" => {
            require_attr_present(c, "title")?;
            forbid_unknown(c, &["title"]) ?;
        }
        "card" => {
            require_attr_present(c, "title")?;
            forbid_unknown(c, &["title"]) ?;
        }

        // Artifacts (require id)
        "artifact.summary" | "artifact.table" | "artifact.markdown" | "artifact.link" | "artifact.json" | "artifact.image" => {
            require_attr_present(c, "id")?;
            match name {
                "artifact.json" => {
                    // optional: collapsed=true|false, depth=0..8
                    if has_attr(c, "collapsed") { parse_bool_attr(c, "collapsed")?; }
                    if has_attr(c, "depth") {
                        let d = parse_int_attr(c, "depth")?; if d < 0 || d > 8 { return Err(ValidateError::AttrBound { name: c.name.clone(), key: "depth".into(), value: d.to_string(), range: "0..=8" }); }
                    }
                    forbid_unknown(c, &["id", "collapsed", "depth"]) ?;
                }
                "artifact.image" => {
                    require_attr_present(c, "alt")?;
                    if has_attr(c, "max_height") { let mh = parse_int_attr(c, "max_height")?; if mh < 128 || mh > 2048 { return Err(ValidateError::AttrBound { name: c.name.clone(), key: "max_height".into(), value: mh.to_string(), range: "128..=2048" }); } }
                    forbid_unknown(c, &["id", "alt", "max_height"]) ?;
                }
                "artifact.link" => {
                    if has_attr(c, "download") { parse_bool_attr(c, "download")?; }
                    if has_attr(c, "title") { /* any string */ }
                    forbid_unknown(c, &["id", "download", "title"]) ?;
                }
                _ => {
                    forbid_unknown(c, &["id"]) ?;
                }
            }
        }

        // Optional repo viewers (accept but do minimal validation)
        "repo.code" | "repo.link" | "repo.tree" | "repo.diff" | "repo.symbol" => {
            // accept known names; unknown attrs left for future
        }
        _ => return Err(ValidateError::UnknownComponent { name: c.name.clone() }),
    }
    Ok(())
}

fn has_attr(c: &proofdown_ast::Component, key: &str) -> bool {
    c.attrs.iter().any(|a| a.key == key)
}

fn require_attr_present(c: &proofdown_ast::Component, key: &str) -> Result<(), ValidateError> {
    if !has_attr(c, key) { return Err(ValidateError::MissingAttribute { name: c.name.clone(), key: key.into() }); }
    Ok(())
}

fn forbid_unknown(c: &proofdown_ast::Component, allowed: &[&str]) -> Result<(), ValidateError> {
    for a in &c.attrs {
        if !allowed.iter().any(|k| k == &a.key) {
            return Err(ValidateError::UnknownAttribute { name: c.name.clone(), key: a.key.clone() });
        }
    }
    Ok(())
}

fn parse_int_attr(c: &proofdown_ast::Component, key: &str) -> Result<i64, ValidateError> {
    let v = c.attrs.iter().find(|a| a.key == key).unwrap();
    v.value.parse::<i64>().map_err(|_| ValidateError::AttrType { name: c.name.clone(), key: key.into(), expected: "integer" })
}

fn parse_bool_attr(c: &proofdown_ast::Component, key: &str) -> Result<bool, ValidateError> {
    let v = c.attrs.iter().find(|a| a.key == key).unwrap();
    match v.value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ValidateError::AttrType { name: c.name.clone(), key: key.into(), expected: "boolean true|false" }),
    }
}
