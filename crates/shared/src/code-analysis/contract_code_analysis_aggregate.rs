// PURPOSE: ICodeAnalysisAggregate — contract aggregate for code-analysis feature orchestrator
use crate::output_report::taxonomy_result_vo::LintResult;
use async_trait::async_trait;

#[async_trait]
pub trait ICodeAnalysisAggregate: Send + Sync {
    /// Run all AES analysis checks on the given files.
    async fn run_analysis(
        &self,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;

    /// Identifier for this aggregate.
    fn name(&self) -> &str;
}
