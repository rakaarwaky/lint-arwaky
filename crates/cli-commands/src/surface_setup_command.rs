// PURPOSE: Setup — CLI thin wrapper
// Calls dispatcher for setup business logic, only adds CLI output.
use shared::common::ExitCode;
use shared::project_setup::SetupManagementAggregate;
use std::sync::Arc;

pub fn handle_init(setup_orchestrator: Arc<dyn SetupManagementAggregate>) -> ExitCode {
    let items = dispatcher::surface_setup_action::collect_init(setup_orchestrator);
    let mut all_ok = true;
    for item in &items {
        if item.ok {
            println!("{}", item.message);
        } else {
            println!("{}", item.message);
            all_ok = false;
        }
    }
    if all_ok {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

pub fn handle_install(setup: Arc<dyn SetupManagementAggregate>, sudo: bool) -> ExitCode {
    let report = dispatcher::surface_setup_action::collect_install(setup, sudo);

    println!("Lint Arwaky — Install Adapter Dependencies");
    println!("{}", "=".repeat(50));

    println!("\n[1/2] Installing Python adapters (ruff, mypy, bandit)...");
    if report.py_ok {
        println!("  Python adapters installed");
    } else {
        println!("  Failed to install Python adapters");
    }

    println!("\n[2/2] Installing JavaScript adapters (eslint, prettier, typescript)...");
    if report.js_ok {
        println!("  JavaScript adapters installed");
    } else {
        println!("  Failed to install JavaScript adapters");
    }

    println!("\n{}", "=".repeat(50));
    if report.py_ok && report.js_ok {
        println!("Done! Run `lint-arwaky doctor` to verify.");
        ExitCode::OK
    } else {
        println!("Installation failed. Run with `--sudo` if npm globally requires permissions.");
        ExitCode::POLICY_FAIL
    }
}

pub fn handle_mcp_config(client: &str) -> ExitCode {
    let report = dispatcher::surface_setup_action::collect_mcp_config(client);
    println!("MCP Client Configuration for: {}", report.client);
    println!("Binary: {}", report.binary);
    println!();
    println!("{}", report.config_json);
    ExitCode::OK
}
