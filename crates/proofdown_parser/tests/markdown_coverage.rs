use proofdown_ast::{Block, Inline, ListKind};
use proofdown_parser::parse;

#[test]
fn commonmark_blocks_and_inlines() {
    let md = r#"
# H1 with *em* and **strong** and `code`

> Quote
>
> - item 1
> - item 2

Some text with a [link](https://example.com "t") and ![alt](img.png).

---

```
code block
```

1. first
2. second
"#;
    let doc = parse(md).expect("parse ok");
    let b = &doc.blocks;
    assert!(matches!(b[0], Block::Heading { .. }));
    if let Block::Heading { inlines, .. } = &b[0] {
        // expect at least some inline nodes
        assert!(!inlines.is_empty());
        // there should be Text/Emph/Strong/Code among them (not asserting order strictly)
        assert!(inlines.iter().any(|i| matches!(i, Inline::Emph { .. })));
        assert!(inlines.iter().any(|i| matches!(i, Inline::Strong { .. })));
        assert!(inlines.iter().any(|i| matches!(i, Inline::Code { .. })));
    }
    assert!(matches!(b[1], Block::BlockQuote { .. }));
    assert!(matches!(b[2], Block::Paragraph { .. }));
    assert!(matches!(b[3], Block::ThematicBreak));
    assert!(matches!(b[4], Block::CodeBlock { .. }));
    assert!(matches!(b[5], Block::List { .. }));
    if let Block::List { kind, .. } = &b[5] {
        assert!(matches!(kind, ListKind::Ordered));
    }
}

#[test]
fn gfm_autolink_and_tasklist_and_table_alignments() {
    let md = r#"
Visit https://example.com for more info.

- [x] done item
- [ ] todo item

| Left | Center | Right |
|:-----|:------:|------:|
| a    |   b    |     c |
"#;
    let doc = parse(md).expect("parse ok");
    let mut saw_link = false;
    let mut saw_tasklist = false;
    let mut saw_table = false;
    for b in &doc.blocks {
        match b {
            Block::Paragraph { inlines } => {
                // Expect an autolink mapped as Inline::Link
                if inlines.iter().any(|i| matches!(i, Inline::Link { url, .. } if url == "https://example.com")) {
                    saw_link = true;
                }
            }
            Block::List { items, .. } => {
                // Expect GFM task flags on items
                if items.iter().any(|it| it.task == Some(true)) && items.iter().any(|it| it.task == Some(false)) {
                    saw_tasklist = true;
                }
            }
            Block::Table { align, header, rows } => {
                assert_eq!(align.len(), 3);
                assert!(matches!(align[0], proofdown_ast::TableAlign::Left));
                assert!(matches!(align[1], proofdown_ast::TableAlign::Center));
                assert!(matches!(align[2], proofdown_ast::TableAlign::Right));
                assert!(header.is_some());
                assert_eq!(rows.len(), 1);
                saw_table = true;
            }
            _ => {}
        }
    }
    assert!(saw_link, "expected autolink Inline::Link");
    assert!(saw_tasklist, "expected GFM task list mapping");
    assert!(saw_table, "expected GFM table mapping");
}

#[test]
fn ordered_list_with_non_one_start_has_start_field() {
    let md = r#"
3. third
4. fourth
"#;
    let doc = parse(md).expect("parse ok");
    let b = &doc.blocks;
    assert!(matches!(b[0], Block::List { .. }));
    if let Block::List { kind, start, .. } = &b[0] {
        assert!(matches!(kind, ListKind::Ordered));
        assert_eq!(*start, Some(3));
    }
}
