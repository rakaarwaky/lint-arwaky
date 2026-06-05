/// python_metrics_adapter — Thin adapters for Python metrics (Radon, file sizes, trends).
use crate::contract::metrics_provider_port::IMetricsProviderPort;
use crate::taxonomy::{Count, FilePath};
use crate::contract::path_normalization_port::IPathNormalizationPort;
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
    async fn get_line_count(&self, path: &FilePath) -> Count {
        let p = &path.value;
        if !std::path::Path::new(p).is_file() {
            return Count::new(0);
        }
        match std::fs::read_to_string(p) {
            Ok(content) => Count::new(content.lines().count() as i64),
            Err(_) => Count::new(0),
        }
    }

    async fn get_history(&self) -> Vec<serde_json::Value> {
        if !std::path::Path::new(&self.history_path).exists() {
            return Vec::new();
        }
        let mut history = Vec::new();
        if let Ok(content) = std::fs::read_to_string(&self.history_path) {
            for line in content.lines() {
                if !line.trim().is_empty() {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
                        history.push(val);
                    }
                }
            }
        }
        history
    }
}
