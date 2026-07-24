// PURPOSE: Benchmark — measure project-setup component throughput.
// Layer: Benchmark (performance validation).
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;

fn bench_container_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup_container");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    black_box(SetupContainer::new());
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_container_instantiation);
criterion_main!(benches);
