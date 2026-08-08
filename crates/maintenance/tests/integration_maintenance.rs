// Integration tests — full DI wiring via MaintenanceContainer.
use shared::common::FilePath;
use shared::maintenance::MaintenanceCommandsAggregate;
use std::sync::Arc;

fn make_container() -> maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer::new(fs)
}

#[test]
fn container_creates_successfully() {
    let _container = make_container();
}

#[test]
fn container_returns_orchestrator() {
    let container = make_container();
    let orch = container.orchestrator();
    let _: Arc<dyn MaintenanceCommandsAggregate> = orch;
}

#[test]
fn orchestrator_stats_on_current_dir() {
    let container = make_container();
    let orch = container.orchestrator();
    let path = FilePath::new(".").unwrap();
    let stats = orch.stats(&path);
    assert!(stats.total_files.value >= 0);
}

#[test]
fn orchestrator_diagnose_toolchain() {
    let container = make_container();
    let orch = container.orchestrator();
    let diag = orch.diagnose_toolchain();
    assert!(!diag.rust_tools.is_empty());
}

#[test]
fn orchestrator_health_check() {
    let container = make_container();
    let orch = container.orchestrator();
    let result = orch.health_check();
    assert_eq!(result.adapters.len(), 9);
}

#[test]
fn orchestrator_doctor() {
    let container = make_container();
    let orch = container.orchestrator();
    let result = orch.doctor();
    assert!(!result.rust_version.value.is_empty() || !result.python_version.value.is_empty());
}

#[test]
fn orchestrator_security_scan() {
    let container = make_container();
    let orch = container.orchestrator();
    let path = FilePath::new(".").unwrap();
    let report = orch.run_security_scan(&path);
    assert!(!report.language.is_empty());
}

#[test]
fn orchestrator_dependency_report() {
    let container = make_container();
    let orch = container.orchestrator();
    let path = FilePath::new(".").unwrap();
    let result = orch.run_dependency_report(&path);
    if let Ok(report) = result {
        assert_eq!(report.language, "Rust");
    }
}

#[test]
fn orchestrator_clean_does_not_panic() {
    let container = make_container();
    let orch = container.orchestrator();
    orch.clean();
}

#[test]
fn orchestrator_cancel_does_not_panic() {
    let container = make_container();
    let orch = container.orchestrator();
    orch.cancel(shared::common::taxonomy_action_vo::JobId::default());
}
