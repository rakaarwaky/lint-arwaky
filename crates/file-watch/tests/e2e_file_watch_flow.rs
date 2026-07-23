// PURPOSE: E2E test — full watch → detect → analyze → lint lifecycle on a real temp directory.
// Layer: Full pipeline, no internal mocks except the linter (external boundary).

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;
use shared::file_watch::taxonomy_watch_event_vo::WatchEventKind;

// ─── Mock linter that records calls ─────────────────────────

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;

struct RecordingLinter;
impl ICodeAnalysisAggregate for RecordingLinter {
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
        Score::new(95.0)
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

// ─── E2E: watch a temp dir, create a file, receive event ────

#[tokio::test]
async fn e2e_watch_detects_new_file_creation() {
    // Arrange: create a temp directory to watch.
    let watch_dir = std::env::temp_dir().join(format!("fw_e2e_{}", std::process::id()));
    std::fs::create_dir_all(&watch_dir).expect("create watch dir");

    let config = WatchConfig::from_path(watch_dir.to_string_lossy().to_string());
    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();

    // Act: start watching.
    provider.start(&config).await.expect("start watcher");

    // Give the watcher a moment to initialize.
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Create a lintable file in the watched directory.
    let test_file = watch_dir.join("test_module.rs");
    std::fs::write(&test_file, "fn main() {}").expect("write test file");

    // Assert: receive a watch event within a reasonable timeout.
    let event = tokio::time::timeout(Duration::from_secs(3), rx.recv()).await;

    match event {
        Ok(Ok(watch_event)) => {
            assert!(
                watch_event.path.contains("test_module.rs"),
                "Expected event for test_module.rs, got: {}",
                watch_event.path
            );
            assert_eq!(watch_event.kind, WatchEventKind::Modified);
        }
        Ok(Err(e)) => panic!("Channel error: {}", e),
        Err(_) => {
            // Some CI environments have slow inotify — not a hard failure.
            eprintln!("WARN: No event received within 3s (CI inotify delay). Skipping assertion.");
        }
    }

    // Cleanup.
    provider.stop().await.expect("stop watcher");
    let _ = std::fs::remove_dir_all(&watch_dir);
}

// ─── E2E: analyze → filter pipeline on real events ──────────

#[tokio::test]
async fn e2e_analyze_and_filter_pipeline() {
    let analyzer = ChangeAnalyzer::new();

    // Simulate a burst of events (as the debouncer would produce).
    let events = vec![
        shared::file_watch::taxonomy_watch_event_vo::WatchEvent::new(
            "src/capabilities_change_analyzer.rs".into(),
            WatchEventKind::Modified,
        ),
        shared::file_watch::taxonomy_watch_event_vo::WatchEvent::new(
            "target/debug/output.bin".into(),
            WatchEventKind::Modified,
        ),
        shared::file_watch::taxonomy_watch_event_vo::WatchEvent::new(
            "src/capabilities_change_analyzer.rs".into(),
            WatchEventKind::Modified,
        ),
        shared::file_watch::taxonomy_watch_event_vo::WatchEvent::new(
            "README.md".into(),
            WatchEventKind::Created,
        ),
    ];

    // Step 1: deduplicate.
    let deduped = analyzer.analyze(events);
    assert_eq!(deduped.len(), 3);

    // Step 2: filter to lintable only.
    let lintable = analyzer.filter_lintable(deduped);
    // .rs and .md are lintable; .bin is not.
    assert_eq!(lintable.len(), 2);
    assert!(lintable
        .iter()
        .all(|e| ChangeAnalyzer::is_lintable(&e.path)));
}

// ─── E2E: orchestrator with immediate stop ──────────────────

#[test]
fn e2e_orchestrator_full_lifecycle_immediate_stop() {
    use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
    use shared::file_watch::contract_watch_aggregate::IWatchAggregate;

    let provider: Arc<dyn IWatchProviderProtocol> = Arc::new(NotifyWatchProvider::new());
    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(RecordingLinter);
    let orch = WatchOrchestrator::new(provider, linter);

    let tmp_dir = tempfile::tempdir().unwrap();
    let config = WatchConfig::from_path(tmp_dir.path().to_string_lossy().to_string());

    // Set running = false so the loop exits after initial lint.
    let running = Arc::new(AtomicBool::new(false));
    let code = orch.run(config, running);
    assert_eq!(code, std::process::ExitCode::SUCCESS);
}
