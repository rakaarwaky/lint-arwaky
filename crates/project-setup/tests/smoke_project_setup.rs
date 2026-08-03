// Smoke tests — container creation and basic operations complete within 5s.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::{ISetupManagementProtocol, SetupManagementAggregate};

#[test]
fn project_setup_container_creates_within_5s() {
    let start = std::time::Instant::now();
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let _container = SetupContainer::new(fs);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn project_setup_aggregate_accessible_within_5s() {
    let start = std::time::Instant::now();
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = SetupContainer::new(fs);
    let agg = container.aggregate();
    let _ = agg.detect_language();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn project_setup_protocol_accessible_within_5s() {
    let start = std::time::Instant::now();
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = SetupContainer::new(fs);
    let proto = container.protocol();
    let _ = proto.generate_mcp_config();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
