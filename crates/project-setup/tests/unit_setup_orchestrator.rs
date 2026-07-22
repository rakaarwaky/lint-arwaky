// PURPOSE: Unit tests for SetupManagementOrchestrator — agent layer delegation.
// Layer: Agent (SetupManagementOrchestrator)

use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use shared::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

fn build_orchestrator() -> SetupManagementOrchestrator {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let protocol = Arc::new(SetupManagementProcessor::new(installer));
    SetupManagementOrchestrator::new(protocol)
}

// ─── Verify orchestrator implements SetupManagementAggregate ──

#[test]
fn orchestrator_implements_setup_management_aggregate() {
    let orch = build_orchestrator();
    let _: &dyn SetupManagementAggregate = &orch;
}

// ─── check_http: Returns success status ──

#[test]
fn orchestrator_check_http_returns_success() {
    let orch = build_orchestrator();
    let url = shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO::new("http://localhost:3001".to_string());
    let status = orch.check_http(&url);
    assert!(status.value);
}

// ─── generate_env: Returns env content ──

#[test]
fn orchestrator_generates_env_content() {
    let orch = build_orchestrator();
    let transport = shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::new("http".to_string());
    let home = shared::common::taxonomy_path_vo::DirectoryPath::new("/tmp".to_string());
    let env_content = orch.generate_env(&transport, &home);
    assert!(!env_content.value.is_empty());
}

// ─── detect_language: Returns default language ──

#[test]
fn orchestrator_detects_default_language() {
    let orch = build_orchestrator();
    let language = orch.detect_language();
    // Should return a valid language (rust, python, or javascript)
    assert!(language.value == "rust" || language.value == "python" || language.value == "javascript");
}

// ─── detect_languages: Returns languages map ──

#[test]
fn orchestrator_detects_languages() {
    let orch = build_orchestrator();
    let languages = orch.detect_languages();
    // Should return at least one language
    assert!(!languages.values.is_empty());
}

// ─── Default trait ──

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SetupManagementOrchestrator>();
}
