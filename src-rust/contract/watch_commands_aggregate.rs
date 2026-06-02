use crate::taxonomy::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait WatchCommandsAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn watch(&self, path: &FilePath);
}
