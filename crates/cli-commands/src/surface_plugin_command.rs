// PURPOSE: Plugin list — CLI thin wrapper
// Calls dispatcher for plugin business logic, only adds CLI output
use shared::common::ExitCode;
use shared::external_lint::IExternalLintAggregate;
use std::sync::Arc;

pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    // Delegate to dispatcher
    dispatcher::surface_plugin_action::handle_adapters(external_lint)
}
