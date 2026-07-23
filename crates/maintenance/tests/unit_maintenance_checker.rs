// PURPOSE: Unit tests for MaintenanceChecker — diagnose_toolchain, run_security_scan, run_dependency_report.
// Layer: Capabilities (target ≥ 70% coverage).

use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::maintenance::contract_maintenance_protocol::IMaintenanceCheckerProtocol;

fn sut() -> MaintenanceChecker {
    MaintenanceChecker::new()
}

// ─── diagnose_toolchain ───

#[tokio::test]
async fn diagnose_toolchain_returns_rust_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    // cargo is required — should always be present in a Rust test environment
    assert!(
        diag.rust_tools.iter().any(|t| t.name == "cargo"),
        "Expected 'cargo' in rust_tools"
    );
}

#[tokio::test]
async fn diagnose_toolchain_returns_python_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    // python3 is optional — list should still be populated
    assert!(
        !diag.python_tools.is_empty(),
        "python_tools should not be empty"
    );
}

#[tokio::test]
async fn diagnose_toolchain_returns_js_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    assert!(!diag.js_tools.is_empty(), "js_tools should not be empty");
}

#[tokio::test]
async fn diagnose_toolchain_returns_vcs_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    assert!(
        diag.vcs_tools.iter().any(|t| t.name == "git"),
        "Expected 'git' in vcs_tools"
    );
}

#[tokio::test]
async fn diagnose_toolchain_binary_path_is_not_empty() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    assert!(
        !diag.binary_path.is_empty(),
        "binary_path should resolve to current exe"
    );
}

#[tokio::test]
async fn diagnose_toolchain_tool_status_has_valid_status_values() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    let all_tools = diag
        .rust_tools
        .iter()
        .chain(diag.python_tools.iter())
        .chain(diag.js_tools.iter())
        .chain(diag.vcs_tools.iter());

    for tool in all_tools {
        assert!(
            ["OK", "WARN", "FAIL"].contains(&tool.status.as_str()),
            "Tool '{}' has invalid status '{}'",
            tool.name,
            tool.status
        );
    }
}

// ─── run_security_scan ───

#[tokio::test]
async fn security_scan_nonexistent_path_returns_report() {
    let checker = sut();
    let path = FilePath::new("/tmp/nonexistent_project_xyz_12345").unwrap();
    let report = checker.run_security_scan(&path).await;

    // Should not panic; returns a report (possibly empty findings)
    assert!(
        report.language == "Rust" || report.language == "Python",
        "Language should be Rust or Python"
    );
}

#[tokio::test]
async fn security_scan_report_has_tool_name() {
    let checker = sut();
    let path = FilePath::new("/tmp/nonexistent_project_xyz_12345").unwrap();
    let report = checker.run_security_scan(&path).await;

    assert!(
        !report.tool_name.is_empty(),
        "tool_name should not be empty"
    );
}

// ─── run_dependency_report ───

#[tokio::test]
async fn dependency_report_no_files_returns_error() {
    let checker = sut();
    let path = FilePath::new("/tmp/empty_dir_no_deps_xyz_99999").unwrap();

    // Create the empty dir so path exists but has no dependency files
    let _ = std::fs::create_dir_all("/tmp/empty_dir_no_deps_xyz_99999");

    let result = checker.run_dependency_report(&path).await;
    assert!(
        result.is_err(),
        "Should return error when no dep files found"
    );
    assert!(result.unwrap_err().contains("No dependency files found"));

    // Cleanup
    let _ = std::fs::remove_dir_all("/tmp/empty_dir_no_deps_xyz_99999");
}

#[tokio::test]
async fn dependency_report_with_requirements_txt() {
    let checker = sut();
    let tmp_dir = tempfile::tempdir().unwrap();
    let dir = tmp_dir.path().to_str().unwrap();
    std::fs::write(
        tmp_dir.path().join("requirements.txt"),
        "flask==2.3.0\nrequests>=2.28\n# comment\nnumpy\n",
    )
    .unwrap();

    let path = FilePath::new(dir.to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Python");
    assert!(report.dependencies.len() >= 3); // flask, requests, numpy (comment skipped)

    let flask = report.dependencies.iter().find(|d| d.name == "flask");
    assert!(flask.is_some());
}

#[tokio::test]
async fn dependency_report_with_cargo_lock() {
    let checker = sut();
    let dir = "/tmp/test_dep_report_cargo_xyz";
    let _ = std::fs::create_dir_all(dir);

    let cargo_lock = r#"
[[package]]
name = "serde"
version = "1.0.193"

[[package]]
name = "my-project"
version = "0.1.0"
"#;
    std::fs::write(format!("{}/Cargo.lock", dir), cargo_lock).unwrap();

    let cargo_toml = r#"
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
    std::fs::write(format!("{}/Cargo.toml", dir), cargo_toml).unwrap();

    let path = FilePath::new(dir.to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Rust");
    assert!(report.dependencies.len() >= 2);

    let serde_dep = report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde_dep.is_some());
    assert_eq!(serde_dep.unwrap().dep_type, "direct");

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}
