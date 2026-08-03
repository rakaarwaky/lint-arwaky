// PURPOSE: Config show — CLI thin wrapper
// Calls dispatcher for config business logic, only adds CLI output.
use shared::common::ExitCode;
use shared::config_system::IConfigOrchestratorAggregate;
use std::sync::Arc;

pub fn handle_config_show(orchestrator: Arc<dyn IConfigOrchestratorAggregate>) -> ExitCode {
    let report = dispatcher::surface_config_action::collect_config_show(orchestrator);

    for entry in &report.entries {
        println!("── [{}] {} ──", entry.language, entry.path);
        println!("{}", entry.content);
    }
    for w in &report.warnings {
        eprintln!("{w}");
    }
    if report.entries.is_empty() {
        println!("No config file found. Run `lint-arwaky init` to create one.");
    }
    ExitCode::OK
}
