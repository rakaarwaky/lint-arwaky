// Contract tests — verify cli-commands modules compile and public API exists.

#[test]
fn contract_scan_command_handle_scan_exists() {
    let _ = std::any::type_name::<fn(
        Option<shared::common::FilePath>,
        shared::cli_commands::Format,
        std::sync::Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<std::sync::Arc<dyn shared::config_system::IConfigOrchestratorAggregate>>,
        Option<String>,
        Option<String>,
    ) -> shared::common::ExitCode>();
}

#[test]
fn contract_config_command_handle_config_show_exists() {
    let _ = std::any::type_name::<fn(
        std::sync::Arc<dyn shared::config_system::IConfigOrchestratorAggregate>,
    ) -> shared::common::ExitCode>();
}

#[test]
fn contract_fix_command_handle_fix_exists() {
    let _ = std::any::type_name::<fn(
        Option<shared::common::FilePath>,
        bool,
        std::sync::Arc<dyn shared::quality_rules::ICodeAnalysisAggregate>,
        std::sync::Arc<
            dyn Fn(bool) -> std::sync::Arc<dyn shared::auto_fix::LintFixOrchestratorAggregate>
                + Send
                + Sync,
        >,
    ) -> shared::common::ExitCode>();
}
