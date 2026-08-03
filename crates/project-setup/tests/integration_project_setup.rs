// Integration tests — full DI wiring via SetupContainer.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::{ISetupManagementProtocol, SetupManagementAggregate};
use std::sync::Arc;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn container_creates_successfully() {
    let _container = make_container();
}

#[test]
fn container_returns_aggregate() {
    let container = make_container();
    let _: Arc<dyn SetupManagementAggregate> = container.aggregate();
}

#[test]
fn container_returns_protocol() {
    let container = make_container();
    let _: Arc<dyn ISetupManagementProtocol> = container.protocol();
}

#[test]
fn aggregate_and_protocol_are_accessible() {
    let container = make_container();
    let agg = container.aggregate();
    let proto = container.protocol();
    let _ = agg.detect_language();
    let _ = proto.generate_mcp_config();
}

#[test]
fn aggregate_detect_language_via_container() {
    let container = make_container();
    let agg = container.aggregate();
    let lang = agg.detect_language();
    assert!(lang.is_some());
    assert!(!lang.unwrap().value().is_empty());
}

#[test]
fn protocol_generate_mcp_config_via_container() {
    let container = make_container();
    let proto = container.protocol();
    let config = proto.generate_mcp_config();
    assert!(config.value().get("lint-arwaky").is_some());
}

#[test]
fn aggregate_get_config_template() {
    let container = make_container();
    let agg = container.aggregate();
    let template = agg.get_config_template("rust").unwrap();
    assert!(!template.is_empty());
}

#[test]
fn protocol_which_mcp_binary() {
    let container = make_container();
    let proto = container.protocol();
    let binary = proto.which_mcp_binary();
    assert!(!binary.value().is_empty());
}
