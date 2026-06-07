use crate::taxonomy::{ConfigResult, FilePath};
use async_trait::async_trait;

#[async_trait]
pub trait IConfigOrchestrationProtocol: Send + Sync {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(&self, project_root: &FilePath, language: &str) -> ConfigResult;
}
