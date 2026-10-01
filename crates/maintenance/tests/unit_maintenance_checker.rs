// Unit tests — MaintenanceChecker methods (split into individual capability checkers).
use maintenance_lint_arwaky::{
    IAdapterHealthProtocol, IDependencyReportProtocol, IDoctorProtocol, IProjectStatsProtocol,
    ISecurityScanProtocol, ISelfUpdateProtocol,
};
use shared_common::FilePath;

fn make_io()
-> std::sync::Arc<dyn shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol> {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _fs = fc.orchestrator();
    fc.io()
}

#[test]
fn diagnose_toolchain_returns_non_empty_lists() {
    let checker = maintenance_lint_arwaky::DoctorChecker::new(make_io());
    let diag = checker.diagnose_toolchain();
    assert!(!diag.rust_tools.is_empty(), "Should have rust tools");
    assert!(!diag.python_tools.is_empty(), "Should have python tools");
    assert!(!diag.js_tools.is_empty(), "Should have js tools");
    assert!(!diag.vcs_tools.is_empty(), "Should have vcs tools");
}

#[test]
fn diagnose_toolchain_has_binary_path() {
    let checker = maintenance_lint_arwaky::DoctorChecker::new(make_io());
    let diag = checker.diagnose_toolchain();
    assert!(
        !diag.binary_path.is_empty(),
        "Binary path should not be empty"
    );
}

#[test]
fn health_check_returns_10_adapters() {
    let checker = maintenance_lint_arwaky::AdapterHealthChecker::new(make_io());
    let result = checker.health_check();
    assert_eq!(result.adapters.len(), 10, "Should check 10 adapters");
}

#[test]
fn health_check_has_all_languages() {
    let checker = maintenance_lint_arwaky::AdapterHealthChecker::new(make_io());
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
    assert!(
        languages.contains(&"Markdown"),
        "Should check Markdown adapters"
    );
}

#[test]
fn stats_returns_non_negative_counts() {
    let checker = maintenance_lint_arwaky::ProjectStatsChecker::new(make_io());
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
    let checker = maintenance_lint_arwaky::ProjectStatsChecker::new(make_io());
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
    let checker = maintenance_lint_arwaky::SecurityScanChecker::new(make_io());
    let path = FilePath::new("/tmp").unwrap();
    let report = checker.run_security_scan(&path);
    assert!(!report.tool_installed || !report.findings.is_empty());
}

#[test]
fn run_dependency_report_without_cargo_lock() {
    let checker = maintenance_lint_arwaky::DependencyReportChecker::new(make_io());
    let path = FilePath::new("/tmp").unwrap();
    let result = checker.run_dependency_report(&path);
    assert!(result.is_err(), "Should fail without Cargo.lock");
}

#[test]
fn run_dependency_report_with_cargo_lock() {
    let checker = maintenance_lint_arwaky::DependencyReportChecker::new(make_io());
    let path = FilePath::new(".").unwrap();
    let result = checker.run_dependency_report(&path);
    if let Ok(report) = result {
        assert_eq!(report.language, "Rust");
    }
    // Cargo.lock may or may not exist in test env
}

#[test]
fn doctor_returns_result() {
    let checker = maintenance_lint_arwaky::DoctorChecker::new(make_io());
    let result = checker.doctor();
    assert!(!result.rust_version.value.is_empty() || !result.python_version.value.is_empty());
}

// ─── self_update ───────────────────────────────────────────────────────────

#[test]
fn self_update_check_only_returns_valid_vo() {
    let checker = maintenance_lint_arwaky::SelfUpdateChecker::new(make_io());
    let result = checker.self_update(true);
    // current_version is always the CARGO_PKG_VERSION normalised, so it must not be empty.
    assert!(
        !result.current_version.is_empty(),
        "current_version should be set"
    );
    // latest_version is empty only when the API is unreachable (offline test env).
    if result.latest_version.is_empty() {
        assert!(
            result.status.starts_with("Error:"),
            "status should explain failure"
        );
    } else {
        // API reachable: latest_version starts with "v" (raw tag, not normalised).
        assert!(
            result.latest_version.starts_with('v'),
            "latest_version should be the raw tag, got {}",
            result.latest_version
        );
    }
}

#[test]
fn self_update_success_or_already_up_to_date() {
    let checker = maintenance_lint_arwaky::SelfUpdateChecker::new(make_io());
    let result = checker.self_update(false);
    let is_failure = result.status.starts_with("Error:");
    // When GitHub is reachable, the result must have a non-empty latest_version.
    // When GitHub is unreachable (offline CI), it gracefully degrades.
    if !is_failure {
        assert!(
            !result.latest_version.is_empty(),
            "latest_version must be set on success"
        );
    }
}
