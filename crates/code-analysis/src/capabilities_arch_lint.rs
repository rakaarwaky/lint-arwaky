// PURPOSE: CodeAnalysisArchLint — IArchLintProtocol implementation delegating to CodeAnalysisOrchestrator
use crate::agent_code_analysis_orchestrator::CodeAnalysisOrchestrator;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_score_vo::compute_score;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use std::sync::Arc;

pub struct CodeAnalysisArchLint {
    orchestrator: Arc<CodeAnalysisOrchestrator>,
}

impl CodeAnalysisArchLint {
    pub fn new(orchestrator: Arc<CodeAnalysisOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

impl IArchLintProtocol for CodeAnalysisArchLint {
    fn run_self_lint(&self, project_root: &str) -> LintResultList {
        LintResultList::new(self.orchestrator.run_self_lint(project_root))
    }

    fn run_self_lint_dir(&self, src_dir: &str) -> LintResultList {
        LintResultList::new(self.orchestrator.run_scan(src_dir))
    }

    fn run_lint(&self, path: &str) -> Vec<LintResult> {
        self.orchestrator.run_self_lint(path)
    }

    fn calc_score(&self, results: &[LintResult]) -> f64 {
        compute_score(results)
    }

    fn check_critical(&self, results: &[LintResult]) -> bool {
        crate::agent_project_target_orchestrator::has_critical(results)
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        self.orchestrator
            .format_report(&results.values, project_root)
    }
}
