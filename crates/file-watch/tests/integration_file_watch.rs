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
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let _agg = container.aggregate(qa.code_analysis_linter());
}

#[test]
fn container_aggregate_is_trait_object() {
    let container = FileWatchContainer::new();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let agg = container.aggregate(qa.code_analysis_linter());
    let _: Arc<dyn IWatchAggregate> = agg;
}

#[test]
fn container_orchestrator_returns_struct() {
    let container = FileWatchContainer::new();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let _orch = container.orchestrator(qa.code_analysis_linter());
}

#[test]
fn is_lintable_static_method() {
    assert!(
        file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(
            "main.rs"
        )
    );
    assert!(
        file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer::is_lintable("app.py")
    );
    assert!(
        file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(
            "index.ts"
        )
    );
    assert!(
        !file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(
            "image.png"
        )
    );
    assert!(
        !file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(
            "data.bin"
        )
    );
}
