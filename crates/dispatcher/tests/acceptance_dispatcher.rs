// Acceptance tests — dispatcher surface actions produce valid output.
use dispatcher_lint_arwaky::surface_output_component::ViolationItem;
use shared::cli_commands::LintResult;
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn acceptance_check_action_on_current_project() {
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

#[test]
fn acceptance_output_component_formats_results() {
    let lr = LintResult::default();
    let item = ViolationItem::from_lint_result(&lr);
    assert_eq!(item.code.code(), "");
}
