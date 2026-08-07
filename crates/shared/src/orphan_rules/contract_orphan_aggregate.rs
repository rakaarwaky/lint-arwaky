// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use crate::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO;
use crate::quality_rules::taxonomy_analysis_vo::GraphAnalysisContext;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &OrphanFileListVO) -> OrphanFileListVO;
    fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult>;
    fn check_orphans_with_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult>;
    /// Run orphan detection on pre-parsed FileEntry from the filesystem crate.
    fn check_orphans_with_entries(
        &self,
        files: &[FileEntry],
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult>;
    fn scan_orphans(
        &self,
        root_dir: &FilePath,
        ignored: &[String],
    ) -> (GraphAnalysisContext, Vec<LintResult>);
}
