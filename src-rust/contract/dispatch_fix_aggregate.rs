use crate::taxonomy::FilePath;
use async_trait::async_trait;


#[async_trait]
pub trait FixCommandsAggregate: Send + Sync {
    async fn fix(&self, project_path: &FilePath);
}
