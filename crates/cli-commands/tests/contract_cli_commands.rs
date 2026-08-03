// Contract tests — verify cli-commands modules compile and are accessible.
#[test]
fn scan_command_module_exists() {
    let _ = std::any::type_name::<cli_commands_lint_arwaky::surface_scan_command::ScanCommand>();
}

#[test]
fn config_command_module_exists() {
    let _ = std::any::type_name::<cli_commands_lint_arwaky::surface_config_command::ConfigCommand>();
}

#[test]
fn fix_command_module_exists() {
    let _ = std::any::type_name::<cli_commands_lint_arwaky::surface_fix_command::FixCommand>();
}
