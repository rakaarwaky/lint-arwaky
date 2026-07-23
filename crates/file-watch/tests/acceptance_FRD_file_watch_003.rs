// PURPOSE: FRD Requirement — Automatic re-trigger of linting pipeline on detected changes.
// "Automatic re-trigger of the linting pipeline on detected changes."

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

// Mock linter that tracks invocation count.
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::atomic::AtomicUsize;

static LINT_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

struct CountingLinter;
impl ICodeAnalysisAggregate for CountingLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> {
        LINT_CALL_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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

#[test]
fn frd_003_orchestrator_triggers_initial_lint_on_startup() {
    LINT_CALL_COUNT.store(0, std::sync::atomic::Ordering::SeqCst);

    let provider: Arc<dyn IWatchProviderProtocol> = Arc::new(NotifyWatchProvider::new());
    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(CountingLinter);
    let orch = WatchOrchestrator::new(provider, linter);

    let config = WatchConfig::from_path(std::env::temp_dir().to_string_lossy().to_string());
    let running = Arc::new(AtomicBool::new(false)); // exit after initial lint
    let _ = orch.run(config, running);

    // The initial full lint must have been called at least once.
    let count = LINT_CALL_COUNT.load(std::sync::atomic::Ordering::SeqCst);
    assert!(
        count >= 1,
        "Initial lint must be triggered on startup, got {} calls",
        count
    );
}
