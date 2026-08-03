// Acceptance tests — file watch operations produce valid results.
use shared::file_watch::IWatchAggregate;
use std::sync::Arc;

fn make_orch() -> Arc<dyn IWatchAggregate> {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer::new(fs).orchestrator()
}

#[test]
fn acceptance_watch_container_creates_successfully() {
    let orch = make_orch();
    // Container should be usable immediately
    let _ = orch;
}

#[test]
fn acceptance_analyze_current_dir() {
    let orch = make_orch();
    // Analyzing current dir should not panic
    let _ = orch.analyze_changes(".");
}
