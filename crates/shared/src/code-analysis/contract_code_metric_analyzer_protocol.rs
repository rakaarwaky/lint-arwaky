// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::contract_system_protocol::IFileSystemProtocol;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        fs: &dyn IFileSystemProtocol,
    ) -> Vec<AesCodeAnalysisViolation>;
}
