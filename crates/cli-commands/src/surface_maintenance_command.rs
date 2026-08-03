// PURPOSE: Maintenance — CLI thin wrapper
// Calls dispatcher for maintenance business logic, only adds CLI output.
use shared::common::ExitCode;
use shared::common::FilePath;
use shared::maintenance::MaintenanceCommandsAggregate;
use std::sync::Arc;
use tracing::error;

use crate::surface_formatting::status_icon;

pub fn handle_doctor(maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> ExitCode {
    let diag = dispatcher::surface_maintenance_action::collect_doctor(maintenance);

    println!("Environment Diagnostics");
    println!();

    println!("Rust Toolchain:");
    for t in &diag.rust_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(t.status == "OK"),
            t.name,
            t.version,
            t.status
        );
    }

    println!();
    println!("Python Toolchain:");
    for t in &diag.python_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(t.status == "OK"),
            t.name,
            t.version,
            t.status
        );
    }

    println!();
    println!("JavaScript Toolchain:");
    for t in &diag.js_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(t.status == "OK"),
            t.name,
            t.version,
            t.status
        );
    }

    println!();
    println!("VCS:");
    for t in &diag.vcs_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(t.status == "OK"),
            t.name,
            t.version,
            t.status
        );
    }

    ExitCode::OK
}

pub fn handle_security(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    match dispatcher::surface_maintenance_action::collect_security(maintenance, path) {
        Ok(report) => {
            println!("Security Vulnerability Scan — {}", target);
            println!();

            if !report.tool_installed {
                println!("Security scanning tool not installed — cannot run scan.");
                return ExitCode::PREREQUISITE_MISSING;
            }

            println!("Language: {}", report.language);
            println!("Tool: {}", report.tool_name);
            println!("Findings: {}", report.findings.len());
            for f in &report.findings {
                println!("  {} {} {}", f.severity.to_uppercase(), f.test_id, f.file);
            }

            if report.findings.is_empty() {
                ExitCode::OK
            } else {
                ExitCode::POLICY_FAIL
            }
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

pub fn handle_dependencies(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    match dispatcher::surface_maintenance_action::collect_dependencies(maintenance, path) {
        Ok(report) => {
            println!("Dependency Report — {}", target);
            println!();
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            for dep in report.dependencies.iter().take(30) {
                println!("  {} {}", dep.name, dep.version);
            }
            if report.dependencies.len() > 30 {
                println!("  ... and {} more", report.dependencies.len() - 30);
            }
            ExitCode::OK
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
