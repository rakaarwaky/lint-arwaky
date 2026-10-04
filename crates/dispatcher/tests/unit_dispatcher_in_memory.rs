// Unit tests — in-memory linter dispatch (no current_exe subprocess fallback).
//
// Pins the architecture invariant: the `scan_aggregates: None` fallback used to
// spawn `lint-arwaky-cli` as a subprocess via `std::env::current_exe()`.
// That path is now forbidden (Semgrep rust.lang.security.current-exe), so
// `collect_scan` with `None` aggregates must return a clear error instead of
// spawning any process.
mod common;

use dispatcher_lint_arwaky::surface_check_action::collect_scan;
use shared_common::taxonomy_path_vo::FilePath;

fn opts_without_aggregates() -> dispatcher_lint_arwaky::surface_check_action::ScanOptions {
    dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(FilePath::new(".").unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: {
            let c = filesystem::root_filesystem_container::FilesystemContainer::new();
            std::sync::Arc::new(
                dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
                    workspace: c.workspace(),
                    parser: c.parser(),
                    aggregate: c.orchestrator(),
                },
            )
        },
        scan_aggregates: None,
    }
}

/// `scan_aggregates: None` must error, not spawn a subprocess.
#[test]
fn collect_scan_without_aggregates_returns_error() {
    let result = collect_scan(opts_without_aggregates());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.contains("scan_aggregates is required"),
        "expected the required-aggregates error, got: {err}"
    );
}

/// `scan_aggregates: Some(..)` still runs in-process and succeeds.
#[test]
fn collect_scan_with_aggregates_runs_in_memory() {
    let agg = common::build_scan_aggregates(".");
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(FilePath::new(".").unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: agg.fs_seam.clone(),
        scan_aggregates: Some(agg),
    };
    assert!(collect_scan(opts).is_ok());
}

/// `collect_role` is in-memory — a nonexistent root fails fast with the
/// documented error, not a spawn error.
#[test]
fn collect_role_missing_path_errors() {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let role = role_rules::root_role_rules_container::RoleContainer::new_with_config(
        shared_config_system::taxonomy_config_system_vo::ArchitectureConfig::default(),
    )
    .orchestrator();
    let result = dispatcher_lint_arwaky::surface_role_action::collect_role(
        Some(FilePath::new("/nonexistent/role/path").unwrap()),
        role,
        None,
        c.orchestrator(),
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

/// `collect_external` is in-memory — a nonexistent root fails fast.
#[test]
fn collect_external_missing_path_errors() {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = c.orchestrator();
    let fs_io = c.io();
    let fs_tools = c.tool_resolution();
    let external = external_lint::root_external_lint_container::ExternalLintContainer::new(
        fs.clone(),
        fs_io,
        fs_tools,
    )
    .aggregate();
    let config_container =
        config_system::root_config_system_container::ConfigContainer::new(fs.clone(), c.io());
    let result = dispatcher_lint_arwaky::surface_external_action::collect_external(
        Some(FilePath::new("/nonexistent/external/path").unwrap()),
        external,
        None,
        fs,
        config_container.parser(),
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}
