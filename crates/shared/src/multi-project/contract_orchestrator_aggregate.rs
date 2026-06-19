// PURPOSE: MultiProjectOrchestratorAggregate — contract trait for multi-project orchestration
use crate::multi_project::taxonomy_workspace_info_vo::WorkspaceInfo;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait MultiProjectOrchestratorAggregate: Send + Sync {
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
