// PURPOSE: CodeAnalysisRequest/CodeAnalysisResponse — request/response VOs for the code analysis aggregate
use crate::common::taxonomy_common_vo::{BooleanVO, Score};
use crate::common::taxonomy_display_content_vo::DisplayContent;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use crate::quality_rules::taxonomy_code_analysis_vo::CodeAnalysisRuleVO;

/// Consumer verb carried by the code analysis aggregate's single entry point.
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

/// Result of a code analysis aggregate request.
pub enum CodeAnalysisResponse {
    Analysis { violations: Vec<LintResult> },
    Score { score: Score },
    Report { content: DisplayContent },
    Critical { is_critical: BooleanVO },
    Rules { rules: Vec<CodeAnalysisRuleVO> },
}

impl CodeAnalysisResponse {
    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::Analysis { violations } => violations,
            _ => Vec::new(),
        }
    }

    pub fn into_score(self) -> Score {
        match self {
            Self::Score { score } => score,
            _ => Score::default(),
        }
    }

    pub fn into_content(self) -> DisplayContent {
        match self {
            Self::Report { content } => content,
            _ => DisplayContent::default(),
        }
    }

    pub fn into_is_critical(self) -> BooleanVO {
        match self {
            Self::Critical { is_critical } => is_critical,
            _ => BooleanVO::default(),
        }
    }

    pub fn into_rules(self) -> Vec<CodeAnalysisRuleVO> {
        match self {
            Self::Rules { rules } => rules,
            _ => Vec::new(),
        }
    }
}
