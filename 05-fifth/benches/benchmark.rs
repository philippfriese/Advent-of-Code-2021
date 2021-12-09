use criterion::{criterion_group, criterion_main, Criterion};

use fifth::fifth::first;
use fifth::fifth::second;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("first", |b| b.iter(|| first("data")));
    c.bench_function("second", |b| b.iter(|| second("data")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
