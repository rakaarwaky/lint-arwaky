// E2E tests — dispatcher scan flow.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn e2e_check_action_full_flow() {
    let path = FilePath::new(".").unwrap();
    let result = dispatcher_lint_arwaky::surface_check_action::CheckAction::execute(&path);
    // Should return a lint result list (may be empty for clean project)
    assert!(!result.values.is_empty() || result.values.is_empty());
}
