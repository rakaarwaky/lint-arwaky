// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real MaintenanceContainer).
use std::sync::Arc;

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::FilePath;

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
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.py"), "pass").unwrap();
    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 1);
}

#[tokio::test]
async fn container_orchestrator_doctor_works() {
    let c = container();
    let orch = c.orchestrator();
    let result = orch.doctor().await;
    assert!(!result.python_version.value().is_empty());
}

#[tokio::test]
async fn container_orchestrator_diagnose_toolchain_works() {
    let c = container();
    let orch = c.orchestrator();
    let diag = orch.diagnose_toolchain().await;
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
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_err());
}

// ─── Full pipeline: stats -> doctor -> diagnose ───

#[tokio::test]
async fn full_maintenance_pipeline_sequential() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("main.py"), "print('hello')").unwrap();
    std::fs::write(dir.path().join("test_main.py"), "def test(): pass").unwrap();
    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 2);
    assert_eq!(stats.test_files.value(), 1);
    let doctor = orch.doctor().await;
    assert!(!doctor.python_version.value().is_empty());
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());
}
