use criterion::{criterion_group, criterion_main, Criterion};
mod d2;

pub fn d2(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d2p1");
        group.bench_function("d2p2_v1",
                             |b| b.iter(|| d2::d2p1_v1(include_str!("d2/d2.txt"))));
    }
    // group.bench_function("d1p1_v2",
    //                      |b | b.iter(||d1::d1p1_v2(include_bytes!("d1/d1.txt"))));

    {
        let mut group = c.benchmark_group("d2p2");
        group.bench_function("d2p2_v1",
                             |b| b.iter(|| d2::d2p2_v1(include_str!("d2/d2.txt"))));
    }
}

criterion_group!(benches, d2);
criterion_main!(benches);