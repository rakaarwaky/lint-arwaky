// PURPOSE: Unit tests for MaintenanceCommandsOrchestrator — stats, clean, doctor, cancel.
// Layer: Agent (target >= 60% coverage).

use maintenance_lint_arwaky::agent_maintenance_orchestrator::{
    MaintenanceCommandsOrchestrator, MaintenanceDeps,
};
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::FilePath;
use shared::common::taxonomy_job_id_vo::JobId;
use shared::maintenance::{IMaintenanceCheckerProtocol, MaintenanceCommandsAggregate};

use std::sync::Arc;

fn sut() -> MaintenanceCommandsOrchestrator {
    let checker: Arc<dyn IMaintenanceCheckerProtocol> = Arc::new(MaintenanceChecker::new());
    MaintenanceCommandsOrchestrator::new(MaintenanceDeps { checker })
}

// ─── stats ───

#[tokio::test]
async fn stats_counts_python_files_in_directory() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("main.py"), "print('hi')").unwrap();
    std::fs::write(dir.path().join("utils.py"), "pass").unwrap();
    std::fs::write(dir.path().join("test_main.py"), "def test(): pass").unwrap();
    std::fs::write(dir.path().join("README.md"), "# Readme").unwrap();
    let orch = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 3);
    assert_eq!(stats.test_files.value(), 1);
    assert!(stats.test_ratio.value() > 0.0);
}

#[tokio::test]
async fn stats_empty_directory_returns_zero_counts() {
    let dir = tempfile::tempdir().unwrap();
    let orch = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
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
    let orch = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 1);
}

#[tokio::test]
async fn stats_test_ratio_is_correct_fraction() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.py"), "").unwrap();
    std::fs::write(dir.path().join("b.py"), "").unwrap();
    std::fs::write(dir.path().join("test_a.py"), "").unwrap();
    std::fs::write(dir.path().join("test_b.py"), "").unwrap();
    let orch = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert!((stats.test_ratio.value() - 0.5).abs() < f64::EPSILON);
}

#[tokio::test]
async fn stats_counts_multi_language_project() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.py"), "pass").unwrap();
    std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.path().join("index.js"), "console.log(1)").unwrap();
    let orch = sut();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 1);
    assert_eq!(stats.rust_files.value(), 1);
    assert_eq!(stats.js_files.value(), 1);
    assert_eq!(stats.total_files.value(), 3);
}

// ─── clean ───

#[tokio::test]
async fn clean_does_not_panic() {
    let orch = sut();
    orch.clean().await;
}

// ─── doctor ───

#[tokio::test]
async fn doctor_returns_structured_result() {
    let orch = sut();
    let result = orch.doctor().await;
    assert!(!result.python_version.value().is_empty());
    assert!(result.adapter_statuses.len() >= 9);
}

#[tokio::test]
async fn doctor_reports_missing_adapters_as_issues() {
    let orch = sut();
    let result = orch.doctor().await;
    let missing_count = result
        .adapter_statuses
        .values()
        .filter(|s| *s == "MISSING")
        .count();
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
    assert!(result.is_err());
}
