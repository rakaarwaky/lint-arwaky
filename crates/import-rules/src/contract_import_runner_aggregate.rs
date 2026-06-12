// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
use async_trait::async_trait;
use output_report::taxonomy_result_vo::LintResult;
use shared::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}