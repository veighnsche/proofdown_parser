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
    // Placeholder for whitelist & attribute schema checks (scaffold only)
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
