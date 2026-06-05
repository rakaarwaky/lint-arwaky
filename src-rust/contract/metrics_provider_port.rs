use crate::taxonomy::{Count, FilePath};
use async_trait::async_trait;

#[async_trait::async_trait]
pub trait IMetricsProviderPort: Send + Sync {
    async fn get_line_count(&self, path: &FilePath) -> Count;
    async fn get_history(&self) -> Vec<serde_json::Value>;
}
