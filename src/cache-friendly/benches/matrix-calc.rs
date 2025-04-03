use cache_friendly::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_access_patterns(c: &mut Criterion) {
    for &size in &[128, 1024, 4096, 8192] {
        let matrix = vec![vec![1u8; size]; size];

        c.bench_function(&format!("Row Major Access {}x{}", size, size), |b| {
            b.iter(|| row_major_call(black_box(&matrix)))
        });

        c.bench_function(&format!("Column Major Access {}x{}", size, size), |b| {
            b.iter(|| column_major_call(black_box(&matrix)))
        });
    }
}

criterion_group!(benches, benchmark_access_patterns);
criterion_main!(benches);