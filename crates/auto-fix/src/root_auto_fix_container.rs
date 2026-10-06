// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)
//
// BF-1: `dry_run` is passed per-request via `execute(path, dry_run)`
// V3:  Wiring updated to compose 4 focused capabilities per FR.

use crate::agent_fix_orchestrator::FixOrchestrator;
use crate::agent_fix_orchestrator::FixOrchestratorDeps;
use crate::capabilities_bypass_fix::BypassFix;
use crate::capabilities_symbol_rename::SymbolRename;
use crate::capabilities_unused_import_fix::UnusedImportFix;
use crate::capabilities_violation_report::ViolationReport;
use shared_auto_fix::IFixAggregate;
use shared_auto_fix::{
    IBypassFixProtocol, ISymbolRenameProtocol, IUnusedImportFixProtocol, IViolationReportProtocol,
};
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
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

    /// Construct orchestrator with caller-provided IO protocol.
    pub fn orchestrator(&self, io: Arc<dyn IFileSystemIOProtocol>) -> Arc<dyn IFixAggregate> {
        let bypass_fix: Arc<dyn IBypassFixProtocol> = Arc::new(BypassFix::new(io.clone()));
        let unused_import_fix: Arc<dyn IUnusedImportFixProtocol> =
            Arc::new(UnusedImportFix::new(io.clone()));
        let symbol_rename: Arc<dyn ISymbolRenameProtocol> = Arc::new(SymbolRename::new(io.clone()));
        let violation_report: Arc<dyn IViolationReportProtocol> = Arc::new(ViolationReport::new(
            self.code_analysis_linter.clone(),
            unused_import_fix.clone(),
            bypass_fix.clone(),
            symbol_rename.clone(),
            io.clone(),
        ));
        Arc::new(FixOrchestrator::new(FixOrchestratorDeps {
            violation_report,
            bypass_fix,
            unused_import_fix,
            symbol_rename,
        }))
    }

    /// Construct orchestrator with filesystem aggregate — creates IO internally.
    pub fn orchestrator_with_filesystem(
        &self,
        _filesystem: Arc<
            dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
        >,
        io: Arc<dyn shared_filesystem::IFileSystemIOProtocol>,
    ) -> Arc<dyn IFixAggregate> {
        self.orchestrator(io)
    }
}
