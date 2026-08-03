// Acceptance test — FR-001: Project initialization generates valid MCP config.
// The setup system must produce a valid MCP configuration for each supported
// AI client (Claude, Hermes, VS Code) that can be written to disk.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use shared::project_setup::SetupManagementAggregate;
use tempfile::TempDir;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn fr001_claude_config_is_valid_json() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_claude(&TransportProtocol::STDAggregate);
    let json_str = serde_json::to_string(config.value()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.is_object(), "Claude config must be a valid JSON object");
}

#[test]
fn fr001_hermes_config_is_valid_json() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_hermes(&TransportProtocol::STDAggregate);
    let json_str = serde_json::to_string(config.value()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.is_object(), "Hermes config must be a valid JSON object");
}

#[test]
fn fr001_vscode_config_is_valid_json() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_vscode(&TransportProtocol::STDAggregate);
    let json_str = serde_json::to_string(config.value()).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.is_object(), "VS Code config must be a valid JSON object");
}

#[test]
fn fr001_claude_config_writable_to_disk() {
    let container = make_container();
    let agg = container.aggregate();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let config = agg.mcp_config_claude(&TransportProtocol::STDAggregate);
    let json_str = serde_json::to_string_pretty(config.value()).unwrap();
    let path = tmp.path().join("mcp_claude.json");
    let result = proto.write_config_file(&path.to_string_lossy(), &json_str);
    assert!(result.is_ok(), "FR-001: Claude config should be writable: {:?}", result);
}

#[test]
fn fr001_hermes_config_writable_to_disk() {
    let container = make_container();
    let agg = container.aggregate();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let config = agg.mcp_config_hermes(&TransportProtocol::STDAggregate);
    let json_str = serde_json::to_string_pretty(config.value()).unwrap();
    let path = tmp.path().join("mcp_hermes.json");
    let result = proto.write_config_file(&path.to_string_lossy(), &json_str);
    assert!(result.is_ok(), "FR-001: Hermes config should be writable: {:?}", result);
}

#[test]
fn fr001_vscode_config_writable_to_disk() {
    let container = make_container();
    let agg = container.aggregate();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();
    let config = agg.mcp_config_vscode(&TransportProtocol::STDAggregate);
    let json_str = serde_json::to_string_pretty(config.value()).unwrap();
    let path = tmp.path().join("mcp_vscode.json");
    let result = proto.write_config_file(&path.to_string_lossy(), &json_str);
    assert!(result.is_ok(), "FR-001: VS Code config should be writable: {:?}", result);
}
