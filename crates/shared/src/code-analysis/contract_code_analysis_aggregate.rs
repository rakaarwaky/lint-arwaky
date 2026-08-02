// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use crate::common::taxonomy_common_vo::{BooleanVO, Score};
use crate::common::taxonomy_display_content_vo::DisplayContent;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

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

    // ── Code Duplication Detection ───────────────────────────

    /// Collect file entries (path, content) for duplication analysis.
    fn collect_file_entries(&self, files: &[String]) -> Vec<(std::path::PathBuf, String)>;

    /// Scan for duplicate code blocks across files.
    fn scan_duplicate_blocks(
        &self,
        entries: Vec<(std::path::PathBuf, String)>,
        min_lines: usize,
    ) -> Vec<Vec<(std::path::PathBuf, usize)>> {
        crate::code_analysis::utility_code_duplication_detector::scan_duplicate_blocks(entries, min_lines)
    }

    /// Build violation results from detected duplicate blocks.
    fn build_violations(
        &self,
        blocks: &[Vec<(std::path::PathBuf, usize)>],
        total_loc: usize,
        min_dup_lines: usize,
    ) -> Vec<crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation> {
        crate::code_analysis::utility_code_duplication_detector::build_violations(blocks, total_loc, min_dup_lines)
    }
}
