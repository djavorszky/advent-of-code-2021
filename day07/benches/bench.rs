use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day07::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    let mut data: Vec<i32> = input
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    c.bench_function("day07 task_1", |b| b.iter(|| task_1(black_box(&mut data))));

    let mut data: Vec<i32> = input
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    c.bench_function("day07 task_2", |b| b.iter(|| task_2(black_box(&mut data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
