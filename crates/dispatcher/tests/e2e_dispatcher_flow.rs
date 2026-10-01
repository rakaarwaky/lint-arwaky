// E2E tests — dispatcher scan flow.
use shared_common::taxonomy_path_vo::FilePath;

#[test]
fn e2e_check_action_full_flow() {
    let path = FilePath::new(".").unwrap();
    let opts = dispatcher_lint_arwaky::orchestrator_check_pipeline::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: {
            let c = filesystem::root_filesystem_container::FilesystemContainer::new();
            std::sync::Arc::new(
                dispatcher_lint_arwaky::orchestrator_check_pipeline::FilesystemSeam {
                    io: c.io(),
                    workspace: c.workspace(),
                    parser: c.parser(),
                    aggregate: c.orchestrator(),
                },
            )
        },
        scan_aggregates: None,
    };
    let result = dispatcher_lint_arwaky::orchestrator_check_pipeline::collect_scan(opts);
    assert!(result.is_ok());
}
