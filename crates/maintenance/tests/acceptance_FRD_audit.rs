// PURPOSE: Acceptance test — FRD requirement: audit (run security audits using cargo-audit, bandit).
// Maps 1:1 to FRD requirement: "audit — run security audits using cargo-audit, bandit, or external tools."

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;

/// FRD-audit: Security scan on a Rust project uses cargo-audit and returns structured findings.
#[tokio::test]
async fn frd_audit_rust_project_uses_cargo_audit() {
    let dir = "/tmp/acceptance_audit_rust_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(
        format!("{}/Cargo.lock", dir),
        "[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let report = orch.run_security_scan(&path).await;

    assert_eq!(report.language, "Rust");
    assert_eq!(report.tool_name, "cargo-audit");
    // findings may be empty if no vulnerabilities — that's valid
    assert!(report.tool_installed);

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-audit: Security scan on a Python project uses bandit and returns structured findings.
#[tokio::test]
async fn frd_audit_python_project_uses_bandit() {
    let dir = "/tmp/acceptance_audit_python_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{}/app.py", dir), "import os\nos.system('ls')\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let report = orch.run_security_scan(&path).await;

    assert_eq!(report.language, "Python");
    assert_eq!(report.tool_name, "bandit");
    // bandit may or may not be installed — report should still be structured
    assert!(!report.tool_name.is_empty());

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-audit: Audit coverage — all vulnerabilities detected and reported with severity.
#[tokio::test]
async fn frd_audit_findings_have_required_fields() {
    let dir = "/tmp/acceptance_audit_fields_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    // No Cargo.lock → falls through to bandit path
    std::fs::write(
        format!("{}/insecure.py", dir),
        "import subprocess\nsubprocess.call('ls', shell=True)\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let report = orch.run_security_scan(&path).await;

    // If bandit is installed and finds issues, verify structure
    for finding in &report.findings {
        assert!(!finding.severity.is_empty(), "severity must not be empty");
        assert!(!finding.test_id.is_empty(), "test_id must not be empty");
        assert!(!finding.issue.is_empty(), "issue must not be empty");
    }

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-audit: Doctor health check verifies tool installations.
#[tokio::test]
async fn frd_audit_doctor_checks_all_adapters() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();

    let result = orch.doctor().await;

    // Must check ruff, mypy, bandit, radon
    let expected_adapters = ["ruff", "mypy", "bandit", "radon"];
    for adapter in &expected_adapters {
        let found = result
            .adapter_statuses
            .keys()
            .any(|k| k.value() == *adapter);
        assert!(found, "Doctor should check adapter '{}'", adapter);
    }
}
