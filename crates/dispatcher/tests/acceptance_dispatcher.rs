// Acceptance tests — dispatcher surface actions produce valid output.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn acceptance_check_action_on_current_project() {
    let path = FilePath::new(".").unwrap();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(path),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: std::sync::Arc::new(shared::filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()),
    };
    let result = dispatcher_lint_arwaky::surface_check_action::collect_scan(opts);
    assert!(result.is_ok());
}

#[test]
fn acceptance_output_component_formats_results() {
    let results = shared::common::taxonomy_lint_result_vo::LintResultList::new(Vec::new());
    let items = vec![];
    let _output = dispatcher_lint_arwaky::surface_output_component::ViolationItem::from_lint_result(&items);
}
