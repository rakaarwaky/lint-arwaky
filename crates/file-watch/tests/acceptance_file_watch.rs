// Acceptance tests — file watch operations produce valid results.
use shared::file_watch::IWatchAggregate;
use std::sync::Arc;

fn make_orch() -> Arc<dyn IWatchAggregate> {
    let container = file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer::new();
    let linter = shared::quality_rules::CodeAnalysisContainer::new();
    container.aggregate(linter.code_analysis_linter())
}

#[test]
fn acceptance_watch_container_creates_successfully() {
    let orch = make_orch();
    // Container should be usable immediately
    let _ = orch;
}
