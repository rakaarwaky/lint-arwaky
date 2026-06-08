use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::code_analysis::taxonomy_fix_vo::FixResult;

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
