// PURPOSE: Setup — CLI thin wrapper
// Calls dispatcher for setup business logic, only adds CLI output
use shared::common::ExitCode;
use shared::project_setup::SetupManagementAggregate;
use std::sync::Arc;

pub fn handle_init(setup_orchestrator: Arc<dyn SetupManagementAggregate>) -> ExitCode {
    dispatcher::surface_setup_action::handle_init(setup_orchestrator)
}

pub fn handle_install(setup: Arc<dyn SetupManagementAggregate>, sudo: bool) -> ExitCode {
    dispatcher::surface_setup_action::handle_install(setup, sudo)
}

pub fn handle_mcp_config(client: &str) -> ExitCode {
    dispatcher::surface_setup_action::handle_mcp_config(client)
}
