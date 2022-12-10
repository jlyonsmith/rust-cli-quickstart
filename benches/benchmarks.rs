use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_cli_quickstart::*;

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib #1 basic 35", |b| b.iter(|| fibonacci_1(black_box(35))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
