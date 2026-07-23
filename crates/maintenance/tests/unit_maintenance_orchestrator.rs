// PURPOSE: Unit tests for MaintenanceCommandsOrchestrator — stats, clean, doctor, cancel.
// Layer: Agent (target ≥ 60% coverage).

use maintenance_lint_arwaky::agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_job_id_vo::JobId;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use std::sync::Arc;

fn sut() -> MaintenanceCommandsOrchestrator {
    let checker: Arc<dyn IMaintenanceCheckerProtocol> = Arc::new(MaintenanceChecker::new());
    MaintenanceCommandsOrchestrator::new(MaintenanceDeps { checker })
}

// ─── stats ───

#[tokio::test]
async fn stats_counts_python_files_in_directory() {
    let dir = "/tmp/test_stats_py_xyz";
    let _ = std::fs::create_dir_all(format!("{}/src", dir));
    std::fs::write(format!("{}/src/main.py", dir), "print('hi')").unwrap();
    std::fs::write(format!("{}/src/utils.py", dir), "pass").unwrap();
    std::fs::write(format!("{}/src/test_main.py", dir), "def test(): pass").unwrap();
    std::fs::write(format!("{}/README.md", dir), "# Readme").unwrap();

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert_eq!(stats.python_files.value(), 3); // main.py, utils.py, test_main.py
    assert_eq!(stats.test_files.value(), 1); // test_main.py
    assert!(stats.test_ratio.value() > 0.0);

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stats_empty_directory_returns_zero_counts() {
    let dir = "/tmp/test_stats_empty_xyz";
    let _ = std::fs::create_dir_all(dir);

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert_eq!(stats.total_files.value(), 0);
    assert_eq!(stats.test_files.value(), 0);
    assert_eq!(stats.test_ratio.value(), 0.0);

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stats_skips_target_and_git_dirs() {
    let dir = "/tmp/test_stats_skip_xyz";
    let _ = std::fs::create_dir_all(format!("{}/target", dir));
    let _ = std::fs::create_dir_all(format!("{}/.git", dir));
    std::fs::write(format!("{}/target/build.py", dir), "pass").unwrap();
    std::fs::write(format!("{}/.git/hook.py", dir), "pass").unwrap();
    std::fs::write(format!("{}/app.py", dir), "pass").unwrap();

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    // Only app.py should be counted (target/ and .git/ are skipped)
    assert_eq!(stats.python_files.value(), 1);

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stats_test_ratio_is_correct_fraction() {
    let dir = "/tmp/test_stats_ratio_xyz";
    let _ = std::fs::create_dir_all(dir);
    // 4 python files, 2 are tests → ratio = 0.5
    std::fs::write(format!("{}/a.py", dir), "").unwrap();
    std::fs::write(format!("{}/b.py", dir), "").unwrap();
    std::fs::write(format!("{}/test_a.py", dir), "").unwrap();
    std::fs::write(format!("{}/test_b.py", dir), "").unwrap();

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert!((stats.test_ratio.value() - 0.5).abs() < f64::EPSILON);

    let _ = std::fs::remove_dir_all(dir);
}

// ─── clean ───

#[tokio::test]
async fn clean_does_not_panic() {
    let orch = sut();
    // clean() operates on cwd — just verify it doesn't panic
    orch.clean().await;
}

// ─── doctor ───

#[tokio::test]
async fn doctor_returns_structured_result() {
    let orch = sut();
    let result = orch.doctor().await;

    // python_version should be set
    assert!(!result.python_version.value().is_empty());

    // adapter_statuses should have entries for ruff, mypy, bandit, radon
    assert!(result.adapter_statuses.len() >= 4);
}

#[tokio::test]
async fn doctor_reports_missing_adapters_as_issues() {
    let orch = sut();
    let result = orch.doctor().await;

    // If any adapter is missing, there should be a corresponding issue
    let missing_count = result
        .adapter_statuses
        .values()
        .filter(|s| *s == "MISSING")
        .count();

    // issues should include at least the missing adapter messages
    // (plus possibly "No configuration file found")
    assert!(result.issues.len() >= missing_count);
}

#[tokio::test]
async fn doctor_healthy_flag_matches_issues() {
    let orch = sut();
    let result = orch.doctor().await;

    let expected_healthy = result.issues.is_empty();
    assert_eq!(result.healthy.value(), expected_healthy);
}

// ─── cancel ───

#[tokio::test]
async fn cancel_does_not_panic_with_arbitrary_job_id() {
    let orch = sut();
    let job_id = JobId::new("test-job-001".to_string());
    orch.cancel(job_id).await;
    // No assertion needed — just verifying no panic
}

// ─── diagnose_toolchain (delegated) ───

#[tokio::test]
async fn diagnose_toolchain_delegates_to_checker() {
    let orch = sut();
    let diag = orch.diagnose_toolchain().await;

    assert!(!diag.rust_tools.is_empty());
    assert!(!diag.binary_path.is_empty());
}

// ─── run_security_scan (delegated) ───

#[tokio::test]
async fn run_security_scan_delegates_to_checker() {
    let orch = sut();
    let path = FilePath::new("/tmp/nonexistent_scan_xyz".to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;

    assert!(!report.tool_name.is_empty());
}

// ─── run_dependency_report (delegated) ───

#[tokio::test]
async fn run_dependency_report_delegates_to_checker() {
    let orch = sut();
    let path = FilePath::new("/tmp/nonexistent_dep_xyz".to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;

    // Should return error since no dep files exist
    assert!(result.is_err());
}