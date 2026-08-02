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

    /// Construct orchestrator with caller-provided file adapter.
    pub fn orchestrator(
        &self,
        dry_run: bool,
        file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol>,
    ) -> Arc<dyn LintFixOrchestratorAggregate> {
        let fix_protocol = crate::capabilities_fix_processor::LintFixProcessor::with_dry_run(
            dry_run,
            self.code_analysis_linter.clone(),
            file_adapter.clone(),
        );
        Arc::new(crate::agent_fix_orchestrator::FixOrchestrator::new(
            Arc::new(fix_protocol),
            file_adapter,
        ))
    }

    /// Construct orchestrator with filesystem aggregate — handles FileAdapter internally.
    pub fn orchestrator_with_filesystem(
        &self,
        dry_run: bool,
        filesystem: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate>,
    ) -> Arc<dyn LintFixOrchestratorAggregate> {
        let file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol> =
            Arc::new(crate::capabilities_file_adapter::FileAdapter::new(filesystem));
        self.orchestrator(dry_run, file_adapter)
    }
}
