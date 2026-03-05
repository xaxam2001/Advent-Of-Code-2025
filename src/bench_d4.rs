use crate::d4::{d4p1_v1, d4p2_v1};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d4;

pub fn d4(c: &mut Criterion) {
    let mut group = c.benchmark_group("d4");
    group.bench_function("d4p1_v1", |b| {
        b.iter(|| d4p1_v1(black_box(include_str!("d4/d4.txt"))))
    });

    group.bench_function("d4p2_v1", |b| {
        b.iter(|| d4p2_v1(black_box(include_str!("d4/d4.txt"))))
    });
}

criterion_group!(benches, d4);
criterion_main!(benches);
