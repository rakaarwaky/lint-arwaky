// PURPOSE: Verify all trait implementations exist for role-rules structs.
// Layer: Contract verification — runs in ms, every PR.

use role_rules_lint_arwaky::agent_role_orchestrator::RoleCheckerDeps;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;

use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::contract_role_contract_protocol::IContractRoleChecker;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;

// ─── IAgentRoleChecker ──────────────────────────────

#[test]
fn agent_role_checker_implements_i_agent_role_checker() {
    fn assert_trait<T: IAgentRoleChecker>() {}
    assert_trait::<AgentRoleChecker>();
}

#[test]
fn agent_role_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AgentRoleChecker>();
}

// ─── ICapabilitiesRoleChecker ────────────────────────

#[test]
fn capabilities_role_checker_implements_capabilities_protocol() {
    fn assert_trait<T: ICapabilitiesRoleChecker>() {}
    assert_trait::<CapabilitiesRoleChecker>();
}

#[test]
fn capabilities_role_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CapabilitiesRoleChecker>();
}

// ─── IContractRoleChecker ──────────────────────────

#[test]
fn contract_role_checker_implements_i_contract_role_checker() {
    fn assert_trait<T: IContractRoleChecker>() {}
    assert_trait::<ContractRoleChecker>();
}

#[test]
fn contract_role_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ContractRoleChecker>();
}

// ─── ISurfaceRoleChecker ───────────────────────────

#[test]
fn surface_role_checker_implements_i_surface_role_checker() {
    fn assert_trait<T: ISurfaceRoleChecker>() {}
    assert_trait::<SurfaceRoleChecker>();
}

#[test]
fn surface_role_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SurfaceRoleChecker>();
}

// ─── ITaxonomyRoleChecker ──────────────────────────

#[test]
fn taxonomy_role_checker_implements_i_taxonomy_role_checker() {
    fn assert_trait<T: ITaxonomyRoleChecker>() {}
    assert_trait::<TaxonomyRoleChecker>();
}

#[test]
fn taxonomy_role_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TaxonomyRoleChecker>();
}

// ─── IUtilityRoleChecker ──────────────────────────

#[test]
fn utility_role_checker_implements_i_utility_role_checker() {
    fn assert_trait<T: IUtilityRoleChecker>() {}
    assert_trait::<UtilityRoleChecker>();
}

#[test]
fn utility_role_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<UtilityRoleChecker>();
}

// ─── IRoleRunnerAggregate ──────────────────────────

#[test]
fn role_orchestrator_implements_i_role_runner_aggregate() {
    fn assert_trait<T: IRoleRunnerAggregate>() {}
    assert_trait::<RoleOrchestrator>();
}

#[test]
fn role_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RoleOrchestrator>();
}

// ─── Container wiring ──────────────────────────────

#[test]
fn role_container_creates_instance() {
    let container = RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    );
    let _ = &container;
}

#[test]
fn role_container_orchestrator_returns_trait_object() {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch: std::sync::Arc<dyn IRoleRunnerAggregate> = container.orchestrator();
    // Verify trait object is usable — call name() accessor
    assert_eq!(orch.name(), "role-rules");
}

// ─── Default trait implementations ──────────────────

#[test]
fn agent_role_checker_implements_default() {
    let _ = AgentRoleChecker::default();
}

#[test]
fn capabilities_role_checker_implements_default() {
    let _ = CapabilitiesRoleChecker::default();
}

#[test]
fn contract_role_checker_implements_default() {
    let _ = ContractRoleChecker::default();
}

#[test]
fn surface_role_checker_implements_default() {
    let _ = SurfaceRoleChecker::default();
}

#[test]
fn taxonomy_role_checker_implements_default() {
    let _ = TaxonomyRoleChecker::default();
}

#[test]
fn utility_role_checker_implements_default() {
    let _ = UtilityRoleChecker::default();
}
