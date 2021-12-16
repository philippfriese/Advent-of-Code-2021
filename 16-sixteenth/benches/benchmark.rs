use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};

use std::fs;

use fourteenth::fourteenth::first;
use fourteenth::fourteenth::second;
use fourteenth::fourteenth::run_01;
use fourteenth::fourteenth::run_02;


pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fourteenth");
    let data = fs::read_to_string("data").expect("Oops");

    group.sample_size(1000);
    group.measurement_time(Duration::from_secs(5));

    // group.bench_function("first", |b| b.iter(|| first(&data, 10)));
    group.bench_function("second", |b| b.iter(|| second(&data, 40)));

    // group.bench_function("run_01", |b| b.iter(|| run_01(&data)));
    group.bench_function("run_02", |b| b.iter(|| run_02(&data)));


    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
