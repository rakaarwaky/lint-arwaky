use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::mcp_server::taxonomy_action_vo::JobId;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use std::fs;

#[tokio::test]
async fn test_stats_returns_non_negative() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let path = FilePath::new(
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    )
    .unwrap_or_default();
    let stats = orch.stats(&path).await;
    assert!(stats.total_files.value >= 0);
    assert!(stats.test_files.value >= 0);
    assert!(stats.test_ratio.value >= 0.0);
    assert!(stats.test_ratio.value <= 1.0);
}

#[tokio::test]
async fn test_stats_counts_python_files() {
    let dir = std::env::temp_dir().join("lint_arwaky_test_stats");
    let _ = fs::create_dir_all(&dir);
    fs::write(dir.join("a.py"), "x = 1").unwrap();
    fs::write(dir.join("test_b.py"), "def test_x(): pass").unwrap();
    fs::write(dir.join("readme.txt"), "hello").unwrap();

    let orch = MaintenanceCommandsOrchestrator::default();
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value, 2);
    assert_eq!(stats.test_files.value, 1);

    let _ = fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_doctor_returns_result() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let result = orch.doctor().await;
    assert_eq!(result.python_version.value, "3.12");
}

#[tokio::test]
async fn test_clean_does_not_panic() {
    let orch = MaintenanceCommandsOrchestrator::default();
    // clean is idempotent — removing non-existent cache dirs is a no-op
    orch.clean().await;
}

#[tokio::test]
async fn test_cancel_does_not_panic() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let job_id = JobId::new("test-job".to_string());
    orch.cancel(job_id).await;
}

#[tokio::test]
async fn test_diagnose_toolchain_via_orchestrator() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty(), "should have rust tools");
    assert!(!diag.python_tools.is_empty(), "should have python tools");
    assert!(!diag.vcs_tools.is_empty(), "should have VCS tools");
    let cargo = &diag.rust_tools[0];
    assert_eq!(cargo.name, "cargo");
}

#[tokio::test]
async fn test_security_scan_via_orchestrator_for_nonexistent_path() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let path = FilePath::new("/nonexistent_test_path_xyz".to_string()).unwrap_or_default();
    let report = orch.run_security_scan(&path).await;
    // No lockfile → falls through to Python bandit, which won't find /nonexistent path
    assert_eq!(report.language, "Python");
    assert_eq!(report.tool_name, "bandit");
}

#[tokio::test]
async fn test_dependency_report_via_orchestrator_fails_for_nonexistent() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let path = FilePath::new("/nonexistent_path_xyz".to_string()).unwrap_or_default();
    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_err(), "expected error for nonexistent path");
    let err = result.unwrap_err();
    assert!(
        err.contains("No dependency files found"),
        "should mention no dependency files"
    );
}

#[tokio::test]
async fn test_container_creates() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    )
    .unwrap_or_default();
    let stats = orch.stats(&path).await;
    assert!(stats.total_files.value >= 0);
}
