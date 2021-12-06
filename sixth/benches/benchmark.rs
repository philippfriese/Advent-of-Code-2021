use criterion::{criterion_group, criterion_main, Criterion};

use sixth::sixth::first;
use sixth::sixth::second;


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("first", |b| b.iter(|| first("data", 80)));
    c.bench_function("second", |b| b.iter(|| second("data", 256)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
