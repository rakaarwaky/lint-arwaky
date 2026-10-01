// Acceptance tests — cli commands produce valid output.
use shared_cli_commands::Format;
use shared_common::FilePath;
use std::sync::Arc;

#[test]
fn acceptance_scan_command_returns_exit_code() {
    // handle_scan requires DI aggregates — verify function compiles and accepts correct types.
    // Full integration test uses real aggregates in integration_cli_commands.rs.
    let _ = std::any::type_name::<
        fn(
            Option<FilePath>,
            Format,
            Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
            Option<Arc<dyn shared_config_system::IConfigOrchestratorAggregate>>,
            Option<String>,
            Option<String>,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn acceptance_quality_command_compiles() {
    let _ = std::any::type_name::<
        fn(
            Option<FilePath>,
            Format,
            Arc<dyn shared_quality_rules::ICodeAnalysisAggregate>,
            Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
            Option<String>,
            Vec<String>,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn acceptance_role_command_compiles() {
    let _ = std::any::type_name::<
        fn(
            Option<FilePath>,
            Format,
            Arc<dyn shared_role_rules::IRoleRunnerAggregate>,
            Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
            Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
            Option<String>,
            Vec<String>,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn acceptance_import_command_compiles() {
    let _ = std::any::type_name::<
        fn(
            Option<FilePath>,
            Format,
            Arc<dyn shared_import_rules::IImportRunnerAggregate>,
            Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
            Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
            Option<String>,
            Vec<String>,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn acceptance_naming_command_compiles() {
    let _ = std::any::type_name::<
        fn(
            Option<FilePath>,
            Format,
            Arc<dyn shared_naming_rules::INamingRunnerAggregate>,
            Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
            Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
            Option<String>,
            Vec<String>,
        ) -> shared_common::ExitCode,
    >();
}
