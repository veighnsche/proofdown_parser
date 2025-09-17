use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parse_minimal(c: &mut Criterion) {
    let pml = include_str!("../tests/fixtures/minimal.pml");
    c.bench_function("parse_minimal", |b| {
        b.iter(|| {
            let doc = proofdown_parser::parse(black_box(pml)).expect("parse ok");
            black_box(doc);
        })
    });
}

fn bench_parse_v2_minimal(c: &mut Criterion) {
    let pml = include_str!("../tests/fixtures/v2_minimal.pml");
    c.bench_function("parse_v2_minimal", |b| {
        b.iter(|| {
            let doc = proofdown_parser::parse(black_box(pml)).expect("parse ok");
            black_box(doc);
        })
    });
}

criterion_group!(benches, bench_parse_minimal, bench_parse_v2_minimal);
criterion_main!(benches);
