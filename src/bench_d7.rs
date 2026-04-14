use criterion::{criterion_group, criterion_main, Criterion};
mod d7;

pub fn d7(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("d7p1");
        group.bench_function("d7p1_v1",
                             |b| b.iter(|| d7::d7p1_v1(include_str!("d7/d7.txt"))));
    }
    {
        let mut group = c.benchmark_group("d7p2");
        group.bench_function("d7p2_v1",
                             |b| b.iter(|| d7::d7p2_v1(include_str!("d7/d7.txt"))));
    }
}

criterion_group!(benches, d7);
criterion_main!(benches);