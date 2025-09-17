use anyhow::Result;
use proofdown_ast::{Attr, Block, Component, Document};

pub fn parse(input: &str) -> Result<Document> {
    // MVP: parse only headings (#..####), minimal components <grid>, <card>, and artifact.*
    // Anything else becomes a paragraph.
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut blocks = Vec::new();
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i] == b'\n' || bytes[i] == b'\r') { i += 1; }
        if i >= bytes.len() { break; }
        if bytes[i] == b'#' {
            let mut level = 0u8; let mut j = i;
            while j < bytes.len() && bytes[j] == b'#' && level < 4 { level += 1; j += 1; }
            if j < bytes.len() && bytes[j] == b' ' { j += 1; }
            let start = j; while j < bytes.len() && bytes[j] != b'\n' { j += 1; }
            let text = input[start..j].trim().to_string();
            blocks.push(Block::Heading { level, text });
            i = j + 1; continue;
        }
        if bytes[i] == b'<' {
            let (comp, used) = parse_component(&input[i..])?;
            blocks.push(Block::Component(comp));
            i += used; continue;
        }
        let start = i; while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'<' { i += 1; }
        let text = input[start..i].trim().to_string();
        if !text.is_empty() { blocks.push(Block::Paragraph(text)); }
        if i < bytes.len() && bytes[i] == b'\n' { i += 1; }
    }
    Ok(Document { blocks })
}

fn parse_component(src: &str) -> Result<(Component, usize)> {
    // Parse <name ...> ... </name> or <name ... />; MVP without nested error kinds
    if !src.starts_with('<') { anyhow::bail!("expected '<'"); }
    let gt = src.find('>').ok_or_else(|| anyhow::anyhow!("unterminated tag"))?;
    let mut head = src[1..gt].trim();
    let self_close = head.ends_with('/');
    if self_close { head = head.trim_end_matches('/').trim(); }
    let mut parts = head.split_whitespace();
    let name = parts.next().ok_or_else(|| anyhow::anyhow!("missing component name"))?.to_string();
    let rest = &head[name.len()..].trim();
    let attrs = parse_attrs(rest)?;
    let mut used = gt + 1;
    let mut children = Vec::new();
    if !self_close {
        let close_tag = format!("</{}>", name);
        let mut remain = &src[used..];
        while !remain.starts_with(&close_tag) {
            if remain.is_empty() { anyhow::bail!("unterminated component <{}>", name); }
            if remain.starts_with('\n') || remain.starts_with('\r') { used += 1; remain = &src[used..]; continue; }
            if remain.starts_with('<') {
                let (child, cused) = parse_component(remain)?;
                children.push(Block::Component(child)); used += cused; remain = &src[used..];
            } else if remain.starts_with('#') {
                let end = remain.find('\n').unwrap_or(remain.len());
                let line = &remain[..end];
                let mut level = 0u8; let mut idx = 0; for b in line.as_bytes() { if *b == b'#' && level < 4 { level += 1; idx += 1; } else { break; } }
                let text = line[idx..].trim().to_string(); children.push(Block::Heading { level, text }); used += end + 1; remain = &src[used..];
            } else {
                let mut end = remain.find('<').unwrap_or(remain.len()); if let Some(nl) = remain.find('\n') { end = end.min(nl); }
                let text = remain[..end].trim().to_string(); if !text.is_empty() { children.push(Block::Paragraph(text)); }
                used += end; remain = &src[used..];
            }
        }
        used += close_tag.len();
    }
    Ok((Component { name, attrs, children, self_closing: self_close }, used))
}

fn parse_attrs(mut src: &str) -> Result<Vec<Attr>> {
    let mut out = Vec::new();
    src = src.trim();
    while !src.is_empty() {
        let eq = match src.find('=') { Some(i) => i, None => break };
        let key = src[..eq].trim().to_string();
        src = &src[eq+1..];
        if src.starts_with('"') {
            src = &src[1..];
            let end = src.find('"').ok_or_else(|| anyhow::anyhow!("unterminated quoted attr"))?;
            out.push(Attr { key, value: src[..end].to_string() });
            src = &src[end+1..].trim_start();
        } else {
            let mut end = src.find(' ').unwrap_or(src.len());
            if let Some(gt) = src.find('>') { end = end.min(gt); }
            out.push(Attr { key, value: src[..end].trim_end_matches('/').to_string() });
            src = &src[end..].trim_start();
        }
    }
    Ok(out)
}

pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.key == key).map(|a| a.value.as_str())
}
