use cli_commands_lint_arwaky::surface_setup_command::handle_mcp_config;
use std::process::ExitCode;

#[test]
fn mcp_config_claude_returns_success() {
    let result = handle_mcp_config("claude");
    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn mcp_config_claude_code_returns_success() {
    let result = handle_mcp_config("claude-code");
    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn mcp_config_cursor_returns_success() {
    let result = handle_mcp_config("cursor");
    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn mcp_config_windsurf_returns_success() {
    let result = handle_mcp_config("windsurf");
    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn mcp_config_copilot_returns_success() {
    let result = handle_mcp_config("copilot");
    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn mcp_config_unknown_client_returns_success() {
    let result = handle_mcp_config("unknown-client");
    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn mcp_config_empty_client_returns_success() {
    let result = handle_mcp_config("");
    assert_eq!(result, ExitCode::SUCCESS);
}
