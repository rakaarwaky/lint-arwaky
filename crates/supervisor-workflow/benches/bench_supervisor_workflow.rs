// PURPOSE: Benchmark for the supervisor workflow — full cycle throughput.
use criterion::{Criterion, criterion_group, criterion_main};
use shared_common::taxonomy_common_vo::Count;
use supervisor_workflow_lint_arwaky::root_supervisor_container::SupervisorContainer;

fn bench_supervisor_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("supervisor_cycle");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(10);

    group.bench_function("cycle_5_issues", |b| {
        b.iter(|| {
            let container = SupervisorContainer::new();
            std::hint::black_box(container.run_supervisor_cycle(Count::new(5)));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_supervisor_cycle,);
criterion_main!(benches);
