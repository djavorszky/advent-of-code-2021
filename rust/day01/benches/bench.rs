use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day01::*;

fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    c.bench_function("day01 task_1", |b| b.iter(|| task_1(black_box(&data))));
    c.bench_function("day01 task_1_fold", |b| {
        b.iter(|| task_1_fold(black_box(&data)))
    });
    c.bench_function("day01 task_2", |b| b.iter(|| task_2(black_box(&data))));
    c.bench_function("day01 task_2_fold", |b| {
        b.iter(|| task_2_fold(black_box(&data)))
    });
    c.bench_function("day01 task_2_alternative", |b| {
        b.iter(|| task_2_alternative(black_box(&data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
