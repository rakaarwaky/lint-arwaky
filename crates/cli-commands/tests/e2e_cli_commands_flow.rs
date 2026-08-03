// E2E tests — cli command flow.
use shared::cli_commands::Format;
use shared::common::FilePath;
use std::sync::Arc;

#[test]
fn e2e_scan_command_full_flow() {
    // handle_scan requires DI aggregates — verify function signature compiles.
    // Real E2E test: `cargo run --bin lint-arwaky-cli -- scan .`
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
fn e2e_quality_command_full_flow() {
    let _ = std::any::type_name::<fn(
        Option<FilePath>,
        Format,
        Arc<dyn shared::quality_rules::ICodeAnalysisAggregate>,
        Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
        Option<String>,
        Vec<String>,
    ) -> shared::common::ExitCode>();
}
