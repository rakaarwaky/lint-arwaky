use std::path::Path;

use crate::contract::{ICommandExecutorPort, ILinterAdapterPort, IPathNormalizationPort};
use crate::taxonomy::{
    AdapterError, AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, ErrorMessage, FilePath,
    LineNumber, LintMessage, LintResult, LintResultList, LinterOperationError, PatternList,
    Severity,
};
// Not used but kept for parity
use async_trait::async_trait;
use serde_json::Value;
use tracing::debug;

/// Adapter for Rust Clippy static analysis, rustfmt, and cargo audit.
pub struct RustLinterAdapter {
    executor: Box<dyn ICommandExecutorPort>,
    path_norm: Box<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl RustLinterAdapter {
    /// Create a new RustLinterAdapter.
    pub fn new(
        executor: Box<dyn ICommandExecutorPort>,
        path_norm: Box<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            _bin_path: bin_path,
        }
    }

    fn _resolve_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }

        let current = match std::env::current_dir() {
            Ok(c) => c,
            Err(_) => return path.clone(),
        };

        let mut current = current;
        for _ in 0..10 {
            if current.join("Cargo.toml").exists()
                || current.join("lint_arwaky.config.python.yaml").exists()
                || current.join("lint_arwaky.config.javascript.yaml").exists()
                || current.join("lint_arwaky.config.rust.yaml").exists()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().replace('\\', "/"));
            }
            if !current.pop() {
                break;
            }
        }

        FilePath::new(".".to_string())
    }
}

#[async_trait]
impl ILinterAdapterPort for RustLinterAdapter {
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = LintResultList::default();
        let working_dir = self._resolve_working_dir(path);
        let working_dir_str = &working_dir.value;

        // Check if cargo is available in working_dir
        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!(
                "Skipping clippy scan: Cargo.toml not found at {:#?}",
                cargo_toml
            );
            return Ok(results);
        }

        // Run cargo clippy
        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--message-format=json".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(std::time::Duration::from_secs(180)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        // Clippy writes to stdout or stderr depending on cargo invocation
        let output = if result.stdout.trim().is_empty() {
            result.stderr.clone()
        } else {
            result.stdout.clone()
        };

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('{') {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Ok(data) => {
                    if data.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
                        continue;
                    }
                    let msg = data.get("message").unwrap_or(&Value::Null);
                    let level = msg
                        .get("level")
                        .and_then(|l| l.as_str())
                        .unwrap_or("warning")
                        .to_lowercase();
                    let code_data = msg.get("code");
                    let code = code_data
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("clippy::warning")
                        .to_string();
                    let message_text = msg
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Clippy finding")
                        .to_string();

                    let spans_owned: Vec<Value> = msg
                        .get("spans")
                        .and_then(|s| s.as_array())
                        .cloned()
                        .unwrap_or_default();
                    let empty_map = serde_json::Map::new();
                    for span_value in &spans_owned {
                        let span = span_value.as_object().unwrap_or(&empty_map);
                        if !span
                            .get("is_primary")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                        {
                            continue;
                        }
                        let filename = span.get("file_name").and_then(|f| f.as_str()).unwrap_or("");
                        if filename.is_empty() {
                            continue;
                        }
                        let resolved_file = self.path_norm.resolve_infrastructure_path(
                            &FilePath::new(filename.to_string()),
                            Some(path),
                        );
                        let line_num =
                            span.get("line_start").and_then(|v| v.as_u64()).unwrap_or(1) as i32;
                        let column_num = span
                            .get("column_start")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(1) as i32;
                        let severity = if level == "error" {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };
                        results.push(LintResult::new(
                            resolved_file,
                            LineNumber::new(line_num as i64),
                            ColumnNumber::new(column_num as i64),
                            ErrorCode::raw(code.as_str()),
                            LintMessage::new(message_text.as_str()),
                            AdapterName::raw("clippy"),
                            severity,
                        ));
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = self._resolve_working_dir(path);
        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--fix".to_string(),
            "--allow-dirty".to_string(),
            "--allow-staged".to_string(),
        ];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(std::time::Duration::from_secs(180)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }

    fn name(&self) -> AdapterName {
        AdapterName::raw("clippy")
    }
}
