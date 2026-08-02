// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
// AES406 compliant: delegates all operations through MaintenanceCommandsAggregate.
// No direct std::process::Command or filesystem I/O — aggregate handles subprocess execution.
use shared::common::ExitCode;
use shared::common::FilePath;
use shared::maintenance::MaintenanceCommandsAggregate;
use std::sync::Arc;

fn status_icon(is_ok: bool) -> &'static str {
    if std::env::var_os("NO_COLOR").is_some() {
        if is_ok { "[OK]  " } else { "[FAIL]" }
    } else {
        if is_ok { "✓" } else { "✗" }
    }
}

pub fn handle_doctor(maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> ExitCode {
    let diag = maintenance.diagnose_toolchain();

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
    let fp = FilePath::new(target.clone()).unwrap_or_default();

    println!("Security Vulnerability Scan — {}", target);
    println!();

    let report = maintenance.run_security_scan(&fp);

    if !report.tool_installed {
        println!("No Cargo.lock found — skipping Rust security scan.");
        return ExitCode::OK;
    }

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);
    println!("Findings: {}", report.findings.len());
    for f in &report.findings {
        println!("  {} {} {}", f.severity.to_uppercase(), f.test_id, f.file);
    }

    ExitCode::OK
}

pub fn handle_dependencies(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = FilePath::new(target.clone()).unwrap_or_default();

    println!("Dependency Report — {}", target);
    println!();

    match maintenance.run_dependency_report(&fp) {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            for dep in report.dependencies.iter().take(100) {
                println!("  {} {}", dep.name, dep.version);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::RUNTIME_ERROR;
        }
    }

    ExitCode::OK
}
