// PURPOSE: Benchmark — measure project-setup component throughput.
// Layer: Benchmark (performance validation).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;

fn bench_container_instantiation(c: &mut Criterion) {
    c.bench_function("setup_container", |b| {
        b.iter(|| SetupContainer::new());
    });
}

fn bench_orchestrator_creation(c: &mut Criterion) {
    c.bench_function("orchestrator_creation", |b| {
        b.iter(|| {
            let installer = SetupInstallerAdapter::new();
            let processor = SetupManagementProcessor::new(installer);
            let _orch = project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator::new(black_box(processor));
        });
    });
}

criterion_group!(benches, bench_container_instantiation, bench_orchestrator_creation);
criterion_main!(benches);
