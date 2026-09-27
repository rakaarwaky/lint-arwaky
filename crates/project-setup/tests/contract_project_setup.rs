// Contract tests — verify all concrete types implement their declared contract traits.
use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use shared::project_setup::{
    IAdapterInstallationProtocol, IConfigTemplateProtocol, IConfigWritingProtocol,
    IEnvGenerationProtocol, IFilePathExistenceProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, IPreFlightProtocol, ISetupAggregate,
};

#[test]
fn setup_management_orchestrator_implements_aggregate() {
    fn assert_trait<T: ISetupAggregate>() {}
    assert_trait::<SetupManagementOrchestrator>();
}

#[test]
fn setup_management_processor_implements_mcp_config_protocol() {
    fn assert_trait<T: IMcpConfigGenerationProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_management_processor_implements_env_generation_protocol() {
    fn assert_trait<T: IEnvGenerationProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_management_processor_implements_language_detection_protocol() {
    fn assert_trait<T: ILanguageDetectionProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_installer_adapter_implements_adapter_installation_protocol() {
    fn assert_trait<T: IAdapterInstallationProtocol>() {}
    assert_trait::<SetupInstallerAdapter>();
}

#[test]
fn setup_management_processor_implements_config_template_protocol() {
    fn assert_trait<T: IConfigTemplateProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_management_processor_implements_config_writing_protocol() {
    fn assert_trait<T: IConfigWritingProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_management_processor_implements_pre_flight_protocol() {
    fn assert_trait<T: IPreFlightProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_management_processor_implements_path_existence_protocol() {
    fn assert_trait<T: IFilePathExistenceProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SetupManagementOrchestrator>();
    assert_send_sync::<SetupManagementProcessor>();
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
    assert_object_safe::<SetupManagementProcessor>();
}

#[test]
fn adapter_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IAdapterInstallationProtocol>() {}
    assert_object_safe::<SetupInstallerAdapter>();
}
