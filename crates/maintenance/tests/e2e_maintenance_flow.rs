// E2E tests — full maintenance flow: diagnose → health check → stats → doctor.
use shared::common::FilePath;
use shared::maintenance::IMaintenanceAggregate;
use shared::maintenance::MaintenanceRequest;

fn make_orch() -> std::sync::Arc<dyn IMaintenanceAggregate> {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fc.orchestrator();
    let io = fc.io();
    maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer::new(fs, io)
        .orchestrator()
}

#[test]
fn e2e_full_maintenance_pipeline() {
    let orch = make_orch();

    // Step 1: Diagnose toolchain
    let diag = orch
        .execute(MaintenanceRequest::diagnose_toolchain())
        .into_toolchain();
    assert!(
        !diag.rust_tools.is_empty(),
        "Step 1: should have rust tools"
    );

    // Step 2: Health check
    let health = orch
        .execute(MaintenanceRequest::health_check())
        .into_health();
    assert_eq!(
        health.adapters.len(),
        10,
        "Step 2: should check 10 adapters"
    );

    // Step 3: Stats on current project
    let path = FilePath::new(".").unwrap();
    let stats = orch.execute(MaintenanceRequest::stats(&path)).into_stats();
    assert!(stats.total_files.value > 0, "Step 3: should find files");

    // Step 4: Doctor
    let doctor = orch.execute(MaintenanceRequest::doctor()).into_doctor();
    assert!(
        !doctor.rust_version.value.is_empty() || !doctor.python_version.value.is_empty(),
        "Step 4: doctor should report versions"
    );
}

#[test]
fn e2e_security_scan_and_dependency_report() {
    let orch = make_orch();
    let path = FilePath::new(".").unwrap();

    let scan = orch
        .execute(MaintenanceRequest::security_scan(&path))
        .into_security_report();
    // Language is "Rust" if Cargo.lock exists in cwd, "Unknown" otherwise
    assert!(!scan.language.is_empty());

    let dep_report = orch
        .execute(MaintenanceRequest::dependency_report(&path))
        .into_dependency_report();
    if let Ok(report) = dep_report {
        assert_eq!(report.language, "Rust");
        // The project has dependencies in Cargo.lock
    }
}
