// PURPOSE: Benchmark — measure project-setup component throughput.
// Layer: Benchmark (performance validation).

use criterion::{criterion_group, criterion_main, Criterion};
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;

fn bench_container_instantiation(c: &mut Criterion) {
    c.bench_function("setup_container", |b| {
        b.iter(|| SetupContainer::new());
    });
}

criterion_group!(benches, bench_container_instantiation);
criterion_main!(benches);
