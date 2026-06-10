// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::shared_common::taxonomy_fix_vo::FixResult;

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
