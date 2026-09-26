// Integration tests — full DI wiring via MaintenanceContainer.
use shared::common::FilePath;
use shared::maintenance::IMaintenanceAggregate;
use shared::maintenance::MaintenanceRequest;
use std::sync::Arc;

fn make_container() -> maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fc.orchestrator();
    let io = fc.io();
    maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer::new(fs, io)
}

#[test]
fn container_creates_successfully() {
    let _container = make_container();
}

#[test]
fn container_returns_orchestrator() {
    let container = make_container();
    let orch = container.orchestrator();
    let _: Arc<dyn IMaintenanceAggregate> = orch;
}

#[test]
fn orchestrator_stats_on_current_dir() {
    let container = make_container();
    let orch = container.orchestrator();
    let path = FilePath::new(".").unwrap();
    let stats = orch.execute(MaintenanceRequest::stats(&path)).into_stats();
    assert!(stats.total_files.value >= 0);
}

#[test]
fn orchestrator_diagnose_toolchain() {
    let container = make_container();
    let orch = container.orchestrator();
    let diag = orch
        .execute(MaintenanceRequest::diagnose_toolchain())
        .into_toolchain();
    assert!(!diag.rust_tools.is_empty());
}

#[test]
fn orchestrator_health_check() {
    let container = make_container();
    let orch = container.orchestrator();
    let result = orch
        .execute(MaintenanceRequest::health_check())
        .into_health();
    assert_eq!(result.adapters.len(), 9);
}

#[test]
fn orchestrator_doctor() {
    let container = make_container();
    let orch = container.orchestrator();
    let result = orch.execute(MaintenanceRequest::doctor()).into_doctor();
    assert!(!result.rust_version.value.is_empty() || !result.python_version.value.is_empty());
}

#[test]
fn orchestrator_security_scan() {
    let container = make_container();
    let orch = container.orchestrator();
    let path = FilePath::new(".").unwrap();
    let report = orch
        .execute(MaintenanceRequest::security_scan(&path))
        .into_security_report();
    assert!(!report.language.is_empty());
}

#[test]
fn orchestrator_dependency_report() {
    let container = make_container();
    let orch = container.orchestrator();
    let path = FilePath::new(".").unwrap();
    let result = orch
        .execute(MaintenanceRequest::dependency_report(&path))
        .into_dependency_report();
    if let Ok(report) = result {
        assert_eq!(report.language, "Rust");
    }
}

#[test]
fn orchestrator_clean_does_not_panic() {
    let container = make_container();
    let orch = container.orchestrator();
    orch.execute(MaintenanceRequest::clean());
}

#[test]
fn orchestrator_cancel_does_not_panic() {
    let container = make_container();
    let orch = container.orchestrator();
    orch.execute(MaintenanceRequest::cancel(
        shared::common::taxonomy_action_vo::JobId::default(),
    ));
}
