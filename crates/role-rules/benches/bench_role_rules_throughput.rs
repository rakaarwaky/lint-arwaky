// PURPOSE: Benchmark — measure role-rules checker throughput.
// Layer: Benchmark (performance validation).

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;

fn bench_checker_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("checker_instantiation");

    for n in [10, 100, 1000] {
        group.bench_with_input(BenchmarkId::new("instantiation", n), &n, |b, _| {
            b.iter(|| {
                for _ in 0..*n {
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
    c.bench_function("container_orchestration", |b| {
        b.iter(|| {
            let container = RoleContainer::new_with_config(
                shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
            );
            let _orch = container.orchestrator();
            let _agg = container.aggregate();
        });
    });
}

fn bench_all_checkers(c: &mut Criterion) {
    c.bench_function("all_checkers_single_source", |b| {
        b.iter(|| {
            black_box(AgentRoleChecker::new());
            black_box(CapabilitiesRoleChecker::new());
            black_box(ContractRoleChecker::new());
            black_box(SurfaceRoleChecker::new());
            black_box(TaxonomyRoleChecker::new());
            black_box(UtilityRoleChecker::new());
        });
    });
}

criterion_group!(benches, bench_checker_instantiation, bench_container_orchestration, bench_all_checkers);
criterion_main!(benches);
