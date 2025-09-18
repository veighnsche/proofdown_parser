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
