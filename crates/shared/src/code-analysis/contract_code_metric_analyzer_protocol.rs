// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::taxonomy_path_vo::DirectoryPath;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(&self, path: Option<DirectoryPath>) -> Vec<AesCodeAnalysisViolation>;
}
