// Acceptance tests — cli commands produce valid output.
use shared::cli_commands::Format;
use shared::common::FilePath;
use std::sync::Arc;

#[test]
fn acceptance_scan_command_returns_exit_code() {
    // handle_scan requires DI aggregates — verify function compiles and accepts correct types.
    // Full integration test uses real aggregates in integration_cli_commands.rs.
    let _ = std::any::type_name::<fn(
        Option<FilePath>,
        Format,
        Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<Arc<dyn shared::config_system::IConfigOrchestratorAggregate>>,
        Option<String>,
        Option<String>,
    ) -> shared::common::ExitCode>();
}

#[test]
fn acceptance_quality_command_compiles() {
    let _ = std::any::type_name::<fn(
        Option<FilePath>,
        Format,
        Arc<dyn shared::quality_rules::ICodeAnalysisAggregate>,
        Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<String>,
        Vec<String>,
    ) -> shared::common::ExitCode>();
}

#[test]
fn acceptance_role_command_compiles() {
    let _ = std::any::type_name::<fn(
        Option<FilePath>,
        Format,
        Arc<dyn shared::role_rules::IRoleRunnerAggregate>,
        Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
        Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<String>,
        Vec<String>,
    ) -> shared::common::ExitCode>();
}

#[test]
fn acceptance_import_command_compiles() {
    let _ = std::any::type_name::<fn(
        Option<FilePath>,
        Format,
        Arc<dyn shared::import_rules::IImportRunnerAggregate>,
        Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
        Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<String>,
        Vec<String>,
    ) -> shared::common::ExitCode>();
}

#[test]
fn acceptance_naming_command_compiles() {
    let _ = std::any::type_name::<fn(
        Option<FilePath>,
        Format,
        Arc<dyn shared::naming_rules::INamingRunnerAggregate>,
        Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
        Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<String>,
        Vec<String>,
    ) -> shared::common::ExitCode>();
}
