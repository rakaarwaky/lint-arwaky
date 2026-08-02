// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)

use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use std::sync::Arc;

#[derive(Clone)]
pub struct AutoFixContainer {
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
}

impl AutoFixContainer {
    pub fn new(code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            code_analysis_linter,
        }
    }

    pub fn orchestrator(
        &self,
        dry_run: bool,
        file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol>,
    ) -> Arc<dyn LintFixOrchestratorAggregate> {
        let fix_protocol =
            crate::capabilities_fix_processor::LintFixProcessor::with_dry_run(
                dry_run,
                self.code_analysis_linter.clone(),
                file_adapter,
            );
        Arc::new(crate::agent_fix_orchestrator::FixOrchestrator::new(
            Arc::new(fix_protocol),
        ))
    }
}
