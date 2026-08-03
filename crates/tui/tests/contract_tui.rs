// Contract tests — verify TUI container wiring and key type bounds.
use tui_lint_arwaky::root_tui_container::TuiContainer;

#[test]
fn tui_container_type_exists() {
    fn assert_type<T>() {}
    assert_type::<TuiContainer>();
}

#[test]
fn tui_utility_modules_importable() {
    use tui_lint_arwaky::utility_file_system;

    // Verify key functions are accessible
    let _ = utility_file_system::list_directory
        as fn(&shared::common::FilePath) -> Vec<shared::tui::FileEntry>;
    let _ = utility_file_system::read_file_preview
        as fn(&shared::common::FilePath, usize) -> shared::common::DisplayContent;
    let _ = utility_file_system::is_valid_directory as fn(&shared::common::FilePath) -> bool;
    let _ = utility_file_system::parent_directory
        as fn(&shared::common::FilePath) -> Option<shared::common::FilePath>;
    let _ = utility_file_system::file_size_human as fn(u64) -> shared::common::DisplayContent;
    let _ = utility_file_system::path_components
        as fn(&shared::common::FilePath) -> Vec<shared::common::FilePath>;
}

#[test]
fn tui_report_formatter_module_importable() {
    use tui_lint_arwaky::utility_report_formatter;

    let _ = utility_report_formatter::format_results
        as fn(&shared::cli_commands::LintResultList) -> shared::common::DisplayContent;
    let _ = utility_report_formatter::format_doctor_report
        as fn(&shared::maintenance::ToolchainDiagnostics) -> shared::tui::LintExecutionResult;
    let _ = utility_report_formatter::format_dependency_report
        as fn(&str, &shared::maintenance::DependencyReport) -> shared::tui::LintExecutionResult;
}
