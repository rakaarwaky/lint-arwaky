// PURPOSE: Integration tests — DI wiring via AutoFixContainer.
// Verifies the container correctly wires LintFixProcessor → FixOrchestrator.

use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

// ─── Mock ICodeAnalysisAggregate ──────────────────────────

struct StubLinter;

impl ICodeAnalysisAggregate for StubLinter {
    fn run_code_analysis(&self, _project_root: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_dir(&self, _src_dir: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_path(&self, _path: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _results: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _results: &[LintResult]) -> BooleanVO {
        false
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> DisplayContent {
        DisplayContent::new("")
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

fn container() -> AutoFixContainer {
    AutoFixContainer::new(Arc::new(StubLinter))
}

// ─── Container wiring ─────────────────────────────────────

#[test]
fn container_creates_orchestrator() {
    let c = container();
    let orch = c.orchestrator(false);
    // Should produce a valid Arc<dyn LintFixOrchestratorAggregate>
    let path = FilePath::new("src/main.rs".to_string()).unwrap();
    let result = orch.execute(&path);
    assert!(result.is_success());
}

#[test]
fn container_creates_dry_run_orchestrator() {
    let c = container();
    let orch = c.orchestrator(true);
    let path = FilePath::new("src/main.rs".to_string()).unwrap();
    let result = orch.execute(&path);
    assert!(result.is_success());
}

#[test]
fn container_is_cloneable() {
    let c = container();
    let c2 = c.clone();
    let orch = c2.orchestrator(false);
    let path = FilePath::new("test.rs".to_string()).unwrap();
    let result = orch.execute(&path);
    assert!(result.is_success());
}

#[test]
fn orchestrator_execute_returns_fix_result() {
    let c = container();
    let orch = c.orchestrator(false);
    let path = FilePath::new("src/lib.rs".to_string()).unwrap();
    let result = orch.execute(&path);
    // With no violations, should report "No automatic fixes applied"
    assert!(result.output.value().contains("No automatic fixes"));
}

// ─── Full pipeline: container → orchestrator → processor ──

#[test]
fn full_pipeline_no_violations_clean_result() {
    let c = container();
    let orch = c.orchestrator(false);
    let path = FilePath::new("clean_file.rs".to_string()).unwrap();
    let result = orch.execute(&path);
    assert!(result.error.is_none());
    assert!(result.output.value().contains("No automatic fixes"));
}
