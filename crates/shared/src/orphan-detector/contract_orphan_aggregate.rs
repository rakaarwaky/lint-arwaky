// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::common::taxonomy_path_vo::FilePath;
use crate::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

/// Aggregate that detects orphan (unreferenced) files in a project.
///
/// AES308 requires that every source file be reachable from at least one
/// entry point. This aggregate builds a dependency graph, identifies
/// orphan entry points, and reports violations.
pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &OrphanFileListVO) -> OrphanFileListVO;
    fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult>;
    /// Check orphans using a pre-built graph context (avoids rebuilding inbound_links per call)
    fn check_orphans_with_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult>;
}
