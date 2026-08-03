// E2E tests — file watch flow: create container → analyze changes → verify.
use shared::file_watch::IWatchAggregate;
use std::sync::Arc;

fn make_orch() -> Arc<dyn IWatchAggregate> {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer::new(fs).orchestrator()
}

#[test]
fn e2e_file_watch_container_creates() {
    let _orch = make_orch();
}

#[test]
fn e2e_analyze_nonexistent_path() {
    let orch = make_orch();
    // Should not panic on nonexistent path
    let _ = orch.analyze_changes("/tmp/nonexistent_path_12345");
}
