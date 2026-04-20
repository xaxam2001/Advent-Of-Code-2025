use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
mod d8;

pub fn d8(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d8p1");
        group.sample_size(10);
        group.bench_function("d8p1_v1",
                             |b| b.iter(|| d8::d8p1_v1(include_str!("d8/d8.txt"), 1000)));
        group.sample_size(100);
        group.bench_function("d8p1_v2",
                             |b| b.iter(|| d8::d8p1_v2(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p1_v3",
                             |b| b.iter(|| d8::d8p1_v3(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p1_v4",
                             |b| b.iter(|| d8::d8p1_v4(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p1_v5",
                             |b| b.iter(|| d8::d8p1_v5(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p1_v6",
                             |b| b.iter(|| d8::d8p1_v6(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p1_v7",
                             |b| b.iter(|| d8::d8p1_v7(include_str!("d8/d8.txt"), 1000)));
    }
    {
        let mut group = c.benchmark_group("d8p2");
        group.sample_size(10);
        group.bench_function("d8p2_v1",
                             |b| b.iter(|| d8::d8p2_v1(include_str!("d8/d8.txt"))))
            .sample_size(25);
    }
}

criterion_group!(benches, d8);
criterion_main!(benches);