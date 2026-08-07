use crate::common::taxonomy_common_vo::{BooleanVO, Score};
use crate::common::taxonomy_display_content_vo::DisplayContent;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use crate::quality_rules::taxonomy_code_analysis_vo::CodeAnalysisRuleVO;

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn run_code_analysis(&self, project_root: &FilePath) -> LintResultList;
    fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList;
    fn run_code_analysis_path(&self, path: &FilePath) -> Vec<LintResult>;
    /// Run analysis on pre-parsed file entries from the filesystem crate.
    fn run_analysis_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult>;
    fn calc_score(&self, results: &[LintResult]) -> Score;
    fn format_report(&self, results: &LintResultList, project_root: &FilePath) -> DisplayContent;
    fn check_critical(&self, results: &[LintResult]) -> BooleanVO;
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO>;
}
