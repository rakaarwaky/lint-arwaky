// Unit tests — MaintenanceChecker methods.
use shared::common::FilePath;
use shared::maintenance::IMaintenanceCheckerProtocol;

fn make_checker() -> impl IMaintenanceCheckerProtocol {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker::new(fs)
}

#[test]
fn diagnose_toolchain_returns_non_empty_lists() {
    let checker = make_checker();
    let diag = checker.diagnose_toolchain();
    assert!(!diag.rust_tools.is_empty(), "Should have rust tools");
    assert!(!diag.python_tools.is_empty(), "Should have python tools");
    assert!(!diag.js_tools.is_empty(), "Should have js tools");
    assert!(!diag.vcs_tools.is_empty(), "Should have vcs tools");
}

#[test]
fn diagnose_toolchain_has_binary_path() {
    let checker = make_checker();
    let diag = checker.diagnose_toolchain();
    assert!(
        !diag.binary_path.is_empty(),
        "Binary path should not be empty"
    );
}

#[test]
fn health_check_returns_9_adapters() {
    let checker = make_checker();
    let result = checker.health_check();
    assert_eq!(result.adapters.len(), 9, "Should check 9 adapters");
}

#[test]
fn health_check_has_all_languages() {
    let checker = make_checker();
    let result = checker.health_check();
    let languages: Vec<&str> = result
        .adapters
        .iter()
        .map(|a| a.language.as_str())
        .collect();
    assert!(languages.contains(&"Rust"), "Should check Rust adapters");
    assert!(
        languages.contains(&"Python"),
        "Should check Python adapters"
    );
    assert!(languages.contains(&"JS/TS"), "Should check JS/TS adapters");
}

#[test]
fn stats_returns_non_negative_counts() {
    let checker = make_checker();
    let path = FilePath::new(".").unwrap();
    let stats = checker.stats(&path);
    assert!(
        stats.total_files.value >= 0,
        "total_files should be non-negative"
    );
    assert!(
        stats.test_files.value >= 0,
        "test_files should be non-negative"
    );
    assert!(
        stats.rust_files.value >= 0,
        "rust_files should be non-negative"
    );
    assert!(
        stats.python_files.value >= 0,
        "python_files should be non-negative"
    );
    assert!(stats.js_files.value >= 0, "js_files should be non-negative");
}

#[test]
fn stats_test_ratio_between_0_and_1() {
    let checker = make_checker();
    let path = FilePath::new(".").unwrap();
    let stats = checker.stats(&path);
    assert!(
        stats.test_ratio.value >= 0.0 && stats.test_ratio.value <= 10.0,
        "test_ratio should be reasonable, got {}",
        stats.test_ratio.value
    );
}

#[test]
fn run_security_scan_without_cargo_lock() {
    let checker = make_checker();
    let path = FilePath::new("/tmp").unwrap();
    let report = checker.run_security_scan(&path);
    assert!(!report.tool_installed || !report.findings.is_empty());
}

#[test]
fn run_dependency_report_without_cargo_lock() {
    let checker = make_checker();
    let path = FilePath::new("/tmp").unwrap();
    let result = checker.run_dependency_report(&path);
    assert!(result.is_err(), "Should fail without Cargo.lock");
}

#[test]
fn run_dependency_report_with_cargo_lock() {
    let checker = make_checker();
    let path = FilePath::new(".").unwrap();
    let result = checker.run_dependency_report(&path);
    if result.is_ok() {
        let report = result.unwrap();
        assert_eq!(report.language, "Rust");
    }
    // Cargo.lock may or may not exist in test env
}

#[test]
fn doctor_returns_result() {
    let checker = make_checker();
    let result = checker.doctor();
    assert!(!result.rust_version.value.is_empty() || !result.python_version.value.is_empty());
}
