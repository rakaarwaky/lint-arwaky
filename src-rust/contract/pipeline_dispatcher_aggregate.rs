use crate::taxonomy::source_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait PipelineActionDispatcherAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn dispatch(&self, action: &str, args: serde_json::Value) -> serde_json::Value;
    fn validate_action(&self, action: &str) -> bool;
}
