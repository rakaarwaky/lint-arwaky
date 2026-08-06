// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::quality_rules::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;

use std::path::PathBuf;

/// Protocol for analysing source-code metrics such as duplication.
///
/// Accepts pre-fetched (path, content) entries from the caller and
/// returns the resulting (file_path, violation) tuples.
pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    /// Run duplication analysis on pre-fetched (path, content) entries.
    /// The caller is responsible for discovering and reading files.
    fn handle_duplicates_entries(
        &self,
        entries: &[(PathBuf, String)],
    ) -> Vec<(String, AesCodeAnalysisViolation)>;
}
