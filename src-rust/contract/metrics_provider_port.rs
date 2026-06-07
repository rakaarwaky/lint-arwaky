use crate::taxonomy::{Count, FilePath};

#[async_trait::async_trait]
pub trait IMetricsProviderPort: Send + Sync {
    async fn get_line_count(&self, path: &FilePath) -> Count;
    async fn get_history(&self) -> Vec<serde_json::Value>;
    async fn save_metric(&self, entry: serde_json::Value) -> bool;
}
