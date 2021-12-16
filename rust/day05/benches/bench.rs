use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day05::*;

fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![];

    c.bench_function("day05 task_1", |b| {
        b.iter(|| task_1(black_box(&data), black_box(991)))
    });
    c.bench_function("day05 task_2", |b| {
        b.iter(|| task_2(black_box(&data), black_box(991)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
