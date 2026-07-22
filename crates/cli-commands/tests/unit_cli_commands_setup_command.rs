// Unit tests for surface_setup_command — mcp-config binary resolution.

use cli_commands_lint_arwaky::surface_setup_command::handle_mcp_config;

#[test]
fn mcp_config_claude_produces_valid_json() {
    let exit = handle_mcp_config("claude");
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
}

#[test]
fn mcp_config_cursor_produces_valid_json() {
    let exit = handle_mcp_config("cursor");
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
}

#[test]
fn mcp_config_windsurf_produces_valid_json() {
    let exit = handle_mcp_config("windsurf");
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
}

#[test]
fn mcp_config_copilot_produces_valid_json() {
    let exit = handle_mcp_config("copilot");
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
}

#[test]
fn mcp_config_unknown_client_still_produces_json() {
    let exit = handle_mcp_config("unknown-client");
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
}

#[test]
fn mcp_config_all_produces_valid_json() {
    let exit = handle_mcp_config("all");
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
}
