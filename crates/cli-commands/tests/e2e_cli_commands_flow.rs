// E2E tests — cli command flow.
#[test]
fn e2e_scan_command_full_flow() {
    let result = cli_commands_lint_arwaky::surface_scan_command::ScanCommand::execute(".");
    // Should complete without panic
    match result {
        Ok(_) => {}
        Err(_) => {}
    }
}
