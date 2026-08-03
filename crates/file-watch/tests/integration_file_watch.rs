// Integration tests — full DI wiring via FileWatchContainer.
use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use std::sync::Arc;

#[test]
fn container_creates_with_default() {
    let _container = FileWatchContainer::new();
}

#[test]
fn container_default_creates() {
    let _container = FileWatchContainer::default();
}

#[test]
fn container_provider_accessible() {
    let container = FileWatchContainer::new();
    let _provider = container.provider();
}

#[test]
fn container_aggregate_needs_linter() {
    let container = FileWatchContainer::new();
    // Need a linter to create the aggregate
    let qa = quality_rules::CodeAnalysisContainer::new();
    let _agg = container.aggregate(qa.code_analysis_linter());
}

#[test]
fn container_aggregate_is_trait_object() {
    let container = FileWatchContainer::new();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let agg = container.aggregate(qa.code_analysis_linter());
    let _: Arc<dyn IWatchAggregate> = agg;
}

#[test]
fn is_lintable_via_protocol() {
    use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
    let analyzer = file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("main.rs"));
    assert!(analyzer.is_lintable("app.py"));
    assert!(analyzer.is_lintable("index.ts"));
    assert!(!analyzer.is_lintable("image.png"));
    assert!(!analyzer.is_lintable("data.bin"));
}
