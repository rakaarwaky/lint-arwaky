// PURPOSE: E2E tests — verify full project-setup pipeline from orchestrator to processor/installer.
// Layer: E2E (full integration of all layers).

use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::cli_commands::taxonomy_protocol_vo::{TransportProtocol, TransportUrlVO};
use shared::common::taxonomy_path_vo::DirectoryPath;

fn create_container() -> SetupContainer {
    SetupContainer::new()
}

// ─── E2E: MCP config generation pipeline ──

#[tokio::test]
async fn e2e_mcp_config_generation() {
    let container = create_container();
    let agg = container.aggregate();

    // 1. Check HTTP connectivity (agent layer)
    let url = TransportUrlVO::new("http://localhost:3001".to_string());
    let _status = agg.check_http(&url);

    // 2. Generate MCP config for different editors (capabilities layer)
    let transport = TransportProtocol::HTTP;
    let claude_config = agg.mcp_config_claude(&transport);
    let hermes_config = agg.mcp_config_hermes(&transport);
    let vscode_config = agg.mcp_config_vscode(&transport);

    // 3. Verify configs are generated
    assert!(!claude_config.value.is_empty());
    assert!(!hermes_config.value.is_empty());
    assert!(!vscode_config.value.is_empty());
}

// ─── E2E: Environment file generation pipeline ──

#[tokio::test]
async fn e2e_env_file_generation() {
    let container = create_container();
    let agg = container.aggregate();

    // 1. Detect project language (capabilities layer)
    let _language = agg.detect_language();
    let _languages = agg.detect_languages();

    // 2. Generate environment content (capabilities layer)
    let transport = TransportProtocol::HTTP;
    let home = DirectoryPath::new("/tmp".to_string()).unwrap();
    let env_content = agg.generate_env(&transport, &home);

    assert!(!env_content.value.is_empty());
}

// ─── E2E: Config directory creation pipeline ──

#[tokio::test]
async fn e2e_config_directory_creation() {
    let container = create_container();
    let agg = container.aggregate();

    // 1. Create global config directory
    let config_dir = agg.create_global_config_dir().unwrap();

    // 2. Verify path is valid
    assert!(!config_dir.to_string_lossy().is_empty());

    // 3. Check if file exists at the config location
    let _exists = agg.file_exists(&config_dir.to_string_lossy());
}

// ─── E2E: Full pipeline with all transport types ──

#[tokio::test]
async fn e2e_all_transport_types() {
    let container = create_container();
    let agg = container.aggregate();

    for transport in [TransportProtocol::HTTP, TransportProtocol::STDAggregate] {
        // Generate all MCP configs
        let _claude = agg.mcp_config_claude(&transport);
        let _hermes = agg.mcp_config_hermes(&transport);
        let _vscode = agg.mcp_config_vscode(&transport);

        // Generate env file
        let home = DirectoryPath::new("/tmp".to_string()).unwrap();
        let _env = agg.generate_env(&transport, &home);
    }
}
