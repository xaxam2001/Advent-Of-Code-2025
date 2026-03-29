use criterion::{criterion_group, criterion_main, Criterion};
mod d5;

pub fn d5(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d5p1");
        group.bench_function("d5p1_v1",
                             |b| b.iter(|| d5::d5p1_v1(include_str!("d5/d5.txt"))));
    }
    // group.bench_function("d1p1_v2",
    //                      |b | b.iter(||d1::d1p1_v2(include_bytes!("d1/d1.txt"))));

    {
        let mut group = c.benchmark_group("d5p2");
        group.bench_function("d5p2_v1",
                             |b| b.iter(|| d5::d5p2_v1(include_str!("d5/d5.txt"))));
    }
}

criterion_group!(benches, d5);
criterion_main!(benches);