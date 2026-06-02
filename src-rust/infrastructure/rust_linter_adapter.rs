use std::path::{Path, PathBuf};

use crate::contract::{ICommandExecutorPort, ILinterAdapterPort, IPathNormalizationPort};
use crate::taxonomy::{
    AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, FilePath, LineNumber, LintMessage,
    LintResult, LintResultList, PatternList, ScanError, Severity, Timeout,
};
use crate::infrastructure::mcp_server_constants::MAX_FILES; // Not used but kept for parity
use async_trait::async_trait;
use serde_json::Value;
use tracing::{debug, error, info};

/// Adapter for Rust Clippy static analysis, rustfmt, and cargo audit.
pub struct RustLinterAdapter {
    executor: Box<dyn ICommandExecutorPort>,
    path_norm: Box<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
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
            bin_path,
        }
    }

    fn name(&self) -> AdapterName {
        AdapterName::new("clippy".to_string())
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
                || current.join("auto_linter.config.python.yaml").exists()
                || current.join("auto_linter.config.javascript.yaml").exists()
                || current.join("auto_linter.config.rust.yaml").exists()
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
    async fn scan(&self, path: FilePath) -> Result<LintResultList, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = LintResultList::new();
        let working_dir = self._resolve_working_dir(&path);
        let working_dir_str = &working_dir.value;

        // Check if cargo is available in working_dir
        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!("Skipping clippy scan: Cargo.toml not found at {:#?}", cargo_toml);
            return Ok(results);
        }

        // Run cargo clippy
        let cmd = vec!["cargo".to_string(), "clippy".to_string(), "--message-format=json".to_string()];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Timeout::new(180.0),
            )
            .await?;

        // Clippy writes to stdout or stderr depending on cargo invocation
        let output = String::from_utf8_lossy(&result.stdout);
        let output = if output.trim().is_empty() {
            String::from_utf8_lossy(&result.stderr)
        } else {
            output
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

                    let severity = if level == "error" {
                        Severity::HIGH
                    } else {
                        Severity::MEDIUM
                    };

                    let spans = msg.get("spans").and_then(|s| s.as_array()).unwrap_or(&vec![]);
                    for span in spans {
                        let span = span.as_object().unwrap_or(&std::collections::HashMap::new());
                        if !span.get("is_primary").and_then(|v| v.as_bool()).unwrap_or(false) {
                            continue;
                        }
                        let filename = span
                            .get("file_name")
                            .and_then(|f| f.as_str())
                            .unwrap_or("");
                        if filename.is_empty() {
                            continue;
                        }
                        let resolved_file = self
                            .path_norm
                            .resolve_infrastructure_path(
                                FilePath::new(filename.to_string()),
                                Some(path.clone()),
                            );
                        let line_num = span
                            .get("line_start")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(1) as i32;
                        let column_num = span
                            .get("column_start")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(1) as i32;
                        results.values.push(LintResult::new(
                            resolved_file,
                            LineNumber::new(line_num),
                            ColumnNumber::new(column_num),
                            ErrorCode::new(code),
                            LintMessage::new(message_text),
                            self.name(),
                            severity,
                        ));
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    async fn apply_fix(&self, path: FilePath) -> Result<ComplianceStatus, Box<dyn std::error::Error + Send + Sync>> {
        let working_dir = self._resolve_working_dir(&path);
        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--fix".to_string(),
            "--allow-dirty".to_string(),
            "--allow-staged".to_string(),
        ];
        self.executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Timeout::new(180.0),
            )
            .await?;
        Ok(ComplianceStatus::new(true))
    }
}
