

use criterion::{criterion_group, criterion_main, Criterion};

use std::fs;
use sixth::sixth::second;
use sixth::sixth::second_faster;


pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sixth");
    let data = fs::read_to_string("data").expect("Oops");

    group.sample_size(5000);
    // c.bench_function("first", |b| b.iter(|| first("data", 80)));
    group.bench_function("second", |b| b.iter(|| second(&data, 2048)));
    group.bench_function("second_faster", |b| b.iter(|| second_faster(&data, 2048)));


    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
