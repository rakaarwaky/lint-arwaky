use async_trait::async_trait;
use super::*;

#[async_trait]
pub trait CheckCommandsAggregate: Send + Sync {
    async fn check(&self, path: FilePath, git_diff: ComplianceStatus);
    async fn scan(&self, path: FilePath);
}
