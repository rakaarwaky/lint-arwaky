// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real MaintenanceContainer).

use std::sync::Arc;

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;

fn container() -> MaintenanceContainer {
    MaintenanceContainer::new()
}

// ─── Container wiring ───

#[test]
fn container_creates_orchestrator_successfully() {
    let c = container();
    let orch = c.orchestrator();
    assert!(Arc::strong_count(&orch) >= 1);
}

#[test]
fn container_orchestrator_returns_same_arc_on_multiple_calls() {
    let c = container();
    let orch1 = c.orchestrator();
    let orch2 = c.orchestrator();
    assert!(Arc::ptr_eq(&orch1, &orch2));
}

// ─── Orchestrator via container ───

#[tokio::test]
async fn container_orchestrator_stats_works() {
    let dir = "/tmp/test_integration_stats_xyz";
    let _ = std::fs::create_dir_all(dir);
    std::fs::write(format!("{}/app.py", dir), "pass").unwrap();

    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert_eq!(stats.python_files.value(), 1);

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn container_orchestrator_doctor_works() {
    let c = container();
    let orch = c.orchestrator();
    let result = orch.doctor().await;

    // Should return a valid DoctorResultVO regardless of environment
    assert!(!result.python_version.value().is_empty());
}

#[tokio::test]
async fn container_orchestrator_diagnose_toolchain_works() {
    let c = container();
    let orch = c.orchestrator();
    let diag = orch.diagnose_toolchain().await;

    // In a Rust test environment, cargo must be present
    assert!(diag.rust_tools.iter().any(|t| t.name == "cargo"));
}

#[tokio::test]
async fn container_orchestrator_security_scan_does_not_panic() {
    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new("/tmp/nonexistent_integ_scan".to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;

    assert!(!report.tool_name.is_empty());
}

#[tokio::test]
async fn container_orchestrator_dependency_report_returns_error_for_empty() {
    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new("/tmp/nonexistent_integ_dep".to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;

    assert!(result.is_err());
}

// ─── Full pipeline: stats → doctor → diagnose ───

#[tokio::test]
async fn full_maintenance_pipeline_sequential() {
    let dir = "/tmp/test_integration_pipeline_xyz";
    let _ = std::fs::create_dir_all(dir);
    std::fs::write(format!("{}/main.py", dir), "print('hello')").unwrap();
    std::fs::write(format!("{}/test_main.py", dir), "def test(): pass").unwrap();

    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    // Step 1: stats
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 2);
    assert_eq!(stats.test_files.value(), 1);

    // Step 2: doctor
    let doctor = orch.doctor().await;
    assert!(!doctor.python_version.value().is_empty());

    // Step 3: diagnose_toolchain
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());

    let _ = std::fs::remove_dir_all(dir);
}
