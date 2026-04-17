use crate::d3::{d3p1_v1, d3p1_v2, d3p1_v3, d3p1_v4, d3p2_v1};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d3;

pub fn d3(c: &mut Criterion) {
    let mut group = c.benchmark_group("d3");
    group.bench_function("d3p1_v1", |b| {
        b.iter(|| d3p1_v1(black_box(include_str!("d3/d3.txt"))))
    });

    group.bench_function("d3p1_v2", |b| {
        b.iter(|| d3p1_v2(black_box(include_str!("d3/d3.txt"))))
    });

    group.bench_function("d3p1_v3", |b| {
        b.iter(|| d3p1_v2(black_box(include_str!("d3/d3.txt"))))
    });
    group.bench_function("d3p1_v4", |b| {
        b.iter(|| d3p1_v4(black_box(include_str!("d3/d3.txt"))))
    });
    group.bench_function("d3p2_v1", |b| {
        b.iter(|| d3p2_v1(black_box(include_str!("d3/d3.txt"))))
    });
}

criterion_group!(benches, d3);
criterion_main!(benches);
