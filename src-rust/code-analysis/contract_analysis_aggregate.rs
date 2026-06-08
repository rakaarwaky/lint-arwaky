use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::layer_rules::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
