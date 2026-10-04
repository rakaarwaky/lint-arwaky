// E2E tests — dispatcher scan flow.
mod common;

use shared_common::taxonomy_path_vo::FilePath;

#[test]
fn e2e_check_action_full_flow() {
    let path = FilePath::new(".").unwrap();
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: std::sync::Arc::new(
            dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
                workspace: c.workspace(),
                parser: c.parser(),
                aggregate: c.orchestrator(),
            },
        ),
        scan_aggregates: Some(common::build_scan_aggregates(".")),
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok());
}
