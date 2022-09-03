use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use desim_mpmc::simulation;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("desim_mpmc");
    for limit in [10000.0, 20000.0, 30000.0, 40000.0, 50000.0] {
        group.bench_with_input(BenchmarkId::from_parameter(limit), &limit, |b, &limit| {
            b.iter(|| simulation(black_box(limit), black_box(3), black_box(4)));
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(100));
    targets = bench
);
criterion_main!(benches);
