// PURPOSE: Smoke test — verify the role-rules crate boots and responds within 5 seconds.
// Layer: Smoke (must complete < 5s).

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;

#[test]
fn smoke_role_rules_crate_boots_and_responds() {
    // 1. Container instantiates without panic
    let container = RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    );

    // 2. Orchestrator is accessible and named correctly
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "role-rules");

    // 3. All checkers instantiate independently
    let _agent = AgentRoleChecker::new();
    let _capabilities = CapabilitiesRoleChecker::new();
    let _contract = ContractRoleChecker::new();
    let _surface = SurfaceRoleChecker::new();
    let _taxonomy = TaxonomyRoleChecker::new();
    let _utility = UtilityRoleChecker::new();
}
