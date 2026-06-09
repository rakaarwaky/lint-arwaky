use crate::shared_common::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
/// Protocol untuk analysis queries.
/// Bukan aggregate — hanya butuh 1 protocol (analysis), bukan kombinasi multiple protocols.
pub trait IAnalysisProtocol: Send + Sync {
    async fn get_complexity(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn get_duplicates(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn get_trends(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn get_dependencies(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
    async fn run(&self, path: &FilePath) -> ArchitectureGovernanceEntity;
}
