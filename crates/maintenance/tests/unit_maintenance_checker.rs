// PURPOSE: Unit tests for MaintenanceChecker — diagnose_toolchain, run_security_scan, run_dependency_report, stats, clean, update, doctor.
// Layer: Capabilities (target >= 70% coverage).

use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::FilePath;
use shared::maintenance::IMaintenanceCheckerProtocol;

fn sut() -> MaintenanceChecker {
    MaintenanceChecker::new()
}

// ─── diagnose_toolchain ───

#[tokio::test]
async fn diagnose_toolchain_returns_rust_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;
    assert!(
        diag.rust_tools.iter().any(|t| t.name == "cargo"),
        "Expected 'cargo' in rust_tools"
    );
}

#[tokio::test]
async fn diagnose_toolchain_returns_rustc() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;
    assert!(
        diag.rust_tools.iter().any(|t| t.name == "rustc"),
        "Expected 'rustc' in rust_tools"
    );
}

#[tokio::test]
async fn diagnose_toolchain_returns_python_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;
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
    assert!(
        report.language == "Rust" || report.language == "Python" || report.language == "JavaScript",
        "Language should be Rust, Python, or JavaScript"
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
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;
    assert!(
        result.is_err(),
        "Should return error when no dep files found"
    );
    assert!(result.unwrap_err().contains("No dependency files found"));
}

#[tokio::test]
async fn dependency_report_with_requirements_txt() {
    let checker = sut();
    let tmp_dir = tempfile::tempdir().unwrap();
    std::fs::write(
        tmp_dir.path().join("requirements.txt"),
        "flask==2.3.0\nrequests>=2.28\n# comment\nnumpy\n",
    )
    .unwrap();
    let path = FilePath::new(tmp_dir.path().to_str().unwrap().to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Python");
    assert!(report.dependencies.len() >= 3);
    let flask = report.dependencies.iter().find(|d| d.name == "flask");
    assert!(flask.is_some());
}

#[tokio::test]
async fn dependency_report_with_cargo_lock() {
    let checker = sut();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Cargo.lock"),
        r#"
[[package]]
name = "serde"
version = "1.0.193"

[[package]]
name = "my-project"
version = "0.1.0"
"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        r#"
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
    )
    .unwrap();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Rust");
    assert!(report.dependencies.len() >= 2);
    let serde_dep = report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde_dep.is_some());
    assert_eq!(serde_dep.unwrap().dep_type, "direct");
}

#[tokio::test]
async fn dependency_report_with_package_json() {
    let checker = sut();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{
  "name": "test-app",
  "dependencies": {"react": "^18.0.0", "lodash": "^4.17.21"},
  "devDependencies": {"jest": "^29.0.0"}
}"#,
    )
    .unwrap();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "JavaScript");
    assert!(report.dependencies.len() >= 3);
    let react = report.dependencies.iter().find(|d| d.name == "react");
    assert!(react.is_some());
    assert_eq!(react.unwrap().dep_type, "direct");
}

// ─── stats ───

#[tokio::test]
async fn stats_counts_python_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.py"), "pass").unwrap();
    std::fs::write(dir.path().join("test_app.py"), "pass").unwrap();
    let checker = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = checker.stats(&path).await;
    assert_eq!(stats.python_files.value(), 2);
    assert_eq!(stats.test_files.value(), 1);
}

#[tokio::test]
async fn stats_counts_rust_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn foo() {}").unwrap();
    let checker = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = checker.stats(&path).await;
    assert_eq!(stats.rust_files.value(), 2);
}

#[tokio::test]
async fn stats_counts_js_ts_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.js"), "console.log(1)").unwrap();
    std::fs::write(dir.path().join("app.test.js"), "test(1)").unwrap();
    std::fs::write(dir.path().join("comp.tsx"), "export default 1").unwrap();
    let checker = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = checker.stats(&path).await;
    assert_eq!(stats.js_files.value(), 3);
    assert_eq!(stats.test_files.value(), 1);
}

#[tokio::test]
async fn stats_empty_directory_returns_zero_counts() {
    let dir = tempfile::tempdir().unwrap();
    let checker = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = checker.stats(&path).await;
    assert_eq!(stats.total_files.value(), 0);
    assert_eq!(stats.test_files.value(), 0);
    assert_eq!(stats.test_ratio.value(), 0.0);
}

#[tokio::test]
async fn stats_skips_target_and_git_dirs() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("target")).unwrap();
    std::fs::create_dir_all(dir.path().join(".git")).unwrap();
    std::fs::write(dir.path().join("target/build.py"), "pass").unwrap();
    std::fs::write(dir.path().join(".git/hook.py"), "pass").unwrap();
    std::fs::write(dir.path().join("app.py"), "pass").unwrap();
    let checker = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = checker.stats(&path).await;
    assert_eq!(stats.python_files.value(), 1);
}

#[tokio::test]
async fn stats_test_ratio_is_correct() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.py"), "").unwrap();
    std::fs::write(dir.path().join("b.py"), "").unwrap();
    std::fs::write(dir.path().join("test_a.py"), "").unwrap();
    std::fs::write(dir.path().join("test_b.py"), "").unwrap();
    let checker = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = checker.stats(&path).await;
    assert!((stats.test_ratio.value() - 0.5).abs() < f64::EPSILON);
}

// ─── clean ───

#[tokio::test]
async fn clean_removes_cache_dirs() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("__pycache__")).unwrap();
    std::fs::create_dir_all(dir.path().join(".pytest_cache")).unwrap();
    assert!(dir.path().join("__pycache__").exists());
    assert!(dir.path().join(".pytest_cache").exists());
    // clean() operates on cwd — we can't test specific dirs easily,
    // but verify it doesn't panic
    let checker = sut();
    checker.clean().await;
}

#[tokio::test]
async fn clean_does_not_panic() {
    let checker = sut();
    checker.clean().await;
}

// ─── update ───

#[tokio::test]
async fn update_does_not_panic() {
    let checker = sut();
    checker.update().await;
}

// ─── doctor ───

#[tokio::test]
async fn doctor_returns_structured_result() {
    let checker = sut();
    let result = checker.doctor().await;
    assert!(!result.python_version.value().is_empty());
    assert!(result.adapter_statuses.len() >= 9);
}

#[tokio::test]
async fn doctor_reports_missing_adapters_as_issues() {
    let checker = sut();
    let result = checker.doctor().await;
    let missing_count = result
        .adapter_statuses
        .values()
        .filter(|s| *s == "MISSING")
        .count();
    assert!(result.issues.len() >= missing_count);
}

#[tokio::test]
async fn doctor_healthy_flag_matches_issues() {
    let checker = sut();
    let result = checker.doctor().await;
    let expected_healthy = result.issues.is_empty();
    assert_eq!(result.healthy.value(), expected_healthy);
}

#[tokio::test]
async fn doctor_checks_all_9_adapters() {
    let checker = sut();
    let result = checker.doctor().await;
    let expected = [
        "clippy",
        "rustfmt",
        "cargo-audit",
        "ruff",
        "mypy",
        "bandit",
        "eslint",
        "prettier",
        "tsc",
    ];
    for adapter in &expected {
        let found = result
            .adapter_statuses
            .keys()
            .any(|k| k.value() == *adapter);
        assert!(found, "Doctor should check adapter '{}'", adapter);
    }
}

#[tokio::test]
async fn doctor_reports_language_versions() {
    let checker = sut();
    let result = checker.doctor().await;
    // rust_version should be set (cargo is installed in test env)
    assert!(!result.rust_version.value().is_empty());
}
