// Unit tests — collect_scan with nonexistent path returns error.
#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

// Linked from crates/shared/tests/common/mock_filesystem.rs — the canonical
// mock lives in the shared crate's tests dir; this #[path] link keeps a single
// source of truth across the workspace (T1/D2).

use dispatcher_lint_arwaky::surface_check_action::{ScanOptions, collect_scan};
use shared::common::FilePath;
use std::sync::Arc;

use mock_filesystem::MockFilesystem;

fn mock_fs() -> Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> {
    Arc::new(MockFilesystem::new())
}

#[test]
fn collect_scan_nonexistent_path_returns_error() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path/that/does/not/exist".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: mock_fs(),
        scan_aggregates: None,
    };
    let result = collect_scan(opts);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("does not exist"));
}

#[test]
fn collect_scan_with_filter_returns_error_for_nonexistent() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: Some("AES".to_string()),
        member: None,
        filesystem: mock_fs(),
        scan_aggregates: None,
    };
    let result = collect_scan(opts);
    assert!(result.is_err());
}
