// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)
//
// BF-1: `dry_run` is passed per-request via `execute(path, dry_run)`

use crate::agent_fix_orchestrator::FixOrchestrator;
use crate::agent_fix_orchestrator::FixOrchestratorDeps;
use crate::capabilities_file_adapter::FileAdapter;
use crate::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::IFixAggregate;
use shared::auto_fix::{
    IBypassFixProtocol, IFileAdapterProtocol, IFixPipelineProtocol, IManualReportProtocol,
    ISymbolRenameProtocol, IUnusedImportFixProtocol,
};
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
        file_adapter: Arc<dyn IFileAdapterProtocol>,
    ) -> Arc<dyn IFixAggregate> {
        // One concrete processor, shared behind each FR-backed seam.
        let processor = Arc::new(LintFixProcessor::new(
            self.code_analysis_linter.clone(),
            file_adapter.clone(),
        ));
        let pipeline: Arc<dyn IFixPipelineProtocol> = processor.clone();
        let manual_report: Arc<dyn IManualReportProtocol> = processor.clone();
        let bypass_fix: Arc<dyn IBypassFixProtocol> = processor.clone();
        let unused_import_fix: Arc<dyn IUnusedImportFixProtocol> = processor.clone();
        let symbol_rename: Arc<dyn ISymbolRenameProtocol> = processor.clone();
        Arc::new(FixOrchestrator::new(
            FixOrchestratorDeps {
                pipeline,
                manual_report,
                bypass_fix,
                unused_import_fix,
                symbol_rename,
            },
            file_adapter,
        ))
    }

    /// Construct orchestrator with filesystem aggregate — handles FileAdapter internally.
    pub fn orchestrator_with_filesystem(
        &self,
        filesystem: Arc<
            dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
        >,
        io: Arc<dyn shared::filesystem::IFileSystemIOProtocol>,
    ) -> Arc<dyn IFixAggregate> {
        let file_adapter: Arc<dyn IFileAdapterProtocol> =
            Arc::new(FileAdapter::new(filesystem, io));
        self.orchestrator(file_adapter)
    }
}
