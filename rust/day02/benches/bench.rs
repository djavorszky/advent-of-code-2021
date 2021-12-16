use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day02::*;

fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    c.bench_function("day02 task_1", |b| b.iter(|| task_1(black_box(&data))));
    c.bench_function("day02 task_2", |b| b.iter(|| task_2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
