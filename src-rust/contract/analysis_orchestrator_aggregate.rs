use async_trait::async_trait;
use crate::taxonomy::{FilePath, GovernanceReport};
use crate::contract::ServiceContainerAggregate;

#[async_trait]
pub trait AnalysisOrchestratorAggregate: Send + Sync {
    fn container(&self) -> &dyn ServiceContainerAggregate;
    async fn get_complexity(&self, path: &FilePath) -> GovernanceReport;
    async fn get_duplicates(&self, path: &FilePath) -> GovernanceReport;
    async fn get_trends(&self, path: &FilePath) -> GovernanceReport;
    async fn get_dependencies(&self, path: &FilePath) -> GovernanceReport;
    async fn run(&self, path: &FilePath) -> GovernanceReport;
}
