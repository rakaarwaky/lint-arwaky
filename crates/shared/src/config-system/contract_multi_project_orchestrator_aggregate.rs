use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use async_trait::async_trait;

#[async_trait]
pub trait MultiProjectOrchestratorAggregate: Send + Sync {
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
