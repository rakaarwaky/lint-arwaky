// E2E tests — TUI flow.
#[test]
fn e2e_tui_container_creates_and_usable() {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = tui_lint_arwaky::root_tui_container::TuiContainer::new(fs);
    let _executor = container.lint_executor();
}
