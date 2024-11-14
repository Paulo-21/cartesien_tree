use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use cartesien_tree::insert_tonnes;
pub fn criterion_benchmark(c: &mut Criterion) {        
    c.bench_function("bench_tree", |b| b.iter(|| insert_tonnes(black_box( 1000))));
    c.bench_function("bench_tree", |b| b.iter(|| insert_tonnes(black_box( 10000))));
    c.bench_function("bench_tree", |b| b.iter(|| insert_tonnes(black_box( 100000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
