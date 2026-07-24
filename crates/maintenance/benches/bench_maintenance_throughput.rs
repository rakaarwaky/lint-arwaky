// PURPOSE: Benchmark tests for maintenance operations — component instantiation throughput.
// Layer: Benchmark (criterion, runs at release gate / nightly).
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use maintenance_lint_arwaky::agent_maintenance_orchestrator::{
    MaintenanceCommandsOrchestrator, MaintenanceDeps,
};
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use std::sync::Arc;

fn bench_orchestrator_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("orchestrator_instantiation");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    let checker = Arc::new(MaintenanceChecker::new());
                    black_box(MaintenanceCommandsOrchestrator::new(MaintenanceDeps {
                        checker,
                    }));
                }
            });
        });
    }
    group.finish();
}

fn bench_checker_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("checker_instantiation");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    black_box(MaintenanceChecker::new());
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_orchestrator_instantiation,
    bench_checker_instantiation
);
criterion_main!(benches);
