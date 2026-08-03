// E2E tests — dispatcher scan flow.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn e2e_check_action_full_flow() {
    let path = FilePath::new(".").unwrap();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator(),
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok());
}
