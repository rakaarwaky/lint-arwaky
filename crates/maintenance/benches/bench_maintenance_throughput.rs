// PURPOSE: Benchmark tests for maintenance operations — component instantiation throughput.
// Layer: Benchmark (criterion, runs at release gate / nightly).

use criterion::{criterion_group, criterion_main, Criterion};
use maintenance_lint_arwaky::agent_maintenance_orchestrator::{
    MaintenanceCommandsOrchestrator, MaintenanceDeps,
};
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use std::sync::Arc;

fn bench_orchestrator_instantiation(c: &mut Criterion) {
    c.bench_function("maintenance_orchestrator", |b| {
        b.iter(|| {
            let checker = Arc::new(MaintenanceChecker::new());
            MaintenanceCommandsOrchestrator::new(MaintenanceDeps { checker })
        });
    });
}

fn bench_checker_instantiation(c: &mut Criterion) {
    c.bench_function("maintenance_checker", |b| b.iter(MaintenanceChecker::new));
}

criterion_group!(
    benches,
    bench_orchestrator_instantiation,
    bench_checker_instantiation
);
criterion_main!(benches);
