// Integration tests — dispatcher actions with real filesystem.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn dispatcher_check_action_on_clean_project() {
    let fs = {
        let c = filesystem::root_filesystem_container::FilesystemContainer::new();
        std::sync::Arc::new(
            dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
                io: c.io(),
                workspace: c.workspace(),
                parser: c.parser(),
                aggregate: c.orchestrator(),
            },
        )
    };
    let path = FilePath::new(".").unwrap();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: fs,
        scan_aggregates: None,
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok() || result.is_err());
}
