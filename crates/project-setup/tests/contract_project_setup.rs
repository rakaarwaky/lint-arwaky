// Contract tests — verify all concrete types implement their declared contract traits.
use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_env_generator::SetupEnvGenerator;
use project_setup_lint_arwaky::capabilities_language_detector::SetupLanguageDetector;
use project_setup_lint_arwaky::capabilities_mcp_config_generator::SetupMcpConfigGenerator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use shared_project_setup::{
    IAdapterInstallationProtocol, IEnvGenerationProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, ISetupAggregate,
};

#[test]
fn setup_management_orchestrator_implements_aggregate() {
    fn assert_trait<T: ISetupAggregate>() {}
    assert_trait::<SetupManagementOrchestrator>();
}

#[test]
fn setup_mcp_config_generator_implements_mcp_config_protocol() {
    fn assert_trait<T: IMcpConfigGenerationProtocol>() {}
    assert_trait::<SetupMcpConfigGenerator>();
}

#[test]
fn setup_env_generator_implements_env_generation_protocol() {
    fn assert_trait<T: IEnvGenerationProtocol>() {}
    assert_trait::<SetupEnvGenerator>();
}

#[test]
fn setup_language_detector_implements_language_detection_protocol() {
    fn assert_trait<T: ILanguageDetectionProtocol>() {}
    assert_trait::<SetupLanguageDetector>();
}

#[test]
fn setup_installer_adapter_implements_adapter_installation_protocol() {
    fn assert_trait<T: IAdapterInstallationProtocol>() {}
    assert_trait::<SetupInstallerAdapter>();
}

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SetupManagementOrchestrator>();
    assert_send_sync::<SetupMcpConfigGenerator>();
    assert_send_sync::<SetupEnvGenerator>();
    assert_send_sync::<SetupLanguageDetector>();
    assert_send_sync::<SetupInstallerAdapter>();
}

#[test]
fn orchestrator_can_be_arc_trait_object() {
    fn assert_object_safe<T: ISetupAggregate>() {}
    assert_object_safe::<SetupManagementOrchestrator>();
}

#[test]
fn mcp_config_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IMcpConfigGenerationProtocol>() {}
    assert_object_safe::<SetupMcpConfigGenerator>();
}

#[test]
fn adapter_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IAdapterInstallationProtocol>() {}
    assert_object_safe::<SetupInstallerAdapter>();
}
