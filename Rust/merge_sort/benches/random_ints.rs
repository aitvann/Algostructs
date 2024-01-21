use merge_sort::sort;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Standard, Rng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("random-ints");
    for size in 1..=10 {
        let size = size * 100_000;
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let rng = rand::thread_rng();
            let mut data: Vec<i32> = rng.sample_iter(Standard).take(size).collect();
            b.iter(|| sort(&mut data));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
