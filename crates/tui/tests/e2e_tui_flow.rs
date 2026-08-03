// E2E tests — TUI type existence and key type bounds.
#[test]
fn e2e_tui_key_types_exist() {
    fn assert_type<T>() {}
    assert_type::<tui_lint_arwaky::root_tui_container::TuiContainer>();
    assert_type::<tui_lint_arwaky::surface_lint_executor::SurfaceLintExecutor>();
    assert_type::<tui_lint_arwaky::surface_action_handler::SurfaceActionHandler>();
}
