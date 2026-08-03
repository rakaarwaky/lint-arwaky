// Acceptance test — FR-001: MCP Configuration Generation.
// Tests all 7 client formats, binary resolution, and alwaysAllow list.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use tempfile::TempDir;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

// ── QA #1–#7: Client-specific config formats ──

#[test]
fn fr001_claude_config_wraps_in_mcp_servers() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_claude();
    let value = config.value();
    assert!(
        value.get("mcpServers").is_some(),
        "FR-001 QA#1: Claude config must have 'mcpServers' wrapper"
    );
    let servers = value.get("mcpServers").unwrap();
    assert!(
        servers.get("lint-arwaky").is_some(),
        "FR-001 QA#1: must contain 'lint-arwaky' entry"
    );
}

#[test]
fn fr001_cursor_config_wraps_in_mcp_servers() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_cursor();
    let value = config.value();
    assert!(
        value.get("mcpServers").is_some(),
        "FR-001 QA#2: Cursor config must have 'mcpServers' wrapper"
    );
}

#[test]
fn fr001_windsurf_config_wraps_in_mcp_servers() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_windsurf();
    let value = config.value();
    assert!(
        value.get("mcpServers").is_some(),
        "FR-001 QA#3: Windsurf config must have 'mcpServers' wrapper"
    );
}

#[test]
fn fr001_copilot_config_wraps_in_mcp_servers() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_copilot();
    let value = config.value();
    assert!(
        value.get("mcpServers").is_some(),
        "FR-001 QA#4: Copilot config must have 'mcpServers' wrapper"
    );
}

#[test]
fn fr001_hermes_config_is_base_directly() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_hermes();
    let value = config.value();
    assert!(
        value.get("lint-arwaky").is_some(),
        "FR-001 QA#5: Hermes config must be base (no wrapper), with 'lint-arwaky' key"
    );
}

#[test]
fn fr001_vscode_config_wraps_in_mcp_servers() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_vscode();
    let value = config.value();
    assert!(
        value.get("mcp").is_some(),
        "FR-001 QA#6: VS Code config must have 'mcp' wrapper"
    );
    let mcp = value.get("mcp").unwrap();
    assert!(
        mcp.get("servers").is_some(),
        "FR-001 QA#6: must contain 'servers' under 'mcp'"
    );
}

#[test]
fn fr001_all_client_contains_all_formats() {
    let container = make_container();
    let agg = container.aggregate();
    let config = agg.mcp_config_all();
    let value = config.value();
    for client in &[
        "claude-code",
        "cursor",
        "windsurf",
        "copilot",
        "hermes",
        "vscode",
    ] {
        assert!(
            value.get(*client).is_some(),
            "FR-001 QA#7: 'all' config must contain '{}'",
            client
        );
    }
}

// ── QA #8–#10: Binary resolution ──

#[test]
fn fr001_binary_resolved_non_empty() {
    let container = make_container();
    let proto = container.protocol();
    let binary = proto.which_mcp_binary();
    assert!(
        !binary.value().is_empty(),
        "FR-001 QA#8: resolved binary must be non-empty"
    );
}

#[test]
fn fr001_binary_falls_back_to_bare_name() {
    let container = make_container();
    let proto = container.protocol();
    let binary = proto.which_mcp_binary();
    assert!(
        binary.value().contains("lint-arwaky"),
        "FR-001 QA#9: fallback binary should reference lint-arwaky, got: {}",
        binary.value()
    );
}

// ── Base config: command field uses resolved binary ──

#[test]
fn fr001_base_config_command_matches_resolved_binary() {
    let container = make_container();
    let proto = container.protocol();
    let bin = proto.which_mcp_binary();
    let config = proto.generate_mcp_config();
    let server = config.value().get("lint-arwaky").unwrap();
    let cmd = server.get("command").unwrap().as_str().unwrap();
    assert_eq!(
        cmd,
        bin.value(),
        "FR-001: command field must use resolved binary path"
    );
}

// ── Base config: alwaysAllow list matches FRD ──

#[test]
fn fr001_always_allow_matches_frd() {
    let container = make_container();
    let proto = container.protocol();
    let config = proto.generate_mcp_config();
    let server = config.value().get("lint-arwaky").unwrap();
    let allow = server.get("alwaysAllow").unwrap().as_array().unwrap();
    let expected = vec![
        "execute_command",
        "list_commands",
        "read_skill",
        "health_check",
        "get_config",
    ];
    let actual: Vec<&str> = allow.iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(
        actual, expected,
        "FR-001: alwaysAllow list must match FRD spec"
    );
}

// ── All configs are valid JSON and writable ──

#[test]
fn fr001_all_client_configs_are_valid_json() {
    let container = make_container();
    let agg = container.aggregate();
    for (name, config) in &[
        ("claude", agg.mcp_config_claude()),
        ("cursor", agg.mcp_config_cursor()),
        ("windsurf", agg.mcp_config_windsurf()),
        ("copilot", agg.mcp_config_copilot()),
        ("hermes", agg.mcp_config_hermes()),
        ("vscode", agg.mcp_config_vscode()),
    ] {
        let json_str = serde_json::to_string(config.value()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(
            parsed.is_object(),
            "FR-001: {} config must be valid JSON",
            name
        );
    }
}

#[test]
fn fr001_configs_writable_to_disk() {
    let container = make_container();
    let agg = container.aggregate();
    let proto = container.protocol();
    let tmp = TempDir::new().unwrap();

    for (name, config) in &[
        ("claude", agg.mcp_config_claude()),
        ("cursor", agg.mcp_config_cursor()),
        ("windsurf", agg.mcp_config_windsurf()),
        ("copilot", agg.mcp_config_copilot()),
        ("hermes", agg.mcp_config_hermes()),
        ("vscode", agg.mcp_config_vscode()),
    ] {
        let json_str = serde_json::to_string_pretty(config.value()).unwrap();
        let path = tmp.path().join(format!("mcp_{}.json", name));
        let result = proto.write_config_file(&path.to_string_lossy(), &json_str);
        assert!(
            result.is_ok(),
            "FR-001: {} config should be writable: {:?}",
            name,
            result
        );
    }
}
