// Smoke tests — verify maintenance container and core operations complete within 5s.
use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::maintenance::MaintenanceCommandsAggregate;

#[test]
fn maintenance_container_creates() {
    let start = std::time::Instant::now();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let _container = MaintenanceContainer::new(filesystem);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn maintenance_orchestrator_creates() {
    let start = std::time::Instant::now();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = MaintenanceContainer::new(filesystem);
    let _orch = container.orchestrator();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn maintenance_orchestrator_is_trait_object() {
    let start = std::time::Instant::now();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = MaintenanceContainer::new(filesystem);
    let orch = container.orchestrator();
    let _: std::sync::Arc<dyn MaintenanceCommandsAggregate> = orch;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
