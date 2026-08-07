// Acceptance tests — TUI operations.
#[test]
fn acceptance_tui_types_exist() {
    fn assert_type<T>() {}
    assert_type::<tui_lint_arwaky::root_tui_container::TuiContainer>();
    assert_type::<tui_lint_arwaky::surface_lint_action::SurfaceLintExecutor>();
}
