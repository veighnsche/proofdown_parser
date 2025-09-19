use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proofdown_ast::{Block, Document, Inline, ListItem, ListKind, TableAlign, TableRow};

fn sample_document() -> Document {
    Document {
        blocks: vec![
            Block::Heading { level: 1, inlines: vec![Inline::Text { text: "Title".into() }] },
            Block::Paragraph { inlines: vec![Inline::Text { text: "Hello world".into() }] },
            Block::List {
                kind: ListKind::Ordered,
                start: Some(3),
                tight: true,
                items: vec![
                    ListItem { children: vec![Block::Paragraph { inlines: vec![Inline::Text { text: "A".into() }] }], task: None },
                    ListItem { children: vec![Block::Paragraph { inlines: vec![Inline::Text { text: "B".into() }] }], task: Some(true) },
                ],
            },
            Block::Table {
                align: vec![TableAlign::Left, TableAlign::Center, TableAlign::Right],
                header: Some(TableRow { cells: vec![
                    vec![Inline::Text { text: "L".into() }],
                    vec![Inline::Text { text: "C".into() }],
                    vec![Inline::Text { text: "R".into() }],
                ]}),
                rows: vec![
                    TableRow { cells: vec![
                        vec![Inline::Text { text: "a".into() }],
                        vec![Inline::Text { text: "b".into() }],
                        vec![Inline::Text { text: "c".into() }],
                    ]},
                ],
            },
        ],
    }
}

fn bench_serde(c: &mut Criterion) {
    let doc = sample_document();
    c.bench_function("serialize_document", |b| {
        b.iter(|| {
            let s = serde_json::to_string(black_box(&doc)).unwrap();
            black_box(s);
        })
    });
    let json = serde_json::to_string(&doc).unwrap();
    c.bench_function("deserialize_document", |b| {
        b.iter(|| {
            let d: Document = serde_json::from_str(black_box(&json)).unwrap();
            black_box(d);
        })
    });
}

criterion_group!(benches, bench_serde);
criterion_main!(benches);
