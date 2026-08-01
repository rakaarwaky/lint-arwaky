// PURPOSE: Acceptance test — FR-002 Stats, FR-003 Clean, FR-004 Update, FR-006 Security, FR-007 Deps.
// Maps 1:1 to FRD requirements for stats, cache cleanup, tool updates, security scanning, dependency reporting.

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::FilePath;

// ─── FR-002: Stats ───

/// FR-002: Stats counts files per language in a multi-language project.
#[tokio::test]
async fn frd_002_stats_counts_multi_language() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.py"), "pass").unwrap();
    std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.path().join("index.js"), "console.log(1)").unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 1);
    assert_eq!(stats.rust_files.value(), 1);
    assert_eq!(stats.js_files.value(), 1);
    assert_eq!(stats.total_files.value(), 3);
}

/// FR-002: Stats identifies test files per language.
#[tokio::test]
async fn frd_002_stats_identifies_test_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.py"), "pass").unwrap();
    std::fs::write(dir.path().join("test_app.py"), "pass").unwrap();
    std::fs::write(dir.path().join("lib.rs"), "pub fn foo() {}").unwrap();
    std::fs::write(dir.path().join("test_lib.rs"), "#[test] fn t() {}").unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.test_files.value(), 2);
}

/// FR-002: Stats on empty directory returns all zeros.
#[tokio::test]
async fn frd_002_stats_empty_directory() {
    let dir = tempfile::tempdir().unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.total_files.value(), 0);
    assert_eq!(stats.test_ratio.value(), 0.0);
}

// ─── FR-003: Clean ───

/// FR-003: Clean removes __pycache__ directories (no-op test since clean works on cwd).
#[tokio::test]
async fn frd_003_clean_does_not_panic() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    orch.clean().await;
}

// ─── FR-004: Update ───

/// FR-004: Update executes without panic (pip + npm).
#[tokio::test]
async fn frd_004_update_executes_without_panic() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    orch.update().await;
}

// ─── FR-006: Security Scan ───

/// FR-006: Security scan on Rust project uses cargo-audit.
#[tokio::test]
async fn frd_006_security_scan_rust_uses_cargo_audit() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Cargo.lock"),
        "[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;
    assert_eq!(report.language, "Rust");
    assert_eq!(report.tool_name, "cargo-audit");
}

/// FR-006: Security scan on Python project uses bandit.
#[tokio::test]
async fn frd_006_security_scan_python_uses_bandit() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("app.py"), "import os\n").unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;
    assert_eq!(report.language, "Python");
    assert_eq!(report.tool_name, "bandit");
}

/// FR-006: Security scan on JS/TS project uses npm audit.
#[tokio::test]
async fn frd_006_security_scan_js_uses_npm_audit() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("package.json"), "{\"name\": \"app\"}").unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;
    assert_eq!(report.language, "JavaScript");
    assert_eq!(report.tool_name, "npm-audit");
}

// ─── FR-007: Dependency Report ───

/// FR-007: Dependency report on Rust project parses Cargo.lock + Cargo.toml.
#[tokio::test]
async fn frd_007_dep_report_rust_parses_cargo() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Cargo.lock"),
        r#"
[[package]]
name = "serde"
version = "1.0.193"
"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0\"\n",
    )
    .unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Rust");
    let serde = report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde.is_some());
    assert_eq!(serde.unwrap().dep_type, "direct");
}

/// FR-007: Dependency report on Python project parses pyproject.toml/requirements.txt.
#[tokio::test]
async fn frd_007_dep_report_python_parses_requirements() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("requirements.txt"),
        "flask==2.3.0\nrequests>=2.28\n",
    )
    .unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Python");
    assert!(report.dependencies.len() >= 2);
}

/// FR-007: Dependency report on JS/TS project parses package.json.
#[tokio::test]
async fn frd_007_dep_report_js_parses_package_json() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{
  "name": "app",
  "dependencies": {"react": "^18.0.0"},
  "devDependencies": {"jest": "^29.0.0"}
}"#,
    )
    .unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "JavaScript");
    assert!(report.dependencies.len() >= 2);
    let react = report.dependencies.iter().find(|d| d.name == "react");
    assert!(react.is_some());
}

/// FR-007: Dependency report with no dependency files returns error.
#[tokio::test]
async fn frd_007_dep_report_no_files_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_err());
}
