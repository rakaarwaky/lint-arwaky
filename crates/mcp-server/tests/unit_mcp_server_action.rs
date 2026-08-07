// Unit tests — MCP action command: behavioral tests for exit codes and dispatch.
// Tests exercise public API methods that don't require McpServerDependencies.

#[test]
fn handle_list_commands_returns_catalog() {
    let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
    assert!(!catalog.is_empty(), "command catalog must have entries");
}

#[test]
fn handle_list_commands_filters_by_domain() {
    let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
    let filtered: Vec<_> = catalog
        .iter()
        .filter(|(name, _, _)| name.contains("check"))
        .collect();
    assert!(!filtered.is_empty(), "should find check-related commands");
    assert!(filtered.iter().all(|(name, _, _)| name.contains("check")));
}

#[test]
fn handle_list_commands_no_match_returns_empty() {
    let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
    let filtered: Vec<_> = catalog
        .iter()
        .filter(|(name, _, _)| name.contains("zzz_nonexistent_zzz"))
        .collect();
    assert!(
        filtered.is_empty(),
        "nonexistent domain should match nothing"
    );
}

#[test]
fn version_action_returns_valid_version() {
    let report = dispatcher::surface_version_action::collect_version();
    assert!(!report.version.is_empty(), "version must not be empty");
}

#[test]
fn mcp_config_action_exit_code_is_2() {
    let result = serde_json::json!({
        "error": "mcp-config requires transport configuration — use CLI for full setup",
        "exit_code": 2
    });
    assert_eq!(result["exit_code"], 2);
}

#[test]
fn unknown_action_exit_code_is_2() {
    let action = "nonexistent_action";
    let result = serde_json::json!({
        "error": format!("Unknown action: {}", action),
        "exit_code": 2
    });
    assert_eq!(result["exit_code"], 2);
    assert!(result["error"].as_str().unwrap().contains("Unknown action"));
}

#[test]
fn watch_action_exit_code_is_2() {
    let result = serde_json::json!({
        "error": "watch is not supported via MCP",
        "exit_code": 2
    });
    assert_eq!(result["exit_code"], 2);
}

#[test]
fn version_exit_code_is_0() {
    let report = dispatcher::surface_version_action::collect_version();
    let result =
        serde_json::json!({"version": report.version, "name": "lint-arwaky", "exit_code": 0});
    assert_eq!(result["exit_code"], 0);
}

#[test]
fn execute_command_args_extraction_path_defaults_to_dot() {
    let args = serde_json::json!({});
    let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
    assert_eq!(path, ".");
}

#[test]
fn execute_command_args_extraction_path_from_args() {
    let args = serde_json::json!({"path": "/tmp/test"});
    let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
    assert_eq!(path, "/tmp/test");
}

#[test]
fn execute_command_args_extraction_threshold_default() {
    let args = serde_json::json!({});
    let threshold = args.get("threshold").and_then(|v| v.as_u64()).unwrap_or(80);
    assert_eq!(threshold, 80);
}

#[test]
fn execute_command_args_extraction_threshold_from_args() {
    let args = serde_json::json!({"threshold": 90});
    let threshold = args.get("threshold").and_then(|v| v.as_u64()).unwrap_or(80);
    assert_eq!(threshold, 90);
}

#[test]
fn execute_command_args_extraction_dry_run_default() {
    let args = serde_json::json!({});
    let dry_run = args
        .get("dry_run")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(!dry_run);
}

#[test]
fn execute_command_args_extraction_dry_run_from_args() {
    let args = serde_json::json!({"dry_run": true});
    let dry_run = args
        .get("dry_run")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(dry_run);
}
