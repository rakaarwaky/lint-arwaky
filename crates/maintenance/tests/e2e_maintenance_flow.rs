// PURPOSE: E2E test — full maintenance lifecycle: create project → stats → scan → report → clean.
// Layer: E2E (full request lifecycle, no internal mocks).

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

#[tokio::test]
async fn full_maintenance_lifecycle_on_python_project() {
    // Arrange: create a temporary Python project
    let dir = "/tmp/e2e_maintenance_python_proj";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(format!("{}/src", dir)).unwrap();
    std::fs::create_dir_all(format!("{}/tests", dir)).unwrap();

    std::fs::write(format!("{}/src/app.py", dir), "def main(): pass\n").unwrap();
    std::fs::write(format!("{}/src/utils.py", dir), "def helper(): pass\n").unwrap();
    std::fs::write(
        format!("{}/tests/test_app.py", dir),
        "def test_main(): pass\n",
    )
    .unwrap();
    std::fs::write(
        format!("{}/requirements.txt", dir),
        "flask==2.3.0\nrequests>=2.28\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    // Act 1: stats
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 3);
    assert_eq!(stats.test_files.value(), 1);
    assert!(stats.test_ratio.value() > 0.0);

    // Act 2: diagnose_toolchain
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());
    assert!(!diag.vcs_tools.is_empty());

    // Act 3: run_security_scan (bandit path since no Cargo.lock)
    let scan = orch.run_security_scan(&path).await;
    assert_eq!(scan.language, "Python");
    assert_eq!(scan.tool_name, "bandit");

    // Act 4: run_dependency_report (requirements.txt path)
    let dep_result = orch.run_dependency_report(&path).await;
    assert!(dep_result.is_ok());
    let dep_report = dep_result.unwrap();
    assert_eq!(dep_report.language, "Python");
    assert!(dep_report.dependencies.len() >= 2);

    // Act 5: doctor
    let doctor = orch.doctor().await;
    assert!(!doctor.python_version.value().is_empty());

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn full_maintenance_lifecycle_on_rust_project() {
    // Arrange: create a temporary Rust project structure
    let dir = "/tmp/e2e_maintenance_rust_proj";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(format!("{}/src", dir)).unwrap();

    std::fs::write(
        format!("{}/Cargo.toml", dir),
        r#"[package]
name = "test-proj"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
    )
    .unwrap();

    std::fs::write(
        format!("{}/Cargo.lock", dir),
        r#"
[[package]]
name = "serde"
version = "1.0.193"

[[package]]
name = "test-proj"
version = "0.1.0"
"#,
    )
    .unwrap();

    std::fs::write(format!("{}/src/main.rs", dir), "fn main() {}\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    // Act 1: run_security_scan (cargo-audit path)
    let scan = orch.run_security_scan(&path).await;
    assert_eq!(scan.language, "Rust");
    assert_eq!(scan.tool_name, "cargo-audit");

    // Act 2: run_dependency_report (Cargo.lock path)
    let dep_result = orch.run_dependency_report(&path).await;
    assert!(dep_result.is_ok());
    let dep_report = dep_result.unwrap();
    assert_eq!(dep_report.language, "Rust");

    let serde_dep = dep_report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde_dep.is_some());
    assert_eq!(serde_dep.unwrap().version, "1.0.193");
    assert_eq!(serde_dep.unwrap().dep_type, "direct");

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}
