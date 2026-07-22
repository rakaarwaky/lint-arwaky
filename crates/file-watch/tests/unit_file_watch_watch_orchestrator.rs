// PURPOSE: Unit tests for WatchOrchestrator — construction, aggregate trait surface.
// Layer: Agent (target ≥ 60% coverage)
//
// Note: Full run_async / run tests require a real ICodeAnalysisAggregate mock.
// These tests verify construction, wiring, and trait-level contracts.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;

// ─── Mock ICodeAnalysisAggregate ────────────────────────────
// Minimal stub so WatchOrchestrator can be constructed.

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::Score;
use shared::common::taxonomy_path_vo::FilePath;

struct MockLinter;

impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _project_root: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_dir(&self, _src_dir: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_path(&self, _path: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _results: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _results: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> String {
        String::from("mock report")
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

// ─── Helpers ────────────────────────────────────────────────

fn make_orchestrator() -> WatchOrchestrator {
    let provider: Arc<dyn IWatchProviderProtocol> = Arc::new(NotifyWatchProvider::new());
    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(MockLinter);
    WatchOrchestrator::new(provider, linter)
}

// ─── Construction ───────────────────────────────────────────

#[test]
fn new_creates_orchestrator() {
    let orch = make_orchestrator();
    let _ = &orch;
}

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WatchOrchestrator>();
}

// ─── IWatchAggregate trait surface ──────────────────────────

#[test]
fn orchestrator_implements_iwatch_aggregate() {
    fn assert_trait<T: IWatchAggregate>() {}
    assert_trait::<WatchOrchestrator>();
}

#[test]
fn orchestrator_can_be_boxed_as_dyn_aggregate() {
    let orch = make_orchestrator();
    let _boxed: Box<dyn IWatchAggregate> = Box::new(orch);
}

#[test]
fn orchestrator_can_be_arced_as_dyn_aggregate() {
    let orch = make_orchestrator();
    let _arced: Arc<dyn IWatchAggregate> = Arc::new(orch);
}

// ─── run with immediate shutdown flag ───────────────────────

#[test]
fn run_with_running_false_exits_immediately() {
    let orch = make_orchestrator();
    let config = shared::file_watch::taxonomy_watch_config_vo::WatchConfig::from_path(
        std::env::temp_dir().to_string_lossy().to_string(),
    );
    let running = Arc::new(AtomicBool::new(false));
    let code = orch.run(config, running);
    // With running=false the loop body never executes; exits SUCCESS.
    assert_eq!(code, std::process::ExitCode::SUCCESS);
}
