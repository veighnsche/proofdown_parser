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

fn detect_task_item<'a>(node: &'a AstNode<'a>) -> Option<bool> {
    // Depth-first search for a TaskItem marker; return its checked state as boolean.
    for ch in node.children() {
        if let NodeValue::TaskItem(payload) = &ch.data.borrow().value {
            // Robust across comrak versions: payload may be bool, Option<char>, Option<bool>
            let s = format!("{:?}", payload);
            let state = match s.as_str() {
                "true" | "Some(true)" | "Some('x')" | "Some('X')" => true,
                _ => false,
            };
            return Some(state);
        }
        if let Some(state) = detect_task_item(ch) {
            return Some(state);
        }
    }
    None
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
    // Compute absolute base index for the attributes slice to improve error positions.
    // Use the untrimmed head to find the precise start.
    let head_raw = &src[1..gt];
    let head_lead_ws = head_raw.len() - head_raw.trim_start().len();
    let head_abs_start = base + 1 + head_lead_ws; // absolute index of the start of `head`
    let rest_pre = &head[name.len()..];
    let rest_lead_ws = rest_pre.len() - rest_pre.trim_start().len();
    let rest = rest_pre.trim();
    let rest_abs_base = head_abs_start + name.len() + rest_lead_ws;
    let attrs = parse_attrs(full, rest, rest_abs_base)?;
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

fn parse_attrs(full: &str, src: &str, abs_base: usize) -> PResult<Vec<Attr>> {
    // Parse attributes from `src`, reporting errors with accurate positions based on `abs_base`.
    let mut out = Vec::new();
    let mut i = 0usize; // byte offset into `src`
    let bytes = src.as_bytes();
    // skip leading whitespace
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    while i < bytes.len() {
        // find '=' separating key=value
        let rest = &src[i..];
        let eq_rel = match rest.find('=') {
            Some(pos) => pos,
            None => {
                // trailing garbage token -> error at current absolute position
                let token = rest.trim();
                if !token.is_empty() {
                    return Err(mk_err(
                        full,
                        abs_base + i,
                        ErrorKind::Syntax,
                        format!("malformed attribute near '{}': expected key=value", token),
                    ));
                }
                break;
            }
        };
        let key = rest[..eq_rel].trim().to_string();
        i += eq_rel + 1; // advance past '='
        if i >= src.len() {
            return Err(mk_err(full, abs_base + i.saturating_sub(1), ErrorKind::Syntax, "missing attribute value"));
        }
        if bytes[i] == b'"' {
            // quoted value
            i += 1; // skip opening quote
            let after = &src[i..];
            match after.find('"') {
                Some(end_rel) => {
                    let val = &src[i..i + end_rel];
                    out.push(Attr { key, value: val.to_string() });
                    i += end_rel + 1; // closing quote
                    // trim following whitespace
                    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                        i += 1;
                    }
                }
                None => {
                    return Err(mk_err(full, abs_base + i, ErrorKind::Syntax, "unterminated quoted attr"));
                }
            }
        } else {
            // bare value until whitespace or '>'
            let rest2 = &src[i..];
            let mut end_rel = rest2.find(' ').unwrap_or(rest2.len());
            if let Some(gt) = rest2.find('>') {
                end_rel = end_rel.min(gt);
            }
            let val = &src[i..i + end_rel];
            out.push(Attr { key, value: val.to_string() });
            i += end_rel;
            // trim following whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
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
                match &child.data.borrow().value {
                    NodeValue::Item(_li) => {
                        let task = detect_task_item(child);
                        let children_blocks = collect_blocks(child);
                        items.push(ListItem { children: children_blocks, task });
                    }
                    NodeValue::TaskItem(payload) => {
                        let s = format!("{:?}", payload);
                        let state = matches!(s.as_str(), "true" | "Some(true)" | "Some('x')" | "Some('X')");
                        let children_blocks = collect_blocks(child);
                        items.push(ListItem { children: children_blocks, task: Some(state) });
                    }
                    _ => {}
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
