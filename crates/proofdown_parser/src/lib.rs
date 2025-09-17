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
use proofdown_ast::{Attr, Block, Component, Document, ErrorKind, ParseError};

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
    fn default() -> Self { Self { max_depth: MAX_DEPTH, max_nodes: MAX_NODES, max_input_bytes: MAX_INPUT_BYTES } }
}

pub fn parse(input: &str) -> PResult<Document> {
    parse_with_limits(input, ParserLimits::default())
}

pub fn parse_with_limits(input: &str, limits: ParserLimits) -> PResult<Document> {
    // MVP: parse only headings (#..####), minimal components <grid>, <card>, and artifact.*
    // Anything else becomes a paragraph.
    // Normalize newlines to \n for consistent scanning across platforms
    let normalized: std::borrow::Cow<'_, str> = if input.contains('\r') {
        std::borrow::Cow::Owned(input.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        std::borrow::Cow::Borrowed(input)
    };
    let input = &*normalized;
    if input.as_bytes().len() > limits.max_input_bytes {
        return Err(mk_err(input, 0, ErrorKind::LimitExceeded, format!(
            "input exceeds max size ({} bytes)", limits.max_input_bytes
        )));
    }
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut blocks = Vec::new();
    while i < bytes.len() {
      while i < bytes.len() && (bytes[i] == b'\n' || bytes[i] == b'\r') { i += 1; }
      if i >= bytes.len() { break; }
      if bytes[i] == b'#' {
          let mut level = 0u8; let mut j = i;
          while j < bytes.len() && bytes[j] == b'#' && level < 4 { level += 1; j += 1; }
          // Require a single space after hashes per grammar; otherwise treat as paragraph
          if j < bytes.len() && bytes[j] == b' ' {
              j += 1;
              let start = j; while j < bytes.len() && bytes[j] != b'\n' { j += 1; }
              let text = input[start..j].trim().to_string();
              if !text.is_empty() {
                  blocks.push(Block::Heading { level, text });
                  i = j + 1; continue;
              }
              // Empty title: fall through as paragraph
          }
          // Treat the entire line as paragraph
          let mut j2 = i; while j2 < bytes.len() && bytes[j2] != b'\n' { j2 += 1; }
          let line = input[i..j2].trim();
          if !line.is_empty() { blocks.push(Block::Paragraph { text: line.to_string() }); }
          i = j2 + (j2 < bytes.len()) as usize; continue;
      }
      if bytes[i] == b'<' {
          let (comp, used) = parse_component(input, i)?;
          blocks.push(Block::Component(comp));
          i += used; continue;
      }
      // Accumulate paragraph across consecutive non-empty lines
      let mut para = String::new();
      loop {
          // Stop if start-of-line is a component or heading
          if bytes[i] == b'<' || bytes[i] == b'#' { break; }
          // Read current line
          let mut j = i; while j < bytes.len() && bytes[j] != b'\n' { j += 1; }
          let line = input[i..j].trim();
          if line.is_empty() { i = j + (j < bytes.len()) as usize; break; }
          if !para.is_empty() { para.push(' '); }
          para.push_str(line);
          if j >= bytes.len() { i = j; break; }
          // Advance to start of next line
          i = j + 1;
          // Peek next line start; stop if EOF
          if i >= bytes.len() { break; }
          // If next line begins a component or heading, stop accumulating
          if bytes[i] == b'<' || bytes[i] == b'#' { break; }
      }
      if !para.is_empty() { blocks.push(Block::Paragraph { text: para }); }
  }
  let doc = Document { blocks };
    // Enforce depth and node limits post-parse (defaults for now; configurable in follow-up API)
    let (depth, nodes) = analyze(&doc);
    if depth > limits.max_depth {
        return Err(mk_err(input, 0, ErrorKind::LimitExceeded, format!(
            "depth exceeded: {} > {}", depth, limits.max_depth
        )));
    }
    if nodes > limits.max_nodes {
        return Err(mk_err(input, 0, ErrorKind::LimitExceeded, format!(
            "node limit exceeded: {} > {}", nodes, limits.max_nodes
        )));
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
    if self_close { head = head.trim_end_matches('/').trim(); }
    let mut parts = head.split_whitespace();
    let name = match parts.next() {
        Some(n) => n.to_string(),
        None => return Err(mk_err(full, base + 1, ErrorKind::Syntax, "missing component name")),
    };
    let rest = &head[name.len()..].trim();
    let attrs = parse_attrs(rest)?;
    let mut used = gt + 1; // relative to base
    let mut children = Vec::new();
    if !self_close {
        let close_tag = format!("</{}>", name);
        loop {
            let remain = &full[base + used..];
            if remain.starts_with(&close_tag) { break; }
            if remain.is_empty() {
                return Err(mk_err(full, base + used, ErrorKind::Syntax, format!(
                    "unterminated component <{}>", name
                )));
            }
            if remain.starts_with('\n') || remain.starts_with('\r') { used += 1; continue; }
            if remain.starts_with("</") {
                // Unexpected close tag (mismatch)
                let name_end = remain[2..].find('>').unwrap_or(remain.len()-2);
                let close_name = &remain[2..2+name_end].trim();
                if &format!("</{}>", name) == &format!("</{}>", close_name) {
                    // matched above in starts_with(check), unreachable here; keep guard
                    break;
                } else {
                    return Err(mk_err(full, base + used, ErrorKind::Syntax, format!(
                        "unexpected close </{}>", close_name
                    )));
                }
            } else if remain.starts_with('<') {
                let (child, cused) = parse_component(full, base + used)?;
                children.push(Block::Component(child)); used += cused;
            } else if remain.starts_with('#') {
                let end = remain.find('\n').unwrap_or(remain.len());
                let line = &remain[..end];
                let mut level = 0u8; let mut idx = 0; for b in line.as_bytes() { if *b == b'#' && level < 4 { level += 1; idx += 1; } else { break; } }
                if idx < line.len() && line.as_bytes()[idx] == b' ' {
                    let text = line[idx+1..].trim();
                    if !text.is_empty() { children.push(Block::Heading { level, text: text.to_string() }); } else { children.push(Block::Paragraph { text: line.trim().to_string() }); }
                } else {
                    // Not a valid heading; keep as paragraph line
                    children.push(Block::Paragraph { text: line.trim().to_string() });
                }
                used += end + 1;
            } else {
                let mut end = remain.find('<').unwrap_or(remain.len()); if let Some(nl) = remain.find('\n') { end = end.min(nl); }
                let text = remain[..end].trim().to_string(); if !text.is_empty() { children.push(Block::Paragraph { text }); }
                used += end;
            }
        }
        used += close_tag.len();
    }
    Ok((Component { name, attrs, children, self_closing: self_close }, used))
}

fn parse_attrs(mut src: &str) -> PResult<Vec<Attr>> {
    let mut out = Vec::new();
    src = src.trim();
    while !src.is_empty() {
        let eq = match src.find('=') { Some(i) => i, None => {
            // if trailing non-whitespace remains, it's a malformed attribute token
            let rest = src.trim();
            if !rest.is_empty() { return Err(ParseError { line: 1, col: 1, kind: ErrorKind::Syntax, msg: format!("malformed attribute near '{}': expected key=value", rest) }); }
            break
        } };
        let key = src[..eq].trim().to_string();
        src = &src[eq+1..];
        if src.starts_with('"') {
            src = &src[1..];
            let end = match src.find('"') { Some(i) => i, None => return Err(ParseError { line: 1, col: 1, kind: ErrorKind::Syntax, msg: "unterminated quoted attr".into() }) };
            out.push(Attr { key, value: src[..end].to_string() });
            src = &src[end+1..].trim_start();
        } else {
            let mut end = src.find(' ').unwrap_or(src.len());
            if let Some(gt) = src.find('>') { end = end.min(gt); }
            // Preserve bare values exactly up to whitespace or '>'
            out.push(Attr { key, value: src[..end].to_string() });
            src = &src[end..].trim_start();
        }
    }
    Ok(out)
}

pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.key == key).map(|a| a.value.as_str())
}

fn analyze(doc: &Document) -> (usize, usize) {
    fn walk(blocks: &[Block], cur_depth: usize, out_depth: &mut usize, out_nodes: &mut usize) {
        *out_depth = (*out_depth).max(cur_depth);
        for b in blocks {
            *out_nodes += 1;
            if let Block::Component(c) = b {
                if !c.children.is_empty() {
                    walk(&c.children, cur_depth + 1, out_depth, out_nodes);
                }
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
    ParseError { line, col, kind, msg: msg.into() }
}

fn pos_to_line_col(s: &str, index: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, ch) in s.char_indices() {
        if i >= index { break; }
        if ch == '\n' { line += 1; col = 1; } else { col += 1; }
    }
    (line, col)
}
