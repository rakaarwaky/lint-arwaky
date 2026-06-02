use crate::taxonomy::{FilePath, GovernanceReport};
use async_trait::async_trait;

#[async_trait]
pub trait LintPipelineOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn run(&self, path: &FilePath) -> GovernanceReport;
}
