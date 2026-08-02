// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use crate::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

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
