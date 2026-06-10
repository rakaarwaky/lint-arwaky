// PURPOSE: Aggregate: Fix aggregation/wiring
use crate::shared_common::taxonomy_fix_vo::FixResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
