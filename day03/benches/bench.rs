use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day03::*;

fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    c.bench_function("day03 task_1", |b| b.iter(|| task_1(black_box(&data))));
    c.bench_function("day03 task_2", |b| b.iter(|| task_2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
