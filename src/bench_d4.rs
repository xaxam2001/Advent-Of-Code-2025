use criterion::{criterion_group, criterion_main, Criterion};
mod d4;

pub fn d4(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d4p1");
        group.bench_function("d4p2_v1",
                             |b| b.iter(|| d4::d4p1_v1(include_str!("d4/d4.txt"))));
    }
    // group.bench_function("d1p1_v2",
    //                      |b | b.iter(||d1::d1p1_v2(include_bytes!("d1/d1.txt"))));

    {
        let mut group = c.benchmark_group("d4p2");
        group.bench_function("d4p2_v1",
                             |b| b.iter(|| d4::d4p2_v1(include_str!("d4/d4.txt"))));
    }
}

criterion_group!(benches, d4);
criterion_main!(benches);