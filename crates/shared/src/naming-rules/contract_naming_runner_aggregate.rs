// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
