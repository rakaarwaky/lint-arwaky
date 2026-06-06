use crate::contract::ServiceContainerAggregate;
use crate::taxonomy::{ArchitectureGovernanceEntity, FilePath};
use async_trait::async_trait;

#[async_trait]
pub trait AnalysisOrchestratorAggregate: Send + Sync {
    fn container(&self) -> &dyn ServiceContainerAggregate;
    async fn get_complexity(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn get_duplicates(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn get_trends(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn get_dependencies(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn run(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
}
