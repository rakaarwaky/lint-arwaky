// PURPOSE: FixOrchestrator — orchestrates auto-fix operations via IFixProtocol (agent layer)
use crate::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::auto_fix::contract_fix_protocol::IFixProtocol;
use crate::shared_common::taxonomy_fix_vo::FixResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct FixOrchestrator {
    fix_protocol: Arc<dyn IFixProtocol>,
}

impl FixOrchestrator {
    pub fn new(fix_protocol: Arc<dyn IFixProtocol>) -> Self {
        Self { fix_protocol }
    }
}

impl LintFixOrchestratorAggregate for FixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }
}
