use criterion::{criterion_group, criterion_main, Criterion};
mod d1;

pub fn d1(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d1p1");
        group.bench_function("d1p1_v1",
                             |b| b.iter(|| d1::d1p1_v1(include_str!("d1/d1.txt"))));
    }
    // group.bench_function("d1p1_v2",
    //                      |b | b.iter(||d1::d1p1_v2(include_bytes!("d1/d1.txt"))));

    {
        let mut group = c.benchmark_group("d1p2");
        group.bench_function("d1p2_v1",
                             |b| b.iter(|| d1::d1p2_v1(include_str!("d1/d1.txt"))));
    }
}

criterion_group!(benches, d1);
criterion_main!(benches);