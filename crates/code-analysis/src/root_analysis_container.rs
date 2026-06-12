// PURPOSE: AnalysisContainer — wiring for code-analysis feature (root layer, wiring only)
use std::sync::Arc;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use crate::CodebaseScanOrchestrator;

pub struct AnalysisContainer {
    arch_linter: Arc<dyn IArchLintProtocol>,
}

impl AnalysisContainer {
    pub fn new() -> Self {
        Self {
            arch_linter: Arc::new(
                CodebaseScanOrchestrator::new(),
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

