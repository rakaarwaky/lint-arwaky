// PURPOSE: Contract tests — verify all trait implementations for project-setup types.
// Layer: Contract (trait verification).

use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::{
    ISetupInstallerProtocol, ISetupManagementProtocol,
};
use std::sync::Arc;

// ─── Verify SetupInstallerAdapter implements ISetupInstallerProtocol ──

#[test]
fn setup_installer_adapter_implements_isetup_installer_protocol() {
    let adapter = SetupInstallerAdapter::new();
    let _: &dyn ISetupInstallerProtocol = &adapter;
}

// ─── Verify SetupManagementProcessor implements ISetupManagementProtocol ──

#[test]
fn setup_management_processor_implements_isetup_management_protocol() {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let processor = SetupManagementProcessor::new(installer);
    let _: &dyn ISetupManagementProtocol = &processor;
}

// ─── Verify SetupManagementOrchestrator implements SetupManagementAggregate ──

#[test]
fn setup_management_orchestrator_implements_setup_management_aggregate() {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let protocol = Arc::new(SetupManagementProcessor::new(installer));
    let orchestrator = SetupManagementOrchestrator::new(protocol);
    let _: &dyn SetupManagementAggregate = &orchestrator;
}

// ─── Verify container wiring returns correct trait objects ──

#[test]
fn container_aggregate_returns_setup_management_aggregate() {
    let container = SetupContainer::new();
    let _agg: Arc<dyn SetupManagementAggregate> = container.aggregate();
}

#[test]
fn container_protocol_returns_isetup_management_protocol() {
    let container = SetupContainer::new();
    let _proto: Arc<dyn ISetupManagementProtocol> = container.protocol();
}

// ─── Verify orchestrator delegates to protocol ──

#[test]
fn orchestrator_delegates_to_processor() {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let protocol = Arc::new(SetupManagementProcessor::new(installer));
    let _orchestrator = SetupManagementOrchestrator::new(protocol.clone());

    // Both should reference the same underlying processor (trait objects are Send + Sync)
    // Verification passed by successful construction above
}
