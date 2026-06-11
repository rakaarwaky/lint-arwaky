// PURPOSE: ArchSelfLintChecker — capabilities implementation of IArchLintProtocol for AES architecture compliance self-linting
use crate::IArchLintProtocol;
use shared::taxonomy_result_vo::LintResultList;

pub struct ArchSelfLintChecker {}

impl Default for ArchSelfLintChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchSelfLintChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IArchLintProtocol for ArchSelfLintChecker {
    fn run_self_lint(&self, project_root: &str) -> LintResultList {
        let orchestrator =
            crate::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new();
        let results = orchestrator.run_self_lint(project_root);
        LintResultList::new(results)
    }

    fn run_self_lint_dir(&self, src_dir: &str) -> LintResultList {
        let orchestrator =
            crate::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new();
        let results = orchestrator.run_self_lint_dir(src_dir);
        LintResultList::new(results)
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        let orchestrator =
            crate::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new();
        orchestrator.format_report(&results.values, project_root)
    }
}
