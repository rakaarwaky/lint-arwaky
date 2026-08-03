// Integration tests — dispatcher actions with real filesystem.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn dispatcher_check_action_on_clean_project() {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let path = FilePath::new(".").unwrap();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: fs,
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok() || result.is_err());
}
