// Integration tests — TUI container and surfaces.

#[test]
fn tui_container_type_exists() {
    fn assert_type<T>() {}
    assert_type::<tui_lint_arwaky::root_tui_container::TuiContainer>();
}

#[test]
fn tui_surface_lint_executor_type_exists() {
    fn assert_type<T>() {}
    assert_type::<tui_lint_arwaky::surface_lint_action::SurfaceLintExecutor>();
}
