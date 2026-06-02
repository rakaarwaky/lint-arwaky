/// python_metrics_adapter — Thin adapters for Python metrics (Radon, file sizes, trends).
use crate::contract::IMetricsProviderPort;
use crate::taxonomy::{Count, ErrorMessage, FilePath, MetricsError, ResponseData, ResponseDataList};
use crate::contract::IPathNormalizationPort;
use std::sync::Arc;

pub struct MetricsProvider {
    path_norm: Arc<dyn IPathNormalizationPort>,
    history_path: String,
}

impl MetricsProvider {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>, history_path: &str) -> Self {
        Self { path_norm, history_path: history_path.to_string() }
    }
}

#[async_trait::async_trait]
impl IMetricsProviderPort for MetricsProvider {
    async fn get_line_count(&self, path: &FilePath) -> Result<Count, MetricsError> {
        let p = &path.value;
        if !std::path::Path::new(p).is_file() {
            return Err(MetricsError::new(format!("File not found: {}", p)));
        }
        match std::fs::read_to_string(p) {
            Ok(content) => Ok(Count::new(content.lines().count() as i64)),
            Err(e) => Err(MetricsError::new(format!("Failed to read: {}", e)))
        }
    }

    async fn get_history(&self) -> Result<Vec<ResponseData>, MetricsError> {
        if !std::path::Path::new(&self.history_path).exists() {
            return Ok(Vec::new());
        }
        let mut history = Vec::new();
        if let Ok(content) = std::fs::read_to_string(&self.history_path) {
            for line in content.lines() {
                if !line.trim().is_empty() {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
                        history.push(ResponseData::new(val));
                    }
                }
            }
        }
        Ok(history)
    }
}
