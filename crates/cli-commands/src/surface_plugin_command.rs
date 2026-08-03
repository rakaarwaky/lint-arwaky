// PURPOSE: Plugin list — CLI thin wrapper
// Calls dispatcher for plugin business logic, only adds CLI output.
use shared::common::ExitCode;
use shared::external_lint::IExternalLintAggregate;
use std::sync::Arc;

pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    let adapters = dispatcher::surface_plugin_action::collect_adapters(external_lint);
    println!("External lint adapters:");
    if adapters.values.is_empty() {
        println!("  (none enabled)");
    } else {
        for adapter in adapters.values.iter() {
            println!("  - {adapter}");
        }
    }
    ExitCode::OK
}
