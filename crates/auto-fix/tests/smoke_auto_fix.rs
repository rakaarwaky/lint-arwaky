// PURPOSE: Smoke test — verifies the auto-fix crate boots and core wiring works.
// Must complete in under 5 seconds.

use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::Score;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

struct NoopLinter;
impl ICodeAnalysisAggregate for NoopLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _: &LintResultList, _: &FilePath) -> String {
        String::new()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

#[test]
fn crate_boots_and_all_components_wire() {
    // 1. FileAdapter instantiates
    let adapter = FileAdapter::new();
    let test_path = FilePath::new("/tmp/smoke_test.rs".to_string()).unwrap();
    let _ = adapter.path_exists(&test_path);

    // 2. Container wires orchestrator
    let container = AutoFixContainer::new(Arc::new(NoopLinter));
    let orch = container.orchestrator(false);

    // 3. Orchestrator executes without panic
    let path = FilePath::new("src/main.rs".to_string()).unwrap();
    let result = orch.execute(&path);
    assert!(result.is_success());
}

#[test]
fn all_public_types_are_constructible() {
    let _adapter = FileAdapter::new();
    let _adapter_default = FileAdapter;

    let linter: Arc<dyn ICodeAnalysisAggregate> = Arc::new(NoopLinter);
    let _processor = LintFixProcessor::new(linter.clone());
    let _processor_dry = LintFixProcessor::with_dry_run(true, linter.clone());

    let protocol: Arc<dyn IFixProtocol> = Arc::new(LintFixProcessor::new(linter));
    let _orchestrator = FixOrchestrator::new(protocol);

    let _container = AutoFixContainer::new(Arc::new(NoopLinter));
}
