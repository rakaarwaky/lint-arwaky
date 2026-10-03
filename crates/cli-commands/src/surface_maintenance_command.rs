// PURPOSE: Maintenance — CLI thin wrapper
// Calls shared_structure_rules::surface_maintenance_action for maintenance business logic, only adds CLI output.
use shared_common::ExitCode;
use shared_common::FilePath;
use shared_maintenance::IMaintenanceAggregate;
use std::sync::Arc;
use tracing::error;

use shared_cli_commands::utility_output_text_formatter::status_icon;

pub fn handle_doctor(maintenance: Arc<dyn IMaintenanceAggregate>) -> ExitCode {
    let diag = shared_structure_rules::surface_maintenance_action::collect_doctor(maintenance);

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
    maintenance: Arc<dyn IMaintenanceAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    match shared_structure_rules::surface_maintenance_action::collect_security(maintenance, path) {
        Ok(report) => {
            println!("Security Vulnerability Scan — {}", target);
            println!();

            if !report.tool_installed {
                println!(
                    "WARNING: No dependency vulnerability scanner available for {} — install `{}` to enable this check. No clean result was produced.",
                    report.language, report.tool_name
                );
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

/// `update` — self-update the CLI binary from the latest GitHub release.
///
/// `check_only` reports whether a newer release exists without downloading.
pub fn handle_self_update(
    maintenance: Arc<dyn IMaintenanceAggregate>,
    check_only: bool,
) -> ExitCode {
    let result = shared_structure_rules::surface_maintenance_action::collect_self_update(
        maintenance,
        check_only,
    );

    println!("Lint Arwaky Self-Update");
    println!();
    println!("  Current version: {}", result.current_version);
    if result.latest_version.is_empty() {
        error!(error = %result.status, "update failed");
        println!("  {}", result.status);
        return ExitCode::RUNTIME_ERROR;
    }
    println!("  Latest release:  {}", result.latest_version);
    println!(
        "  {} {}",
        status_icon(result.upgraded || result.already_up_to_date),
        result.status
    );
    if check_only {
        println!();
        println!("  Check only — no binary was downloaded or replaced.");
    }

    ExitCode::OK
}

pub fn handle_dependencies(
    maintenance: Arc<dyn IMaintenanceAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    match shared_structure_rules::surface_maintenance_action::collect_dependencies(
        maintenance,
        path,
    ) {
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
