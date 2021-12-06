use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day06::*;

fn criterion_benchmark(c: &mut Criterion) {
    let data: Vec<u8> = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    c.bench_function("day06 task_1", |b| b.iter(|| task_1(black_box(&data))));
    // c.bench_function("day06 task_2", |b| b.iter(|| task_2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
