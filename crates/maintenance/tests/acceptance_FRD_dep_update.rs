// PURPOSE: Acceptance test — FRD requirement: dep-update (update dependencies across workspace).
// Maps 1:1 to FRD requirement: "dep-update — update Rust/Python/JS dependencies across the workspace."

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

/// FRD-dep-update: The system can analyze project dependencies and produce a structured report.
/// This validates the "dependency_report" capability that underpins the update workflow.
#[tokio::test]
async fn frd_dep_update_dependency_report_identifies_direct_and_transitive() {
    let dir = "/tmp/acceptance_dep_update_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();

    std::fs::write(
        format!("{}/Cargo.toml", dir),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(
        format!("{}/Cargo.lock", dir),
        "[[package]]\nname = \"serde\"\nversion = \"1.0.193\"\n\n[[package]]\nname = \"serde_derive\"\nversion = \"1.0.193\"\n\n[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_ok(), "Dependency report should succeed");

    let report = result.unwrap();
    assert_eq!(report.language, "Rust");

    // serde is a direct dependency
    let serde = report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde.is_some(), "serde should be in dependencies");
    assert_eq!(serde.unwrap().dep_type, "direct");

    // serde_derive is transitive
    let derive = report
        .dependencies
        .iter()
        .find(|d| d.name == "serde_derive");
    assert!(derive.is_some(), "serde_derive should be in dependencies");
    assert_eq!(derive.unwrap().dep_type, "transitive");

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-dep-update: The update command executes without error (upgrades linter tools via pip).
#[tokio::test]
async fn frd_dep_update_update_command_executes_without_panic() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();

    // update() calls pip install --upgrade for ruff, mypy, bandit, radon
    // We just verify it doesn't panic (actual pip may or may not be installed)
    orch.update().await;
}
