//! Port trait for collecting and persisting metrics.
//!
//! Defines the outbound interface for gathering code metrics
//! (such as line counts) and storing historical metric data.

use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait::async_trait]
pub trait IMetricsProviderPort: Send + Sync {
    async fn get_line_count(&self, path: &FilePath) -> Count;
    async fn get_history(&self) -> Vec<serde_json::Value>;
    async fn save_metric(&self, entry: serde_json::Value) -> bool;
}
