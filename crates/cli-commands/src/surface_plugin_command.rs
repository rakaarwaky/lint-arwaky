// PURPOSE: PluginCommandsSurface — CLI surface for listing adapters/plugins
use shared::common::taxonomy_common_error::ExitCode;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::sync::Arc;

pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");
    let adapters = external_lint.adapter_names();
    if adapters.is_empty() {
        println!("  (none enabled)");
    } else {
        for adapter in adapters.iter() {
            println!("  - {adapter}");
        }
    }
    ExitCode::OK
}
