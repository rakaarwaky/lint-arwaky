use crate::taxonomy::{FilePath, GovernanceReport};
use async_trait::async_trait;

#[async_trait]
pub trait ProjectContainerAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn run_analysis(&self, path: &FilePath) -> GovernanceReport;
    async fn get_health(&self) -> serde_json::Value;
}
