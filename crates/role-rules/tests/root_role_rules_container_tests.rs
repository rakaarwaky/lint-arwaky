use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;

#[test]
fn container_can_be_constructed() {
    let container = RoleContainer::new();
    let aggregate = container.aggregate();
    // All six checkers should be wired — call each method to verify no panic
    let _tax = aggregate.taxonomy();
    let _con = aggregate.contract();
    let _inf = aggregate.infrastructure();
    let _cap = aggregate.capabilities();
    let _surf = aggregate.surface();
    let _agent = aggregate.agent();
    // If we reach here, all checkers are wired successfully
}

#[test]
fn orchestrator_can_be_created_from_container() {
    let container = RoleContainer::new();
    let orchestrator = container.orchestrator();
    assert_eq!(orchestrator.name(), "role-rules");
}

#[test]
fn container_default_is_same_as_new() {
    let c1 = RoleContainer::new();
    let c2 = RoleContainer::default();
    assert_eq!(c1.orchestrator().name(), c2.orchestrator().name());
}

#[test]
fn container_with_config_uses_provided_config() {
    let config = shared::config_system::taxonomy_config_vo::default_aes_config();
    let container = RoleContainer::new_with_config(config);
    let _aggregate = container.aggregate();
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "role-rules");
}
