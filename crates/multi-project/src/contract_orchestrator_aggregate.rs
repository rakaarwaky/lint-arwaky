// PURPOSE: MultiProjectOrchestratorAggregate — contract trait for multi-project orchestration
use async_trait::async_trait;
use shared::taxonomy_summary_vo::{AggregatedResults, ProjectResult};
use shared::Count;
use shared::FilePath;
use shared::FilePathList;
use shared::Identity;

#[async_trait]
pub trait MultiProjectOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn analyze_project(&self, path: &FilePath) -> ProjectResult;
    async fn scan_all_projects(
        &self,
        paths: &FilePathList,
        max_concurrency: Count,
    ) -> AggregatedResults;
    fn load_config(&self, config_path: Option<&FilePath>) -> FilePathList;
    fn find_projects(&self, root: &FilePath, config_name: &Identity) -> FilePathList;
}
