use criterion::{black_box, criterion_group, criterion_main, Criterion};

use testing::sploosh;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh 8 9 10", |b| b.iter(|| sploosh(black_box(8), black_box(9), black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);