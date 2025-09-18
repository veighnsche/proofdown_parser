//! Proofdown parser — minimal, deterministic grammar to a typed AST.
//!
//! Scope (v1):
//! - Headings `#..####` with one space after the marker; empty titles become paragraphs.
//! - Paragraphs: consecutive non-empty lines until a blank or a component/heading start.
//! - Components: HTML-like `<name key="value" ...>` with optional children until `</name>`; self-closing `<name ... />` supported.
//! - Attributes: quoted or bare values preserved exactly; parsed as strings here, typed later by validators.
//! - Limits: depth ≤ 16, nodes ≤ 50k, input bytes ≤ 1 MiB (configurable via `ParserLimits`).
//!
//! Non-goals:
//! - No IO; no semantic validation — that is handled by `proofdown_validate`/SSG.
//! - No HTML passthrough. Rendering is separate and deterministic.
//!
//! See also: `.specs/00_proofdown_parser.md`, v2 additive notes, and `.docs/proofdown-authoring-guide.md`.
use comrak::nodes::{AstNode, ListType, NodeValue, TableAlignment};
use comrak::{parse_document, Arena, ComrakOptions};
use proofdown_ast::{
    Attr, Block, Component, Document, ErrorKind, Inline, ListItem, ListKind, ParseError,
};

type PResult<T> = std::result::Result<T, ParseError>;

// Default limits per contract (§6): depth <= 16, nodes <= 50k, input <= 1 MiB
const MAX_DEPTH: usize = 16;
const MAX_NODES: usize = 50_000;
const MAX_INPUT_BYTES: usize = 1 << 20;

#[derive(Debug, Clone, Copy)]
pub struct ParserLimits {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub max_input_bytes: usize,
}

impl Default for ParserLimits {
    fn default() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            max_nodes: MAX_NODES,
            max_input_bytes: MAX_INPUT_BYTES,
        }
    }
}

pub fn parse(input: &str) -> PResult<Document> {
    parse_with_limits(input, ParserLimits::default())
}

pub fn parse_with_limits(input: &str, limits: ParserLimits) -> PResult<Document> {
    // Normalize newlines to \n for consistent scanning across platforms
    let normalized: std::borrow::Cow<'_, str> = if input.contains('\r') {
        std::borrow::Cow::Owned(input.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        std::borrow::Cow::Borrowed(input)
    };
    let input = &*normalized;
    if input.len() > limits.max_input_bytes {
        return Err(mk_err(
            input,
            0,
            ErrorKind::LimitExceeded,
            format!("input exceeds max size ({} bytes)", limits.max_input_bytes),
        ));
    }
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut blocks = Vec::new();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let (comp, used) = parse_component(input, i)?;
            blocks.push(Block::Component(comp));
            i += used;
        } else {
            // Take chunk until next '<' or EOF and parse as CommonMark
            let next_lt = input[i..].find('<').map(|o| i + o).unwrap_or(bytes.len());
            let chunk = &input[i..next_lt];
            let mut md_blocks = parse_markdown_blocks(chunk);
            blocks.append(&mut md_blocks);
            i = next_lt;
        }
        // skip any trailing newlines
        while i < bytes.len() && (bytes[i] == b'\n' || bytes[i] == b'\r') {
            i += 1;
        }
    }
    let doc = Document { blocks };
    // Enforce depth and node limits post-parse (defaults for now; configurable in follow-up API)
    let (depth, nodes) = analyze(&doc);
    if depth > limits.max_depth {
        return Err(mk_err(
            input,
            0,
            ErrorKind::LimitExceeded,
            format!("depth exceeded: {} > {}", depth, limits.max_depth),
        ));
    }
    if nodes > limits.max_nodes {
        return Err(mk_err(
            input,
            0,
            ErrorKind::LimitExceeded,
            format!("node limit exceeded: {} > {}", nodes, limits.max_nodes),
        ));
    }
    Ok(doc)
}

