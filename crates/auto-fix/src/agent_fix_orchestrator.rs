// PURPOSE: FixOrchestrator — orchestrates auto-fix operations via IFixProtocol (agent layer)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
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
        self.fix_protocol
            .report_non_fixable(violations)
            .iter()
            .map(|m| m.to_string())
            .collect()
    }
}

impl LintFixOrchestratorAggregate for FixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }
}
