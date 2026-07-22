// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real SetupContainer).

use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

fn container() -> SetupContainer {
    SetupContainer::new()
}

// ─── Container wiring tests ──

#[test]
fn container_creates_aggregate_successfully() {
    let c = container();
    let _agg = c.aggregate();
}

#[test]
fn container_returns_same_arc_on_multiple_calls() {
    let c = container();
    let agg1 = c.aggregate();
    let agg2 = c.aggregate();
    assert!(Arc::ptr_eq(&agg1, &agg2));
}

#[test]
fn container_aggregate_returns_trait_object() {
    let c = container();
    let _agg: Arc<dyn SetupManagementAggregate> = c.aggregate();
}

#[test]
fn container_protocol_returns_trait_object() {
    let c = container();
    let _proto: Arc<dyn ISetupManagementProtocol> = c.protocol();
}

// ─── Cross-layer collaboration test ──

#[test]
fn full_pipeline_wiring_works() {
    let c = container();
    let agg = c.aggregate();

    // Verify all public methods work through the aggregate
    let url = shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO::new(
        "http://localhost:3001".to_string(),
    );
    let _status = agg.check_http(&url);

    let transport =
        shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::new("http".to_string());
    let _env = agg.generate_env(
        &transport,
        &shared::common::taxonomy_path_vo::DirectoryPath::new("/tmp".to_string()),
    );

    let _config = agg.mcp_config_claude(&transport);
    let _config = agg.mcp_config_hermes(&transport);
    let _config = agg.mcp_config_vscode(&transport);

    let _languages = agg.detect_languages();
    assert!(agg.file_exists("/tmp"));
}

// ─── Default trait ──

#[test]
fn container_default_creates_valid_instance() {
    let _ = SetupContainer::default();
}
