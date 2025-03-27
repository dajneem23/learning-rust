use criterion::{criterion_group, criterion_main, Criterion};
use microservices_pipeline::optimized_sum;

fn bench_optimized_sum(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).collect();
    c.bench_function("optimized_sum", |b| {
        b.iter(|| {
            let _ = optimized_sum(&data);
        })
    });
}

criterion_group!(benches, bench_optimized_sum);
criterion_main!(benches);