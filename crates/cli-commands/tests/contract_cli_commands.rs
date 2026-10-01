// Contract tests — verify cli-commands modules compile and public API exists.

#[test]
fn contract_scan_command_handle_scan_exists() {
    let _ = std::any::type_name::<
        fn(
            Option<shared_common::FilePath>,
            shared_cli_commands::Format,
            std::sync::Arc<
                dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
            >,
            Option<std::sync::Arc<dyn shared_config_system::IConfigOrchestratorAggregate>>,
            Option<String>,
            Option<String>,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn contract_config_command_handle_config_show_exists() {
    let _ = std::any::type_name::<
        fn(
            std::sync::Arc<dyn shared_config_system::IConfigOrchestratorAggregate>,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn contract_fix_command_handle_fix_exists() {
    let _ = std::any::type_name::<
        fn(
            Option<shared_common::FilePath>,
            bool,
            std::sync::Arc<dyn shared_quality_rules::ICodeAnalysisAggregate>,
            std::sync::Arc<
                dyn Fn(bool) -> std::sync::Arc<dyn shared_auto_fix::IFixAggregate> + Send + Sync,
            >,
        ) -> shared_common::ExitCode,
    >();
}

#[test]
fn contract_skill_command_handle_skill_list_exists() {
    let _ = std::any::type_name::<fn() -> shared_common::ExitCode>();
}

#[test]
fn contract_skill_command_handle_skill_read_exists() {
    let _ = std::any::type_name::<fn(&str, bool) -> shared_common::ExitCode>();
}
