// Acceptance tests — TUI operations.
#[test]
fn acceptance_tui_types_exist() {
    let _ = std::any::type_name::<tui_lint_arwaky::surface_tui_command::TuiCommand>();
}
