use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_path_vo::FilePath;
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
    let container = AutoFixContainer::new(
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    let _ = container;
}

#[test]
fn container_orchestrator_creation_live_mode() {
    let container = AutoFixContainer::new(
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    let orch = container.orchestrator(false);
    let path = FilePath::new("/nonexistent/test.rs".to_string()).unwrap_or_default();
    let result = orch.execute(&path);
    // With mock linter, no violations → fix result with no fixes
    assert!(result.error.is_none());
}

#[test]
fn container_orchestrator_creation_dry_run() {
    let container = AutoFixContainer::new(
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    let orch = container.orchestrator(true);
    let path = FilePath::new("/nonexistent/test.rs".to_string()).unwrap_or_default();
    let result = orch.execute(&path);
    assert!(result.error.is_none());
    assert!(result.output.to_string().contains("Dry-run"));
}

#[test]
fn container_orchestrator_is_cloneable() {
    let container = AutoFixContainer::new(
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    let orch1 = container.orchestrator(false);
    let orch2 = container.orchestrator(true);
    // Both should work independently
    let path = FilePath::new("test.rs".to_string()).unwrap_or_default();
    let r1 = orch1.execute(&path);
    let r2 = orch2.execute(&path);
    assert!(r1.error.is_none());
    assert!(r2.error.is_none());
}
