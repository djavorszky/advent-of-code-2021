use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day08::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    let data: Vec<&str> = input.lines().collect();

    c.bench_function("day08 task_1", |b| b.iter(|| task_1(black_box(&data))));
    c.bench_function("day08 task_2", |b| b.iter(|| task_2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
