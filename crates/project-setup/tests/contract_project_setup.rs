// Contract tests — verify all concrete types implement their declared contract traits.
use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use shared::project_setup::{
    ISetupInstallerProtocol, ISetupManagementProtocol, SetupManagementAggregate,
};

#[test]
fn setup_management_orchestrator_implements_aggregate() {
    fn assert_trait<T: SetupManagementAggregate>() {}
    assert_trait::<SetupManagementOrchestrator>();
}

#[test]
fn setup_management_processor_implements_protocol() {
    fn assert_trait<T: ISetupManagementProtocol>() {}
    assert_trait::<SetupManagementProcessor>();
}

#[test]
fn setup_installer_adapter_implements_installer_protocol() {
    fn assert_trait<T: ISetupInstallerProtocol>() {}
    assert_trait::<SetupInstallerAdapter>();
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
    fn assert_object_safe<T: SetupManagementAggregate>() {}
    assert_object_safe::<SetupManagementOrchestrator>();
}

#[test]
fn protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: ISetupManagementProtocol>() {}
    assert_object_safe::<SetupManagementProcessor>();
}

#[test]
fn installer_can_be_arc_trait_object() {
    fn assert_object_safe<T: ISetupInstallerProtocol>() {}
    assert_object_safe::<SetupInstallerAdapter>();
}
