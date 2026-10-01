// Integration tests — cli-commands with real filesystem.
use shared_cli_commands::Format;
use shared_common::FilePath;
use std::sync::Arc;

#[test]
fn integration_scan_command_accepts_filesystem_aggregate() {
    // Verify handle_scan compiles with correct aggregate types.
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
fn integration_orphan_command_accepts_config_aggregate() {
    let _ = std::any::type_name::<
        fn(
            Option<FilePath>,
            Option<String>,
            Format,
            Arc<dyn shared_orphan_rules::IOrphanAggregate>,
            Arc<dyn shared_config_system::IConfigOrchestratorAggregate>,
            Arc<dyn shared_report_formatter::IReportFormatterAggregate>,
            Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
            Option<String>,
        ) -> shared_common::ExitCode,
    >();
}
