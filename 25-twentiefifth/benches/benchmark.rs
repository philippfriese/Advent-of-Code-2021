use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};

use std::fs;

use twentiefirst::twentiefirst::first;
use twentiefirst::twentiefirst::second;


pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("twentiefirst");
    let data = fs::read_to_string("data").expect("Oops");

    group.sample_size(1000);
    group.measurement_time(Duration::from_secs(5));

    // group.bench_function("first", |b| b.iter(|| first(&data)));
    group.bench_function("second", |b| b.iter(|| second(&data)));



    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
