use criterion::{criterion_group, criterion_main, Criterion};
mod d1;

pub fn d1(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d1p1");
        group.bench_function("d1p1_v1",
                             |b| b.iter(|| d1::d1p1_v1(include_str!("d1/d1.txt"))));
        group.bench_function("d1p1_v2",
                             |b | b.iter(||d1::d1p1_v2(include_str!("d1/d1.txt"))));
        group.bench_function("d1p1_v3",
                             |b | b.iter(||d1::d1p1_v3(include_str!("d1/d1.txt"))));
        group.bench_function("d1p1_v4",
                             |b | b.iter(||d1::d1p1_v4(include_str!("d1/d1.txt"))));
        group.bench_function("d1p1_v5",
                             |b | b.iter(||d1::d1p1_v5(include_str!("d1/d1.txt"))));
        group.bench_function("d1p1_v6",
                             |b | b.iter(||d1::d1p1_v6(include_str!("d1/d1.txt"))));
        group.bench_function("d1p1_v7",
                             |b | b.iter(||d1::d1p1_v7(include_str!("d1/d1.txt"))));
    }

    // long file benchmark
    // {
    //     let mut group = c.benchmark_group("d1p1_long");
    //     group.bench_function("d1p1_v1",
    //                          |b| b.iter(|| d1::d1p1_v1(include_str!("d1/d1_long.txt"))));
    //     group.bench_function("d1p1_v2",
    //                          |b | b.iter(||d1::d1p1_v2(include_str!("d1/d1_long.txt"))));
    //     group.bench_function("d1p1_v3",
    //                          |b | b.iter(||d1::d1p1_v3(include_str!("d1/d1_long.txt"))));
    // }

    {
        let mut group = c.benchmark_group("d1p2");
        group.bench_function("d1p2_v1",
                             |b| b.iter(|| d1::d1p2_v1(include_str!("d1/d1.txt"))));
        group.bench_function("d1p2_v2",
                             |b| b.iter(|| d1::d1p2_v2(include_str!("d1/d1.txt"))));
        group.bench_function("d1p2_v3",
                             |b| b.iter(|| d1::d1p2_v3(include_str!("d1/d1.txt"))));
        group.bench_function("d1p2_v4",
                             |b| b.iter(|| d1::d1p2_v4(include_str!("d1/d1.txt"))));
    }
}

criterion_group!(benches, d1);
criterion_main!(benches);