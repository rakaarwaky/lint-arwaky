// Integration tests — TUI container and surfaces.
use std::sync::Arc;

#[test]
fn tui_container_creates() {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let _container = tui_lint_arwaky::root_tui_container::TuiContainer::new(fs);
}

#[test]
fn tui_container_returns_components() {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let container = tui_lint_arwaky::root_tui_container::TuiContainer::new(fs);
    let _executor = container.lint_executor();
}
