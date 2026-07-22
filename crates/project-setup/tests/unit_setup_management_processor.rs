// PURPOSE: Unit tests for SetupManagementProcessor — business logic for setup artifacts.
// Layer: Capabilities (SetupManagementProcessor)

use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

fn processor() -> SetupManagementProcessor {
    let installer = Arc::new(SetupInstallerAdapter::new());
    SetupManagementProcessor::new(installer)
}

// ─── generate_env: Happy Path ──

#[test]
fn processor_generates_env_content() {
    let proc = processor();
    let home = DirectoryPath::new("/tmp".to_string()).unwrap();

    let env_content = proc.generate_env(&home);
    assert!(!env_content.value.is_empty());
}

// ─── generate_mcp_config: All formats ──

#[test]
fn processor_generates_mcp_config_claude() {
    let proc = processor();
    let config = proc.mcp_config_claude();
    assert!(!config.value.is_empty());
}

#[test]
fn processor_generates_mcp_config_hermes() {
    let proc = processor();
    let config = proc.mcp_config_hermes();
    assert!(!config.value.is_empty());
}

#[test]
fn processor_generates_mcp_config_vscode() {
    let proc = processor();
    let config = proc.mcp_config_vscode();
    assert!(!config.value.is_empty());
}

// ─── file_exists: Happy Path ──

#[test]
fn processor_file_exists_returns_true_for_existing() {
    let proc = processor();
    assert!(proc.file_exists("/tmp"));
}

#[test]
fn processor_file_exists_returns_false_for_nonexistent() {
    let proc = processor();
    assert!(!proc.file_exists("/nonexistent/path/that/does/not/exist"));
}

// ─── create_global_config_dir: Returns valid path ──

#[test]
fn processor_creates_global_config_dir() {
    let proc = processor();
    let config_dir = proc.create_global_config_dir();
    assert!(config_dir.is_ok());
}

// ─── Default trait ──

#[test]
fn setup_management_processor_default_creates_valid_instance() {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let _ = SetupManagementProcessor::new(installer);
}
