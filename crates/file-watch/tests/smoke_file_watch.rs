// PURPOSE: Smoke test — verify the file-watch crate boots and core types are functional.
// Must complete in < 5 seconds.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

// Minimal mock linter for smoke.
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;

struct SmokeLinter;
impl ICodeAnalysisAggregate for SmokeLinter {
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

#[test]
fn smoke_crate_boots_and_core_types_work() {
    // 1. Container wires without panic.
    let container = FileWatchContainer::new();

    // 2. Provider is accessible.
    let provider = container.provider();
    let _rx = provider.subscribe();

    // 3. Orchestrator is constructable.
    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(SmokeLinter);
    let orch = container.orchestrator(linter);

    // 4. ChangeAnalyzer basic operation.
    let _analyzer = ChangeAnalyzer::new();
    assert!(ChangeAnalyzer::is_lintable("main.rs"));
    assert!(!ChangeAnalyzer::is_lintable("photo.jpg"));

    // 5. Orchestrator with running=false exits immediately.
    let tmp_dir = tempfile::tempdir().unwrap();
    let config = WatchConfig::from_path(tmp_dir.path().to_string_lossy().to_string());
    let running = Arc::new(AtomicBool::new(false));
    let code = orch.run(config, running);
    assert_eq!(code, std::process::ExitCode::SUCCESS);
}
