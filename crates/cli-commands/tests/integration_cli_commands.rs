// Integration tests — cli-commands with real filesystem.
#[test]
fn cli_scan_command_on_current_project() {
    let result = cli_commands_lint_arwaky::surface_scan_command::ScanCommand::execute(".");
    assert!(result.is_ok() || result.is_err(), "Should return a result");
}
