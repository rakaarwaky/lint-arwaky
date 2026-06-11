// PURPOSE: RsClippyAdapter — ILinterAdapterPort implementation for Clippy linting integration
use std::path::Path;
use std::sync::Arc;

use crate::cli_transport::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::language_adapters::taxonomy_adapter_error::AdapterError;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use serde_json::Value;
use tracing::debug;

/// Adapter for Rust Clippy static analysis.
pub struct RustLinterAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl RustLinterAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
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
                || current.join("lint_arwaky.config.yaml").exists()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default());
            }
            if !current.pop() {
                break;
            }
        }

        FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
    }
}

#[async_trait]
impl ILinterAdapterPort for RustLinterAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("clippy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self._resolve_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!(
                "Skipping clippy scan: Cargo.toml not found at {:?}",
                cargo_toml
            );
            return Ok(LintResultList::new(results));
        }

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
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    180.0,
                )),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

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
                    let msg = match data.get("message") {
                        Some(m) => m,
                        None => continue,
                    };
                    let level = msg
                        .get("level")
                        .and_then(|l| l.as_str())
                        .unwrap_or("warning")
                        .to_lowercase();
                    let code = msg
                        .get("code")
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("clippy::warning")
                        .to_string();
                    let message_text = msg
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Clippy finding")
                        .to_string();
                    let spans: Vec<Value> = msg
                        .get("spans")
                        .and_then(|s| s.as_array())
                        .cloned()
                        .unwrap_or_default();

                    for span in &spans {
                        let is_primary = span
                            .get("is_primary")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        if !is_primary {
                            continue;
                        }
                        let filename = match span.get("file_name").and_then(|f| f.as_str()) {
                            Some(f) if !f.is_empty() => f,
                            _ => continue,
                        };
                        let resolved_file = self.path_norm.resolve_infrastructure_path(
                            FilePath::new(filename.to_string()).unwrap_or_else(|_| path.clone()),
                            Some(path.clone()),
                        );
                        let line_num =
                            span.get("line_start").and_then(|v| v.as_u64()).unwrap_or(1) as i64;
                        let column_num = span
                            .get("column_start")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(1) as i64;
                        let severity = if level == "error" {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };
                        results.push(LintResult {
                            file: resolved_file,
                            line: LineNumber::new(line_num),
                            column: ColumnNumber::new(column_num),
                            code: ErrorCode::raw(code.as_str()),
                            message: LintMessage::new(message_text.as_str()),
                            source: Some(AdapterName::raw("clippy")),
                            severity,
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(LintResultList::new(results))
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
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    180.0,
                )),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}
