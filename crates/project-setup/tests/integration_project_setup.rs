// Integration tests — full DI wiring via SetupContainer.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::{
    IAdapterInstallationProtocol, IConfigTemplateProtocol, IConfigWritingProtocol,
    IEnvGenerationProtocol, IFilePathExistenceProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, IPreFlightProtocol, ISetupAggregate, SetupRequest,
};
use std::sync::Arc;

fn make_container() -> SetupContainer {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = c.io();
    SetupContainer::new(fs)
}

#[test]
fn container_creates_successfully() {
    let _container = make_container();
}

#[test]
fn container_returns_aggregate() {
    let container = make_container();
    let _: Arc<dyn ISetupAggregate> = container.aggregate();
}

#[test]
fn container_returns_mcp_config() {
    let container = make_container();
    let _: &Arc<dyn IMcpConfigGenerationProtocol> = container.mcp_config();
}

#[test]
fn container_returns_env_generation() {
    let container = make_container();
    let _: &Arc<dyn IEnvGenerationProtocol> = container.env_generation();
}

#[test]
fn container_returns_language_detection() {
    let container = make_container();
    let _: &Arc<dyn ILanguageDetectionProtocol> = container.language_detection();
}

#[test]
fn container_returns_adapter_installation() {
    let container = make_container();
    let _: &Arc<dyn IAdapterInstallationProtocol> = container.adapter_installation();
}

#[test]
fn container_returns_config_template() {
    let container = make_container();
    let _: &Arc<dyn IConfigTemplateProtocol> = container.config_template();
}

#[test]
fn container_returns_config_writing() {
    let container = make_container();
    let _: &Arc<dyn IConfigWritingProtocol> = container.config_writing();
}

#[test]
fn container_returns_pre_flight() {
    let container = make_container();
    let _: &Arc<dyn IPreFlightProtocol> = container.pre_flight();
}

#[test]
fn container_returns_path_existence() {
    let container = make_container();
    let _: &Arc<dyn IFilePathExistenceProtocol> = container.path_existence();
}

#[test]
fn aggregate_and_protocol_are_accessible() {
    let container = make_container();
    let agg = container.aggregate();
    let _ = agg.execute(SetupRequest::detect_language()).into_language();
    let mcp = container.mcp_config();
    let _ = mcp.generate_mcp_config();
}

#[test]
fn aggregate_detect_language_via_container() {
    let container = make_container();
    let agg = container.aggregate();
    let lang = agg.execute(SetupRequest::detect_language()).into_language();
    assert!(lang.is_some());
    assert!(!lang.unwrap().value().is_empty());
}

#[test]
fn mcp_config_generate_via_container() {
    let container = make_container();
    let mcp = container.mcp_config();
    let config = mcp.generate_mcp_config();
    assert!(config.value().get("lint-arwaky").is_some());
}

#[test]
fn aggregate_get_config_template() {
    let container = make_container();
    let agg = container.aggregate();
    let template = agg
        .execute(SetupRequest::get_config_template("rust"))
        .into_template()
        .unwrap();
    assert!(!template.is_empty());
}

#[test]
fn mcp_which_binary_via_container() {
    let container = make_container();
    let mcp = container.mcp_config();
    let binary = mcp.which_mcp_binary();
    assert!(!binary.value().is_empty());
}
