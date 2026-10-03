// PURPOSE: Plugin list — CLI thin wrapper
// Calls shared_structure_rules::surface_plugin_action for plugin business logic, only adds CLI output.
use shared_common::ExitCode;
use shared_external_lint::IExternalLintAggregate;
use std::sync::Arc;

pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    let adapters = shared_structure_rules::surface_plugin_action::collect_adapters(external_lint);
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
