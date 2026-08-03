// E2E tests — file watch flow: create container → analyze changes → verify.
use shared::file_watch::IWatchAggregate;
use std::sync::Arc;

fn make_orch() -> Arc<dyn IWatchAggregate> {
    let container = file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer::new();
    let linter = quality_rules::CodeAnalysisContainer::new();
    container.aggregate(linter.code_analysis_linter())
}

#[test]
fn e2e_file_watch_container_creates() {
    let _orch = make_orch();
}
