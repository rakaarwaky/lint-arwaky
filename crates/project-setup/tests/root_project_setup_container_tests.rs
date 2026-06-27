// PURPOSE: Integration tests for SetupContainer — construction and accessor sanity
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;

#[test]
fn container_default_constructs_with_aggregate_and_protocol() {
    let container = SetupContainer::new();
    let agg = container.aggregate();
    let proto = container.protocol();
    // Verify accessors return functional objects
    assert!(agg.detect_language().value.len() >= 2);
    assert!(!proto.which_mcp_binary().value.is_empty());
}

#[test]
fn container_default_trait_via_new() {
    let container = SetupContainer::default();
    let agg = container.aggregate();
    let _lang = agg.detect_language();
    let exists = agg.file_exists("Cargo.toml");
    assert!(exists);
}

#[test]
fn container_aggregate_generate_mcp_config() {
    let container = SetupContainer::new();
    let agg = container.aggregate();
    let config = agg.generate_mcp_config(&TransportProtocol::STDAggregate);
    let val = config.value();
    assert!(val.contains_key("transport"));
    assert_eq!(val.get("transport").unwrap(), "Stdio");
}

#[test]
fn container_aggregate_detect_language_returns_rust() {
    let container = SetupContainer::new();
    let agg = container.aggregate();
    let lang = agg.detect_language();
    // From workspace root where crates/ exists
    assert_eq!(lang.value, "rust");
}

#[test]
fn container_protocol_generate_mcp_config_has_lint_arwaky_entry() {
    let container = SetupContainer::new();
    let proto = container.protocol();
    let config = proto.generate_mcp_config();
    let val = config.value();
    assert!(val.contains_key("lint-arwaky"));
    let entry = val.get("lint-arwaky").unwrap();
    assert_eq!(entry["command"], "lint-arwaky");
}

#[test]
fn container_protocol_mcp_config_claude_wraps_in_mcp_servers() {
    let container = SetupContainer::new();
    let proto = container.protocol();
    let config = proto.mcp_config_claude();
    let val = config.value();
    assert!(val.contains_key("mcpServers"));
}

#[test]
fn container_protocol_get_config_template_returns_non_empty() {
    let container = SetupContainer::new();
    let proto = container.protocol();
    let tmpl = proto.get_config_template("rust");
    assert!(!tmpl.is_empty());
    let tmpl_py = proto.get_config_template("python");
    assert!(!tmpl_py.is_empty());
}

#[test]
fn container_protocol_file_exists_checks_paths() {
    let container = SetupContainer::new();
    let proto = container.protocol();
    assert!(proto.file_exists("Cargo.toml"));
    assert!(!proto.file_exists("nonexistent_xyz_123.txt"));
}
