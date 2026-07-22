// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::taxonomy_path_vo::DirectoryPath;

/// Protocol for analysing source-code metrics such as duplication.
///
/// The single method scans a directory for duplicated blocks and returns
/// the resulting violations so they can be reported in the final lint output.
pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(&self, path: Option<DirectoryPath>) -> Vec<AesCodeAnalysisViolation>;
}
