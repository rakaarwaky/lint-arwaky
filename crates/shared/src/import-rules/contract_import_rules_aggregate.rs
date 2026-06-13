// PURPOSE: IImportRulesAggregate — contract aggregate for import-rules feature orchestrator
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IImportRulesAggregate: Send + Sync {
    /// Run all import rule audits on the given target path.
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;

    /// Identifier for this aggregate.
    fn name(&self) -> &str;
}
