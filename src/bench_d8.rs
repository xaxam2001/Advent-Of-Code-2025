use criterion::{criterion_group, criterion_main, Criterion};
mod d8;

pub fn d8(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d8p1");
        group.bench_function("d8p1_v1",
                             |b| b.iter(|| d8::d8p1_v1(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p2_v2",
                             |b| b.iter(|| d8::d8p1_v2(include_str!("d8/d8.txt"), 1000)));
        group.bench_function("d8p2_v3",
                             |b| b.iter(|| d8::d8p1_v3(include_str!("d8/d8.txt"), 1000)));
    }
    {
        let mut group = c.benchmark_group("d8p2");
        group.bench_function("d8p2_v1",
                             |b| b.iter(|| d8::d8p2_v1(include_str!("d8/d8.txt"))));
    }
}

criterion_group!(benches, d8);
criterion_main!(benches);