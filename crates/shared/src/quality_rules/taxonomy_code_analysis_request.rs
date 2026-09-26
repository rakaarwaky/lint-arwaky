// PURPOSE: CodeAnalysisRequest — request payload for the code_analysis aggregate

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub enum CodeAnalysisRequest {
    /// Run quality checks on pre-parsed file entries from the filesystem crate.
    RunAnalysis { files: Vec<FileEntry> },
    /// Calculate compliance score from a list of lint results.
    CalcScore { results: Vec<LintResult> },
    /// Format a report from a list of lint results.
    FormatReport {
        results: Vec<LintResult>,
        project_root: FilePath,
    },
    /// Check if any result has CRITICAL severity.
    CheckCritical { results: Vec<LintResult> },
    /// Return the list of active code analysis rules from config.
    ActiveRules,
}

impl CodeAnalysisRequest {
    pub fn run_analysis(files: &[FileEntry]) -> Self {
        Self::RunAnalysis {
            files: files.to_vec(),
        }
    }
    pub fn calc_score(results: &[LintResult]) -> Self {
        Self::CalcScore {
            results: results.to_vec(),
        }
    }
    pub fn format_report(results: &[LintResult], project_root: &FilePath) -> Self {
        Self::FormatReport {
            results: results.to_vec(),
            project_root: project_root.clone(),
        }
    }
    pub fn check_critical(results: &[LintResult]) -> Self {
        Self::CheckCritical {
            results: results.to_vec(),
        }
    }
    pub fn active_rules() -> Self {
        Self::ActiveRules
    }
}
