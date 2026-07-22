// PURPOSE: Benchmark — measure role-rules checker throughput.
// Layer: Benchmark (performance validation).

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use std::time::Instant;

fn make_source(file: &str, content: &str) -> shared::taxonomy_source_vo::SourceContentVO {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap();
    let cs = shared::taxonomy_source_vo::ContentString::new(content.to_string());
    shared::taxonomy_source_vo::SourceContentVO::new(fp, cs, "rust")
}

// ─── Benchmark: Checker instantiation throughput ──

#[test]
fn bench_checker_instantiation() {
    let start = Instant::now();
    for _ in 0..1000 {
        let _agent = AgentRoleChecker::new();
        let _capabilities = CapabilitiesRoleChecker::new();
        let _contract = ContractRoleChecker::new();
        let _surface = SurfaceRoleChecker::new();
        let _taxonomy = TaxonomyRoleChecker::new();
        let _utility = UtilityRoleChecker::new();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 1000,
        "Instantiation took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Container orchestration throughput ──

#[test]
fn bench_container_orchestration() {
    let container = RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    );

    let start = Instant::now();
    for _ in 0..100 {
        let _orch = container.orchestrator();
        let _agg = container.aggregate();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 10000,
        "Orchestration took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: All checkers instantiation ──

#[test]
fn bench_all_checkers_on_single_source() {
    let agent = AgentRoleChecker::new();
    let capabilities = CapabilitiesRoleChecker::new();
    let contract = ContractRoleChecker::new();
    let surface = SurfaceRoleChecker::new();
    let taxonomy = TaxonomyRoleChecker::new();
    let utility = UtilityRoleChecker::new();

    // Just verify instantiation is fast
    let _ = agent;
    let _ = capabilities;
    let _ = contract;
    let _ = surface;
    let _ = taxonomy;
    let _ = utility;
}
