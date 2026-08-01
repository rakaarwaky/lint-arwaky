// PURPOSE: E2E test — full maintenance lifecycle on Python and Rust projects.
// Layer: E2E (full request lifecycle, no internal mocks).
use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::FilePath;

#[tokio::test]
async fn full_maintenance_lifecycle_on_python_project() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::create_dir_all(dir.path().join("tests")).unwrap();
    std::fs::write(dir.path().join("src/app.py"), "def main(): pass\n").unwrap();
    std::fs::write(dir.path().join("src/utils.py"), "def helper(): pass\n").unwrap();
    std::fs::write(
        dir.path().join("tests/test_app.py"),
        "def test_main(): pass\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("requirements.txt"),
        "flask==2.3.0\nrequests>=2.28\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();

    // Act 1: stats
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 3);
    assert_eq!(stats.test_files.value(), 1);
    assert!(stats.test_ratio.value() > 0.0);

    // Act 2: diagnose_toolchain
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());
    assert!(!diag.vcs_tools.is_empty());

    // Act 3: run_security_scan (bandit path since no Cargo.lock/package.json)
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
}

#[tokio::test]
async fn full_maintenance_lifecycle_on_rust_project() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        r#"[package]
name = "test-proj"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("Cargo.lock"),
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
    std::fs::write(dir.path().join("src/main.rs"), "fn main() {}\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();

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
}

#[tokio::test]
async fn full_maintenance_lifecycle_on_js_project() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{
  "name": "test-app",
  "dependencies": {"react": "^18.0.0"},
  "devDependencies": {"jest": "^29.0.0"}
}"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("index.js"), "console.log(1)\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();

    // Act 1: run_security_scan (npm-audit path since package.json exists)
    let scan = orch.run_security_scan(&path).await;
    assert_eq!(scan.language, "JavaScript");
    assert_eq!(scan.tool_name, "npm-audit");

    // Act 2: run_dependency_report (package.json path)
    let dep_result = orch.run_dependency_report(&path).await;
    assert!(dep_result.is_ok());
    let dep_report = dep_result.unwrap();
    assert_eq!(dep_report.language, "JavaScript");
    assert!(dep_report.dependencies.len() >= 2);
}
