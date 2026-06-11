// PURPOSE: FixOrchestrator — orchestrates auto-fix operations via IFixProtocol (agent layer)
use crate::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::auto_fix::contract_fix_protocol::IFixProtocol;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct FixOrchestrator {
    fix_protocol: Arc<dyn IFixProtocol>,
}

impl FixOrchestrator {
    pub fn new(fix_protocol: Arc<dyn IFixProtocol>) -> Self {
        Self { fix_protocol }
    }

    /// Orchestrate: execute fix + report non-fixable
    pub fn run_fix(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }

    /// Orchestrate: get report of violations that need manual fix
    pub fn manual_report(&self, violations: &[LintResult]) -> Vec<String> {
        self.fix_protocol.report_non_fixable(violations)
    }
}

impl LintFixOrchestratorAggregate for FixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }
}
