use criterion::{criterion_group, criterion_main, Criterion};
mod d6;

pub fn d6(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d6p1");
        group.bench_function("d6p1_v1",
                             |b| b.iter(|| d6::d6p1_v1(include_str!("d6/d6.txt"))));
    }
    // group.bench_function("d1p1_v2",
    //                      |b | b.iter(||d1::d1p1_v2(include_bytes!("d1/d1.txt"))));

    {
        let mut group = c.benchmark_group("d6p2");
        group.bench_function("d6p2_v1",
                             |b| b.iter(|| d6::d6p2_v1(include_str!("d6/d6.txt"))));
    }
}

criterion_group!(benches, d6);
criterion_main!(benches);