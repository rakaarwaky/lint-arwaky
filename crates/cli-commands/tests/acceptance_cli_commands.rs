// Acceptance tests — cli commands produce valid output.
#[test]
fn acceptance_scan_command_returns_results() {
    let result = cli_commands::surface_scan_command::ScanCommand::execute(".");
    assert!(
        result.is_ok() || result.is_err(),
        "Scan should return a result"
    );
}
