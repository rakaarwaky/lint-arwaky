// Integration tests — dispatcher actions with real filesystem.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn dispatcher_check_action_on_clean_project() {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let path = FilePath::new(".").unwrap();
    // Should not panic
    let _ = dispatcher_lint_arwaky::surface_check_action::CheckAction::execute(&path);
}
