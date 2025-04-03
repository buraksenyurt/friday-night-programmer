use cache_friendly::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_access_patterns(c: &mut Criterion) {
    let matrix = [[1u8; MAX_SIZE]; MAX_SIZE];

    c.bench_function("Row Major Access", |b| {
        b.iter(|| row_major_call(black_box(&matrix)))
    });

    c.bench_function("Column Major Access", |b| {
        b.iter(|| column_major_call(black_box(&matrix)))
    });
}

criterion_group!(benches, benchmark_access_patterns);
criterion_main!(benches);
