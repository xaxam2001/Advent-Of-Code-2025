use criterion::{criterion_group, criterion_main, Criterion};
mod d3;

pub fn d3(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d3p1");
        group.bench_function("d3p1_v1",
                             |b| b.iter(|| d3::d3p1_v1(include_str!("d3/d3.txt"))));
        group.bench_function("d3p1_v2",
                             |b| b.iter(|| d3::d3p1_v2(include_str!("d3/d3.txt"))));
        group.bench_function("d3p1_v3",
                             |b| b.iter(|| d3::d3p1_v3(include_str!("d3/d3.txt"))));
        group.bench_function("d3p1_v4",
                             |b| b.iter(|| d3::d3p1_v4(include_str!("d3/d3.txt"))));
        group.bench_function("d3p1_v5",
                             |b| b.iter(|| d3::d3p1_v5(include_str!("d3/d3.txt"))));
        group.bench_function("d3p1_v6",
                             |b| b.iter(|| d3::d3p1_v6(include_str!("d3/d3.txt"))));
    }

    {
        let mut group = c.benchmark_group("d3p2");
        group.bench_function("d3p2_v1",
                             |b| b.iter(|| d3::d3p2_v1(include_str!("d3/d3.txt"))));
    }
}

criterion_group!(benches, d3);
criterion_main!(benches);