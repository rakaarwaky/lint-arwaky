// PURPOSE: Config show — CLI thin wrapper
// Calls dispatcher for config business logic, only adds CLI output
use shared::common::{ExitCode, FilePath};
use shared::config_system::IConfigOrchestratorAggregate;
use std::sync::Arc;

pub fn handle_config_show(orchestrator: Arc<dyn IConfigOrchestratorAggregate>) -> ExitCode {
    // Delegate to dispatcher
    dispatcher::surface_config_action::handle_config_show(orchestrator)
}
