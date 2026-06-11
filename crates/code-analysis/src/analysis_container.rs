// PURPOSE: AnalysisContainer — wiring for code-analysis feature (root layer, wiring only)
use std::sync::Arc;
use crate::IArchLintProtocol;

pub struct AnalysisContainer {
    arch_linter: Arc<dyn IArchLintProtocol>,
}

impl AnalysisContainer {
    pub fn new() -> Self {
        Self {
            arch_linter: Arc::new(
                crate::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new(),
            ),
        }
    }

    pub fn architecture_linter(&self) -> Arc<dyn IArchLintProtocol> {
        self.arch_linter.clone()
    }
}
impl Default for AnalysisContainer {
    fn default() -> Self {
        Self::new()
    }
}
