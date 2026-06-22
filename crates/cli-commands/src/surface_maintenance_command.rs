// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub struct MaintenanceCommandsSurface;

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
            if status.status == "OK" { "✓" } else { "✗" },
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
            if status.status == "OK" { "✓" } else { "✗" },
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
            if status.status == "OK" { "✓" } else { "✗" },
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
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    ExitCode::SUCCESS
}

pub async fn handle_security(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<String>,
) -> ExitCode {
    let target = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fp = match shared::source_parsing::taxonomy_path_vo::FilePath::new(target.clone()) {
        Ok(fp) => fp,
        Err(_) => shared::source_parsing::taxonomy_path_vo::FilePath::default(),
    };
    println!("Security Vulnerability Scan — {}", target);
    println!();

    let report = maintenance_orchestrator.run_security_scan(&fp).await;

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);

    if !report.tool_installed {
        println!("{} not available. Please install it.", report.tool_name);
        return ExitCode::SUCCESS;
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

    ExitCode::SUCCESS
}

pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<String>,
) -> ExitCode {
    let target = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fp = match shared::source_parsing::taxonomy_path_vo::FilePath::new(target.clone()) {
        Ok(fp) => fp,
        Err(_) => shared::source_parsing::taxonomy_path_vo::FilePath::default(),
    };
    println!("Dependency Report — {}", target);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");
            for dep in report.dependencies.iter().take(30) {
                println!("{:<25} {:<12} {}", dep.name, dep.version, dep.dep_type);
            }
            if report.dependencies.len() > 30 {
                println!("... and {} more", report.dependencies.len() - 30);
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }

    ExitCode::SUCCESS
}
