// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use auto_fix::taxonomy_fix_vo::FixResult;
use source_parsing::taxonomy_path_vo::FilePath;

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
