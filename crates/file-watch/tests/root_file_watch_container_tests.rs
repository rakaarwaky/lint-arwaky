use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use std::sync::Arc;

struct MockLinter;

impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _path: &str) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_dir(&self, _src_dir: &str) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_path(&self, _path: &str) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _results: &[LintResult]) -> f64 {
        0.0
    }
    fn check_critical(&self, _results: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &str) -> String {
        String::new()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

#[test]
fn container_can_be_constructed() {
    let container = FileWatchContainer::new();
    let _provider = container.provider();
}

#[test]
fn container_default_is_same_as_new() {
    let c1 = FileWatchContainer::new();
    let c2 = FileWatchContainer::default();
    let _ = c1.provider();
    let _ = c2.provider();
}

#[test]
fn container_provider_is_accessible() {
    let container = FileWatchContainer::new();
    let provider = container.provider();
    let available = provider.is_available();
    // is_available returns whether the 'watch' feature is enabled (sync)
    let _ = available;
}

#[test]
fn container_orchestrator_can_be_created() {
    let container = FileWatchContainer::new();
    let _orchestrator =
        container.orchestrator(Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>);
}

#[test]
fn container_orchestrator_runs_with_mock_linter() {
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
    use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;
    use std::sync::atomic::{AtomicBool, Ordering};

    let container = FileWatchContainer::new();
    let orchestrator =
        container.orchestrator(Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>);
    // Use an actual temp directory so the watcher can start
    let dir = std::env::temp_dir().join(format!("watch_container_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let config = WatchConfig {
        path: FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default(),
        ignore_patterns: vec![],
        recursive: false,
        debounce_ms: 100,
    };
    let running = Arc::new(AtomicBool::new(false));
    // With running=false, should stop after initial lint + watcher start
    let exit_code = orchestrator.run(config, running);
    assert_eq!(
        exit_code,
        std::process::ExitCode::SUCCESS,
        "orchestrator should exit gracefully"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
