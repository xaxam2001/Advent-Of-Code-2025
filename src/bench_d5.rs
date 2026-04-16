use crate::d5::{d5p1_v1, d5p1_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d5;

pub fn d5(c: &mut Criterion) {
    let mut group = c.benchmark_group("d5");
    group.bench_function("d5p1_v1", |b| {
        b.iter(|| d5p1_v1(black_box(include_str!("d5/d5.txt"))))
    });

    group.bench_function("d5p1_v2", |b| {
        b.iter(|| d5p1_v2(black_box(include_str!("d5/d5.txt"))))
    });
}

criterion_group!(benches, d5);
criterion_main!(benches);
