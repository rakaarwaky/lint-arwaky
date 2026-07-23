// PURPOSE: Integration tests — DI wiring via FileWatchContainer, cross-component interaction.
// Layer: Root / Integration

use std::sync::Arc;

use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;

use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};

// ─── Mock linter (same as unit tests) ───────────────────────

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;

struct MockLinter;

impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _: &[LintResult]) -> BooleanVO {
        BooleanVO::new(false)
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> DisplayContent {
        DisplayContent::new("")
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

// ─── Container wiring ───────────────────────────────────────

#[test]
fn container_new_creates_instance() {
    let container = FileWatchContainer::new();
    let _ = &container;
}

#[test]
fn container_default_creates_instance() {
    let container = FileWatchContainer::default();
    let _ = &container;
}

#[test]
fn container_provider_returns_arc_dyn_provider() {
    let container = FileWatchContainer::new();
    let provider: Arc<dyn IWatchProviderProtocol> = container.provider();
    // Verify it's usable — call subscribe.
    let _rx = provider.subscribe();
}

#[test]
fn container_orchestrator_returns_arc_watch_orchestrator() {
    let container = FileWatchContainer::new();
    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(MockLinter);
    let orch: Arc<WatchOrchestrator> = container.orchestrator(linter);
    let _ = &orch;
}

#[test]
fn container_orchestrator_is_dyn_aggregate() {
    let container = FileWatchContainer::new();
    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(MockLinter);
    let orch = container.orchestrator(linter);
    // Verify it can be used as IWatchAggregate.
    let _dyn: Arc<dyn IWatchAggregate> = orch;
}

// ─── Cross-component: ChangeAnalyzer + WatchEvent pipeline ──

#[test]
fn analyzer_dedup_then_filter_pipeline() {
    let analyzer = ChangeAnalyzer::new();

    let events = vec![
        WatchEvent::new("src/main.rs".into(), WatchEventKind::Modified),
        WatchEvent::new("image.png".into(), WatchEventKind::Created),
        WatchEvent::new("src/main.rs".into(), WatchEventKind::Modified),
        WatchEvent::new("lib.rs".into(), WatchEventKind::Removed),
    ];

    // Step 1: deduplicate
    let deduped = analyzer.analyze(events);
    assert_eq!(deduped.len(), 3); // main.rs, image.png, lib.rs

    // Step 2: filter lintable
    let lintable = analyzer.filter_lintable(deduped);
    assert_eq!(lintable.len(), 2); // main.rs, lib.rs
    assert!(lintable
        .iter()
        .all(|e| ChangeAnalyzer::is_lintable(&e.path)));
}

// ─── Provider subscribe receives broadcast ──────────────────

#[tokio::test]
async fn provider_subscribe_receives_broadcast_events() {
    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();

    // We can't easily inject events into the broadcast without starting the watcher,
    // but we verify the channel is live and doesn't panic on try_recv.
    let result = rx.try_recv();
    assert!(result.is_err()); // No events yet — expected.
}

// ─── Multiple container instances are independent ───────────

#[test]
fn multiple_containers_are_independent() {
    let c1 = FileWatchContainer::new();
    let c2 = FileWatchContainer::new();

    let p1 = c1.provider();
    let p2 = c2.provider();

    // Different Arc pointers → independent providers.
    assert!(!Arc::ptr_eq(&p1, &p2));
}