fn parse_component(full: &str, base: usize) -> PResult<(Component, usize)> {
    // Parse <name ...> ... </name> or <name ... />; MVP without nested error kinds
    let src = &full[base..];
    if !src.starts_with('<') {
        return Err(mk_err(full, base, ErrorKind::Syntax, "expected '<'"));
    }
    let gt = match src.find('>') {
        Some(i) => i,
        None => return Err(mk_err(full, base, ErrorKind::Syntax, "unterminated tag")),
    };
    let mut head = src[1..gt].trim();
    let self_close = head.ends_with('/');
    if self_close {
        head = head.trim_end_matches('/').trim();
    }
    let mut parts = head.split_whitespace();
    let name = match parts.next() {
        Some(n) => n.to_string(),
        None => {
            return Err(mk_err(
                full,
                base + 1,
                ErrorKind::Syntax,
                "missing component name",
            ))
        }
    };
    let rest = &head[name.len()..].trim();
    let attrs = parse_attrs(rest)?;
    let mut used = gt + 1; // relative to base
    let mut children = Vec::new();
    if !self_close {
        let close_tag = format!("</{}>", name);
        loop {
            let remain = &full[base + used..];
            if remain.starts_with(&close_tag) {
                break;
            }
            if remain.is_empty() {
                return Err(mk_err(
                    full,
                    base + used,
                    ErrorKind::Syntax,
                    format!("unterminated component <{}>", name),
                ));
            }
            if let Some(stripped) = remain.strip_prefix("</") {
                // mismatched close
                let name_end = stripped.find('>').unwrap_or(stripped.len());
                let close_name = stripped[..name_end].trim();
                return Err(mk_err(
                    full,
                    base + used,
                    ErrorKind::Syntax,
                    format!("unexpected close </{}>", close_name),
                ));
            }
            if remain.starts_with('<') {
                let (child, cused) = parse_component(full, base + used)?;
                children.push(Block::Component(child));
                used += cused;
            } else {
                // consume until next '<' or close_tag
                let idx_close = remain.find(&close_tag);
                let idx_lt = remain.find('<');
                let end_rel = match (idx_close, idx_lt) {
                    (Some(c), Some(l)) => c.min(l),
                    (Some(c), None) => c,
                    (None, Some(l)) => l,
                    (None, None) => remain.len(),
                };
                let chunk = &remain[..end_rel];
                let mut md_children = parse_markdown_blocks(chunk);
                children.append(&mut md_children);
                used += end_rel;
            }
        }
        used += close_tag.len();
    }
    Ok((
        Component {
            name,
            attrs,
            children,
            self_closing: self_close,
        },
        used,
    ))
}

fn parse_attrs(mut src: &str) -> PResult<Vec<Attr>> {
    let mut out = Vec::new();
    src = src.trim();
    while !src.is_empty() {
        let eq = match src.find('=') {
            Some(i) => i,
            None => {
                // if trailing non-whitespace remains, it's a malformed attribute token
                let rest = src.trim();
                if !rest.is_empty() {
                    return Err(ParseError {
                        line: 1,
                        col: 1,
                        kind: ErrorKind::Syntax,
                        msg: format!("malformed attribute near '{}': expected key=value", rest),
                    });
                }
                break;
            }
        };
        let key = src[..eq].trim().to_string();
        src = &src[eq + 1..];
        if src.starts_with('"') {
            src = &src[1..];
            let end = match src.find('"') {
                Some(i) => i,
                None => {
                    return Err(ParseError {
                        line: 1,
                        col: 1,
                        kind: ErrorKind::Syntax,
                        msg: "unterminated quoted attr".into(),
                    })
                }
            };
            out.push(Attr {
                key,
                value: src[..end].to_string(),
            });
            src = src[end + 1..].trim_start();
        } else {
            let mut end = src.find(' ').unwrap_or(src.len());
            if let Some(gt) = src.find('>') {
                end = end.min(gt);
            }
            // Preserve bare values exactly up to whitespace or '>'
            out.push(Attr {
                key,
                value: src[..end].to_string(),
            });
            src = src[end..].trim_start();
        }
    }
    Ok(out)
}

pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| a.key == key)
        .map(|a| a.value.as_str())
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
                Block::BlockQuote { children } => {
                    walk(children, cur_depth, out_depth, out_nodes);
                }
                Block::List { items, .. } => {
                    for it in items {
                        walk(&it.children, cur_depth, out_depth, out_nodes);
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

fn mk_err(full: &str, index: usize, kind: ErrorKind, msg: impl Into<String>) -> ParseError {
    let (line, col) = pos_to_line_col(full, index);
    ParseError {
        line,
        col,
        kind,
        msg: msg.into(),
    }
}

fn pos_to_line_col(s: &str, index: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, ch) in s.char_indices() {
        if i >= index {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn parse_markdown_blocks(src: &str) -> Vec<Block> {
    let arena = Arena::new();
    let mut opts = ComrakOptions::default();
    // Deterministic subset: disable smart punctuation, disallow raw HTML
    opts.parse.smart = false;
    opts.render.unsafe_ = false;
    // Enable selected GFM extensions
    opts.extension.table = true;
    opts.extension.strikethrough = true;
    opts.extension.autolink = true;
    opts.extension.tasklist = true;
    let root = parse_document(&arena, src, &opts);
    let mut blocks = Vec::new();
    for node in root.children() {
        if let Some(b) = map_block(node) {
            blocks.push(b);
        }
    }
    blocks
}

fn map_block<'a>(node: &'a AstNode<'a>) -> Option<Block> {
    match &node.data.borrow().value {
        NodeValue::Paragraph => Some(Block::Paragraph {
            inlines: collect_inlines(node),
        }),
        NodeValue::Heading(h) => Some(Block::Heading {
            level: h.level,
            inlines: collect_inlines(node),
        }),
        NodeValue::ThematicBreak => Some(Block::ThematicBreak),
        NodeValue::BlockQuote => Some(Block::BlockQuote {
            children: collect_blocks(node),
        }),
        NodeValue::CodeBlock(cb) => {
            let info = cb.info.clone();
            let text = cb.literal.clone();
            Some(Block::CodeBlock { info, text })
        }
        NodeValue::List(l) => {
            let kind = match l.list_type {
                ListType::Bullet => ListKind::Bullet,
                ListType::Ordered => ListKind::Ordered,
            };
            let mut items = Vec::new();
            for child in node.children() {
                if let NodeValue::Item(_li) = &child.data.borrow().value {
                    let task = if let Some(ch) = child.first_child() {
                        if let NodeValue::TaskItem(checked) = ch.data.borrow().value {
                            Some(checked.is_some())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    items.push(ListItem {
                        children: collect_blocks(child),
                        task,
                    });
                }
            }
            let start = if l.start == 1 {
                None
            } else {
                Some(l.start as u64)
            };
            Some(Block::List {
                kind,
                start,
                tight: l.tight,
                items,
            })
        }
        NodeValue::Table(t) => {
            // Alignments
            let align = t
                .alignments
                .iter()
                .map(|a| match a {
                    TableAlignment::None => proofdown_ast::TableAlign::None,
                    TableAlignment::Left => proofdown_ast::TableAlign::Left,
                    TableAlignment::Center => proofdown_ast::TableAlign::Center,
                    TableAlignment::Right => proofdown_ast::TableAlign::Right,
                })
                .collect::<Vec<_>>();
            // Gather rows; use TableRow(bool) header flag where provided
            let mut header: Option<proofdown_ast::TableRow> = None;
            let mut rows = Vec::new();
            for ch in node.children() {
                if let NodeValue::TableRow(is_header) = ch.data.borrow().value {
                    let row = map_table_row(ch);
                    if is_header && header.is_none() {
                        header = Some(row);
                    } else {
                        rows.push(row);
                    }
                }
            }
            Some(Block::Table {
                align,
                header,
                rows,
            })
        }
        NodeValue::HtmlBlock(..) => None, // drop raw HTML blocks
        NodeValue::Text(t) if t.is_empty() => None,
        _ => None,
    }
}

fn map_table_row<'a>(row: &'a AstNode<'a>) -> proofdown_ast::TableRow {
    // Map a Comrak TableRow into a TableRow with inline cells.
    let mut cells: Vec<Vec<Inline>> = Vec::new();
    for cell in row.children() {
        if let NodeValue::TableCell = &cell.data.borrow().value {
            let mut inlines: Vec<Inline> = Vec::new();
            // Some parsers wrap cell contents in a Paragraph; support both.
            for n in cell.children() {
                match &n.data.borrow().value {
                    NodeValue::Paragraph => {
                        // Paragraph's children are inline nodes.
                        inlines.extend(collect_inlines(n));
                    }
                    _ => {
                        // Try to map directly as an inline, otherwise collect any inline children.
                        if let Some(i) = map_inline(n) {
                            inlines.push(i);
                        } else {
                            inlines.extend(collect_inlines(n));
                        }
                    }
                }
            }
            cells.push(inlines);
        }
    }
    proofdown_ast::TableRow { cells }
}

fn collect_blocks<'a>(node: &'a AstNode<'a>) -> Vec<Block> {
    let mut v = Vec::new();
    for ch in node.children() {
        if let Some(b) = map_block(ch) {
            v.push(b);
        }
    }
    v
}

fn collect_inlines<'a>(node: &'a AstNode<'a>) -> Vec<Inline> {
    let mut out = Vec::new();
    for ch in node.children() {
        if let Some(i) = map_inline(ch) {
            out.push(i);
        }
    }
    out
}

fn map_inline<'a>(node: &'a AstNode<'a>) -> Option<Inline> {
    match &node.data.borrow().value {
        NodeValue::Text(t) => Some(Inline::Text {
            text: t.to_string(),
        }),
        NodeValue::SoftBreak => Some(Inline::SoftBreak),
        NodeValue::LineBreak => Some(Inline::HardBreak),
        NodeValue::Code(code) => Some(Inline::Code {
            text: code.literal.clone(),
        }),
        NodeValue::Strikethrough => Some(Inline::Strikethrough {
            children: collect_inlines(node),
        }),
        NodeValue::Emph => Some(Inline::Emph {
            children: collect_inlines(node),
        }),
        NodeValue::Strong => Some(Inline::Strong {
            children: collect_inlines(node),
        }),
        NodeValue::Link(l) => Some(Inline::Link {
            url: l.url.to_string(),
            title: if l.title.is_empty() {
                None
            } else {
                Some(l.title.to_string())
            },
            children: collect_inlines(node),
        }),
        NodeValue::Image(l) => Some(Inline::Image {
            url: l.url.to_string(),
            title: if l.title.is_empty() {
                None
            } else {
                Some(l.title.to_string())
            },
            alt: collect_inlines(node)
                .into_iter()
                .filter_map(|i| match i {
                    Inline::Text { text } => Some(text),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" "),
        }),
        NodeValue::HtmlInline(..) => None,
        _ => None,
    }
}
