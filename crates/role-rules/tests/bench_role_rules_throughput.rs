// PURPOSE: Benchmark — measure role-rules checker throughput.
// Layer: Benchmark (performance validation).

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
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
    let start = Instant::new();
    for _ in 0..1000 {
        let _agent = AgentRoleChecker::new();
        let _capabilities = CapabilitiesRoleChecker::new();
        let _contract = ContractRoleChecker::new();
        let _surface = SurfaceRoleChecker::new();
        let _taxonomy = TaxonomyRoleChecker::new();
        let _utility = UtilityRoleChecker::new();
    }
    let elapsed = start.elapsed();
    // Should complete within 1 second (1000 instantiations)
    assert!(elapsed.as_millis() < 1000, "Instantiation took {}ms", elapsed.as_millis());
}

// ─── Benchmark: Single checker throughput ──

#[test]
fn bench_single_checker_performace() {
    let agent = AgentRoleChecker::new();
    let source = make_source("bench_test.rs", "pub struct Foo;");
    let mut violations = Vec::new();

    let start = Instant::now();
    for _ in 0..10000 {
        violations.clear();
        agent.check_container(&source, &mut violations);
    }
    let elapsed = start.elapsed();
    // 10000 checks should complete within 5 seconds
    assert!(elapsed.as_millis() < 5000, "10000 checks took {}ms", elapsed.as_millis());
}

// ─── Benchmark: Container orchestration throughput ──

#[test]
fn bench_container_orchestration() {
    let container = RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    );
    let orch = container.orchestrator();
    let source = make_source("bench_test.rs", "pub struct Foo;");

    let start = Instant::now();
    for _ in 0..100 {
        let mut violations = Vec::new();
        orch.run_audit(&FilePath::new("/tmp".to_string()).unwrap());
    }
    let elapsed = start.elapsed();
    // 100 orchestration calls should complete within 10 seconds
    assert!(elapsed.as_millis() < 10000, "Orchestration took {}ms", elapsed.as_millis());
}

// ─── Benchmark: All checkers on same source ──

#[test]
fn bench_all_checkers_on_single_source() {
    let agent = AgentRoleChecker::new();
    let capabilities = CapabilitiesRoleChecker::new();
    let contract = ContractRoleChecker::new();
    let surface = SurfaceRoleChecker::new();
    let taxonomy = TaxonomyRoleChecker::new();
    let utility = UtilityRoleChecker::new();

    let source = make_source("bench_multi.rs", "pub struct Foo;");

    let start = Instant::now();
    for _ in 0..1000 {
        let mut violations = Vec::new();
        agent.check_container(&source, &mut violations);
        capabilities.check_capability_routing(&source, "capabilities", &mut violations);
        contract.check_protocol(&source, &mut violations);
        surface.check_fn_count_limit(&source, &mut violations);
        taxonomy.check_entity(&source, &mut violations);
    }
    let elapsed = start.elapsed();
    // 1000 rounds of 5 checkers should complete within 10 seconds
    assert!(elapsed.as_millis() < 10000, "All checkers took {}ms", elapsed.as_millis());
}
