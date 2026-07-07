use cli_commands_lint_arwaky::surface_check_action::find_workspace_root;

#[test]
fn find_workspace_root_returns_some_for_project() {
    let result = find_workspace_root(".");
    assert!(
        result.is_some(),
        "should find workspace root from project dir"
    );
}

#[test]
fn find_workspace_root_returns_none_for_nonexistent_path() {
    let result = find_workspace_root("/nonexistent/path/xyz_123_test_only");
    assert!(result.is_none(), "nonexistent path should return None");
}
