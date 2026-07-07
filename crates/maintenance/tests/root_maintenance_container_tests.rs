// PURPOSE: Integration tests for MaintenanceContainer — DI wiring for maintenance commands
use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

#[test]
fn container_new_and_default_construct() {
    let c1 = MaintenanceContainer::new();
    let _ = c1.orchestrator();
    let c2: MaintenanceContainer = Default::default();
    let _ = c2.orchestrator();
}

#[tokio::test]
async fn container_orchestrator_stats_via_aggregate_trait() {
    let container = MaintenanceContainer::new();
    let orch_arc = container.orchestrator();
    let orch: &dyn MaintenanceCommandsAggregate = orch_arc.as_ref();
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

#[tokio::test]
async fn container_orchestrator_doctor_via_aggregate_trait() {
    let container = MaintenanceContainer::new();
    let orch_arc = container.orchestrator();
    let orch: &dyn MaintenanceCommandsAggregate = orch_arc.as_ref();
    let result = orch.doctor().await;
    assert_eq!(result.python_version.value, "3.12");
}

#[tokio::test]
async fn container_orchestrator_clean_does_not_panic() {
    let container = MaintenanceContainer::new();
    let orch_arc = container.orchestrator();
    let orch: &dyn MaintenanceCommandsAggregate = orch_arc.as_ref();
    orch.clean().await;
}

#[tokio::test]
async fn container_orchestrator_cancel_does_not_panic() {
    let container = MaintenanceContainer::new();
    let orch_arc = container.orchestrator();
    let orch: &dyn MaintenanceCommandsAggregate = orch_arc.as_ref();
    orch.cancel(shared::mcp_server::taxonomy_action_vo::JobId::new(
        "test".to_string(),
    ))
    .await;
}
