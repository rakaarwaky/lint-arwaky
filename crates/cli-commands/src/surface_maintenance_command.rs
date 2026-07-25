// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
//
// Three independent subcommands, all delegated to MaintenanceCommandsAggregate:
//   - doctor:     toolchain diagnostics (cargo, python3, node, git, etc.)
//   - security:   vulnerability scan via cargo-audit (Rust) or bandit (Python)
//   - deps:       dependency report from Cargo.lock / pyproject.toml / requirements.txt

use shared::common::{
    ExitCode,
    FilePath,
};

use shared::maintenance::MaintenanceCommandsAggregate;
use std::sync::Arc;

fn status_icon(is_ok: bool) -> &'static str {
    if std::env::var_os("NO_COLOR").is_some() {
        if is_ok {
            "[OK]  "
        } else {
            "[FAIL]"
        }
    } else {
        if is_ok {
            "✓"
        } else {
            "✗"
        }
    }
}

pub async fn handle_doctor(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
) -> ExitCode {
    println!("Environment Diagnostics");
    println!();

    let diag = maintenance_orchestrator.diagnose_toolchain().await;

    println!("Rust Toolchain:");
    for status in &diag.rust_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(status.status == "OK"),
            status.name,
            status.version,
            status.status
        );
    }
    if !diag.binary_path.is_empty() {
        println!("  binary: {}", diag.binary_path);
    }

    println!();
    println!("Python Toolchain:");
    for status in &diag.python_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(status.status == "OK"),
            status.name,
            status.version,
            status.status
        );
    }

    println!();
    println!("JavaScript Toolchain:");
    for status in &diag.js_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(status.status == "OK"),
            status.name,
            status.version,
            status.status
        );
    }

    println!();
    println!("VCS:");
    for status in &diag.vcs_tools {
        println!(
            "  {} {} {}  ({})",
            status_icon(status.status == "OK"),
            status.name,
            status.version,
            status.status
        );
    }

    ExitCode::OK
}

pub async fn handle_security(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    println!("Security Vulnerability Scan — {}", target);
    println!();

    let report = maintenance_orchestrator.run_security_scan(&fp).await;

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);

    if !report.tool_installed {
        eprintln!("Error: {} is not installed.", report.tool_name);
        return ExitCode::PREREQUISITE_MISSING;
    }

    println!("Findings: {}", report.findings.len());
    for f in &report.findings {
        println!(
            "  {} {} {}:{} {}",
            f.severity.to_uppercase(),
            f.test_id,
            f.file,
            f.line,
            f.issue
        );
    }

    if report.findings.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    println!("Dependency Report — {}", target);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();

            let pkg_width = report
                .dependencies
                .iter()
                .map(|d| d.name.len())
                .max()
                .unwrap_or(25)
                .max(7);
            let ver_width = report
                .dependencies
                .iter()
                .map(|d| d.version.len())
                .max()
                .unwrap_or(12)
                .max(7);

            println!("{:<pkg_width$}  {:<ver_width$}  Type", "Package", "Version");
            for dep in report.dependencies.iter() {
                println!(
                    "{:<pkg_width$}  {:<ver_width$}  {}",
                    dep.name, dep.version, dep.dep_type
                );
            }
        }
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::RUNTIME_ERROR;
        }
    }

    ExitCode::OK
}
