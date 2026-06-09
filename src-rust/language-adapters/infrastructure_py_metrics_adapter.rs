/// python_metrics_adapter — Thin adapters for Python metrics (Radon, file sizes, trends).
use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;
use crate::metrics_service::taxonomy_metrics_error::MetricsError;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct MetricsProvider {
    path_norm: Arc<dyn IPathNormalizationPort>,
    history_path: String,
}

impl MetricsProvider {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>, history_path: &str) -> Self {
        Self {
            path_norm,
            history_path: history_path.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl IMetricsProviderPort for MetricsProvider {
    async fn get_line_count(&self, path: &FilePath) -> Count {
        let normalized = self.path_norm.normalize_path(path.clone());
        let p = &normalized.value;
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

    async fn save_metric(&self, entry: serde_json::Value) -> bool {
        // Ensure the directory exists
        if let Some(parent) = std::path::Path::new(&self.history_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        // Append to history file (JSON-lines format)
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.history_path)
        {
            Ok(mut file) => {
                use std::io::Write;
                let line = match serde_json::to_string(&entry) {
                    Ok(s) => s,
                    Err(e) => {
                        let _err = MetricsError::new(ErrorMessage::new(format!(
                            "Failed to serialize metric entry: {}",
                            e
                        )));
                        return false;
                    }
                };
                writeln!(file, "{}", line).is_ok()
            }
            Err(_) => false,
        }
    }
}
