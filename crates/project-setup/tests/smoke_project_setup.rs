// PURPOSE: Smoke test — verify the project-setup crate boots and responds within 5 seconds.
// Layer: Smoke (must complete < 5s).

use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;

#[test]
fn smoke_project_setup_crate_boots_and_responds() {
    // 1. Container instantiates without panic
    let container = SetupContainer::new();

    // 2. Aggregate is accessible
    let _agg = container.aggregate();

    // 3. Protocol is accessible
    let _proto = container.protocol();

    // 4. All components instantiate
    let _installer = SetupInstallerAdapter::new();

    let installer = std::sync::Arc::new(SetupInstallerAdapter::new());
    let _processor = SetupManagementProcessor::new(installer);

    // 5. Orchestrator responds
    let installer = std::sync::Arc::new(SetupInstallerAdapter::new());
    let protocol = std::sync::Arc::new(SetupManagementProcessor::new(installer));
    let orch = SetupManagementOrchestrator::new(protocol);

    // 6. Basic method call works
    let url = shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO::new(
        "http://localhost:3001".to_string(),
    );
    let status = orch.check_http(&url);
    assert!(status.value);
}
