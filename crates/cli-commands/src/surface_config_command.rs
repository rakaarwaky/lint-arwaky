// PURPOSE: Config show — CLI thin wrapper
// Calls dispatcher for config business logic, only adds CLI output.
use shared_common::ExitCode;
use shared_config_system::IConfigOrchestratorAggregate;
use std::sync::Arc;
use tracing::warn;

pub fn handle_config_show(orchestrator: Arc<dyn IConfigOrchestratorAggregate>) -> ExitCode {
    let report = dispatcher::surface_config_action::collect_config_show(orchestrator);

    for entry in &report.entries {
        println!("── [{}] {} ──", entry.language, entry.path);
        println!("{}", entry.content);
    }
    for w in &report.warnings {
        warn!(warning = %w);
    }
    if report.entries.is_empty() {
        println!("No config file found. Run `lint-arwaky init` to create one.");
        return ExitCode::OK;
    }
    // A config that exists but does not validate is a prerequisite failure:
    // the linters are not running with the settings on disk, and printing the
    // file without saying so is how a typo looks like a passing setup.
    if !report.errors.is_empty() {
        println!();
        println!("Configuration is invalid (exit code {}):", ExitCode::PREREQUISITE_MISSING.value());
        for error in &report.errors {
            println!("  - {error}");
        }
        return ExitCode::PREREQUISITE_MISSING;
    }
    ExitCode::OK
}

pub struct _Aes303Placeholder {
    _placeholder: (),
}
