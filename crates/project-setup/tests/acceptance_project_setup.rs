// PURPOSE: Acceptance tests — verify FRD requirements for project-setup.
// Layer: Acceptance (FRD requirement validation).

use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::cli_commands::taxonomy_protocol_vo::{TransportProtocol, TransportUrlVO};
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;

fn container() -> SetupContainer {
    SetupContainer::new()
}

// ─── Acceptance: MCP config generation for all editors ──

#[test]
fn acceptance_project_setup_generates_claude_config() {
    // FRD requirement: Must generate Claude-compatible MCP config
    let container = container();
    let agg = container.aggregate();

    let transport = TransportProtocol::HTTP;
    let config = agg.mcp_config_claude(&transport);

    assert!(
        !config.value.is_empty(),
        "Claude config should not be empty"
    );
}

#[test]
fn acceptance_project_setup_generates_hermes_config() {
    // FRD requirement: Must generate Hermes-compatible MCP config
    let container = container();
    let agg = container.aggregate();

    let transport = TransportProtocol::HTTP;
    let config = agg.mcp_config_hermes(&transport);

    assert!(
        !config.value.is_empty(),
        "Hermes config should not be empty"
    );
}

#[test]
fn acceptance_project_setup_generates_vscode_config() {
    // FRD requirement: Must generate VSCode-compatible MCP config
    let container = container();
    let agg = container.aggregate();

    let transport = TransportProtocol::HTTP;
    let config = agg.mcp_config_vscode(&transport);

    assert!(
        !config.value.is_empty(),
        "VSCode config should not be empty"
    );
}

// ─── Acceptance: Language detection ──

#[test]
fn acceptance_project_setup_detects_language() {
    // FRD requirement: Must detect project language (rust, python, javascript)
    let container = container();
    let agg = container.aggregate();

    let language = agg.detect_language();

    assert!(
        language.value == "rust" || language.value == "python" || language.value == "javascript",
        "Detected language should be rust, python, or javascript"
    );
}

#[test]
fn acceptance_project_setup_detects_all_languages() {
    // FRD requirement: Must detect all project languages
    let container = container();
    let agg = container.aggregate();

    let languages = agg.detect_languages();

    assert!(
        !languages.values.is_empty(),
        "Should detect at least one language"
    );
}

// ─── Acceptance: Config directory creation ──

#[test]
fn acceptance_project_setup_creates_config_dir() {
    // FRD requirement: Must create ~/.config/lint-arwaky directory structure
    let container = container();
    let agg = container.aggregate();

    let config_dir = agg.create_global_config_dir().unwrap();

    assert!(
        config_dir.to_string_lossy().contains("lint-arwaky"),
        "Config dir should contain 'lint-arwaky' in path"
    );
}

// ─── Acceptance: HTTP connectivity check ──

#[test]
fn acceptance_project_setup_checks_http() {
    // FRD requirement: Must check MCP server HTTP connectivity
    let container = container();
    let agg = container.aggregate();

    let url = TransportUrlVO::new("http://localhost:3001".to_string());
    let status = agg.check_http(&url);

    assert!(status.value, "HTTP check should return success status");
}

// ─── Acceptance: File existence check ──

#[test]
fn acceptance_project_setup_checks_file_exists() {
    // FRD requirement: Must verify file existence before writing
    let container = container();
    let agg = container.aggregate();

    assert!(agg.file_exists("/tmp"), "/tmp should exist");
    assert!(
        !agg.file_exists("/nonexistent/path"),
        "Nonexistent path should return false"
    );
}
