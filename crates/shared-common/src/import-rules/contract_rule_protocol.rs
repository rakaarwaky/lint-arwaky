// PURPOSE: IAnalyzer — port trait for analyzing import compliance
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IAnalyzer: Send + Sync {
    fn analyze_imports(&self, file_path: &FilePath, violations: &mut Vec<LintResult>);
}
