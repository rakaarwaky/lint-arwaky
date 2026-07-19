use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;

#[tokio::test]
async fn test_diagnose_toolchain_returns_cargo() {
    let checker = MaintenanceChecker::new();
    let diag = checker.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());
    let cargo = &diag.rust_tools[0];
    assert_eq!(cargo.name, "cargo");
    assert_eq!(cargo.status, "OK");
}

#[tokio::test]
async fn test_diagnose_toolchain_has_git() {
    let checker = MaintenanceChecker::new();
    let diag = checker.diagnose_toolchain().await;
    let git = diag.vcs_tools.iter().find(|t| t.name == "git");
    assert!(git.is_some());
    assert_eq!(git.unwrap().status, "OK");
}

#[tokio::test]
async fn test_diagnose_toolchain_returns_non_empty_sections() {
    let checker = MaintenanceChecker::new();
    let diag = checker.diagnose_toolchain().await;
    assert!(!diag.python_tools.is_empty());
    assert!(!diag.js_tools.is_empty());
}

#[tokio::test]
async fn test_dependency_report_no_lockfile() {
    let checker = MaintenanceChecker::new();
    let path = FilePath::new("/nonexistent_path".to_string()).unwrap_or_default();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_dependency_report_with_requirements_txt() {
    let dir = std::env::temp_dir().join("lint_arwaky_test_maint");
    let _ = fs::create_dir_all(&dir);
    let reqs = dir.join("requirements.txt");
    fs::write(&reqs, "requests==2.28.0\nflask>=2.0\n# comment\n").unwrap();

    let checker = MaintenanceChecker::new();
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Python");
    assert_eq!(report.dependencies.len(), 2);
    assert_eq!(report.dependencies[0].name, "requests==2.28.0");

    let _ = fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_dependency_report_with_cargo_lock() {
    let dir = std::env::temp_dir().join("lint_arwaky_test_maint_cargo");
    let _ = fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        "[dependencies]\nserde = \"1.0\"\ntokio = \"1.0\"\n",
    )
    .unwrap();
    let cargo_lock = dir.join("Cargo.lock");
    fs::write(
        &cargo_lock,
        "[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"tokio\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"autocfg\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();

    let checker = MaintenanceChecker::new();
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Rust");
    assert_eq!(report.dependencies.len(), 3);

    let serde = report
        .dependencies
        .iter()
        .find(|d| d.name == "serde")
        .unwrap();
    assert_eq!(serde.dep_type, "direct");

    let autocfg = report
        .dependencies
        .iter()
        .find(|d| d.name == "autocfg")
        .unwrap();
    assert_eq!(autocfg.dep_type, "transitive");

    let _ = fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_security_scan_returns_report() {
    let checker = MaintenanceChecker::new();
    let path = FilePath::new("/nonexistent_path".to_string()).unwrap_or_default();
    let report = checker.run_security_scan(&path).await;
    assert_eq!(report.language, "Python");
    assert_eq!(report.tool_name, "bandit");
}
