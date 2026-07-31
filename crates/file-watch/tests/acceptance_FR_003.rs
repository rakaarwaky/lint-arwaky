// PURPOSE: FRD Requirement — Automatic re-trigger of linting pipeline on detected changes.
// "Automatic re-trigger of the linting pipeline on detected changes."

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::{IWatchAggregate, IWatchProviderProtocol, WatchConfig};

// Mock linter that tracks invocation count.
use shared::cli_commands::{LintResult, LintResultList};
use shared::code_analysis::{CodeAnalysisRuleVO, ICodeAnalysisAggregate};

use shared::common::{BooleanVO, Score};
use shared::common::{DisplayContent, FilePath};

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
