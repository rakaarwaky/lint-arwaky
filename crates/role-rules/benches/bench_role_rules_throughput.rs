// PURPOSE: Benchmark — measure role-rules checker throughput.
// Layer: Benchmark (performance validation).
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;

fn bench_checker_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("checker_instantiation");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [10usize, 100, 1000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instantiation", n), &n, |b, val| {
            b.iter(|| {
                let count = *val;
                for _ in 0..count {
                    let _agent = AgentRoleChecker::new();
                    let _capabilities = CapabilitiesRoleChecker::new();
                    let _contract = ContractRoleChecker::new();
                    let _surface = SurfaceRoleChecker::new();
                    let _taxonomy = TaxonomyRoleChecker::new();
                    let _utility = UtilityRoleChecker::new();
                }
            });
        });
    }
    group.finish();
}

fn bench_container_orchestration(c: &mut Criterion) {
    let mut group = c.benchmark_group("container_orchestration");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("instances", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    let container = RoleContainer::new_with_config(
                        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
                    );
                    black_box((container.orchestrator(), container.aggregate()));
                }
            });
        });
    }
    group.finish();
}

fn bench_all_checkers(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_checkers");
    group.sample_size(30);

    for n in [1, 10, 100] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("runs", n), &n, |b, val| {
            let count = *val;
            b.iter(|| {
                for _ in 0..count {
                    black_box(AgentRoleChecker::new());
                    black_box(CapabilitiesRoleChecker::new());
                    black_box(ContractRoleChecker::new());
                    black_box(SurfaceRoleChecker::new());
                    black_box(TaxonomyRoleChecker::new());
                    black_box(UtilityRoleChecker::new());
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_checker_instantiation,
    bench_container_orchestration,
    bench_all_checkers
);
criterion_main!(benches);
