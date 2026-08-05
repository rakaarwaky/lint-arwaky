// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)
//
// Changes from previous version:
// - BF-1: `dry_run` is no longer baked into the container — passed per-request via `execute(path, dry_run)`
// - `with_dry_run` is deprecated in favor of `new()` + per-request dry_run

use crate::agent_fix_orchestrator::FixOrchestrator;
use crate::capabilities_file_adapter::FileAdapter;
use crate::capabilities_fix_processor::LintFixProcessor;
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
    /// `dry_run` is now passed per-request via `execute(path, dry_run)`.
    pub fn orchestrator(
        &self,
        file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol>,
    ) -> Arc<dyn LintFixOrchestratorAggregate> {
        let fix_protocol =
            LintFixProcessor::new(self.code_analysis_linter.clone(), file_adapter.clone());
        Arc::new(FixOrchestrator::new(Arc::new(fix_protocol), file_adapter))
    }

    /// Construct orchestrator with filesystem aggregate — handles FileAdapter internally.
    pub fn orchestrator_with_filesystem(
        &self,
        filesystem: Arc<
            dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
        >,
    ) -> Arc<dyn LintFixOrchestratorAggregate> {
        let file_adapter: Arc<dyn shared::auto_fix::IFileAdapterProtocol> =
            Arc::new(FileAdapter::new(filesystem));
        self.orchestrator(file_adapter)
    }
}
