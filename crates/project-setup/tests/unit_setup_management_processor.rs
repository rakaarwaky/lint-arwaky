// PURPOSE: Unit tests for SetupManagementProcessor — business logic for setup artifacts.
// Layer: Capabilities (SetupManagementProcessor)

use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO};
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use shared::project_setup::taxonomy_setup_contract_vo::{
    McpBinaryNameVO, ProjectLanguageVO, ProjectLanguagesVO,
};
use std::sync::Arc;

fn processor() -> SetupManagementProcessor {
    let installer = Arc::new(SetupInstallerAdapter::new());
    SetupManagementProcessor::new(installer)
}

// ─── generate_env: Happy Path ──

#[test]
fn processor_generates_env_content() {
    let proc = processor();
    let transport =
        shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::new("http".to_string());
    let home = DirectoryPath::new("/tmp".to_string());

    let env_content = proc.generate_env(&transport, &home);
    assert!(!env_content.value.is_empty());
}

// ─── generate_mcp_config: All formats ──

#[test]
fn processor_generates_mcp_config_claude() {
    let proc = processor();
    let transport =
        shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::new("http".to_string());

    let config = proc.mcp_config_claude(&transport);
    assert!(!config.value.is_empty());
}

#[test]
fn processor_generates_mcp_config_hermes() {
    let proc = processor();
    let transport =
        shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::new("http".to_string());

    let config = proc.mcp_config_hermes(&transport);
    assert!(!config.value.is_empty());
}

#[test]
fn processor_generates_mcp_config_vscode() {
    let proc = processor();
    let transport =
        shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::new("http".to_string());

    let config = proc.mcp_config_vscode(&transport);
    assert!(!config.value.is_empty());
}

// ─── file_exists: Happy Path ──

#[test]
fn processor_file_exists_returns_true_for_existing() {
    let proc = processor();
    // /tmp should exist on most systems
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
    // Should return a valid path (typically ~/.config/lint-arwaky)
    assert!(!config_dir.value.is_empty());
}

// ─── Default trait ──

#[test]
fn setup_management_processor_default_creates_valid_instance() {
    let installer = Arc::new(SetupInstallerAdapter::new());
    let _ = SetupManagementProcessor::default(installer);
}
