// PURPOSE: RsClippyAdapter — ILinterAdapterPort implementation for Clippy linting integration
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
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

        let current = std::path::Path::new(path_str);
        if current.is_dir() {
            if current.join("Cargo.toml").exists() {
                return path.clone();
            }
        } else if let Some(parent) = current.parent() {
            if parent.join("Cargo.toml").exists() {
                return match FilePath::new(parent.to_string_lossy().replace('\\', "/")) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                };
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.toml").exists() {
                    return match FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    };
                }
            }
        }

        match FilePath::new("nonexistent_directory_for_cargo_toml".to_string()) {
            Ok(fp) => fp,
            Err(_) => FilePath::default(),
        }
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
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
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
                    let level = match msg.get("level").and_then(|l| l.as_str()) {
                        Some(l) => l.to_lowercase(),
                        None => "warning".to_string(),
                    };
                    let code = match msg
                        .get("code")
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                    {
                        Some(c) => c.to_string(),
                        None => "clippy::warning".to_string(),
                    };
                    let message_text = match msg.get("message").and_then(|m| m.as_str()) {
                        Some(m) => m.to_string(),
                        None => "Clippy finding".to_string(),
                    };
                    let spans: Vec<Value> = match msg.get("spans").and_then(|s| s.as_array()) {
                        Some(s) => s.clone(),
                        None => Vec::new(),
                    };

                    for span in &spans {
                        let is_primary = match span.get("is_primary").and_then(|v| v.as_bool()) {
                            Some(v) => v,
                            None => false,
                        };
                        if !is_primary {
                            continue;
                        }
                        let filename = match span.get("file_name").and_then(|f| f.as_str()) {
                            Some(f) if !f.is_empty() => f,
                            _ => continue,
                        };
                        let resolved_file = self.path_norm.resolve_infrastructure_path(
                            match FilePath::new(filename.to_string()) {
                                Ok(fp) => fp,
                                Err(_) => path.clone(),
                            },
                            Some(path.clone()),
                        );
                        let line_num = match span.get("line_start").and_then(|v| v.as_u64()) {
                            Some(v) => v as i64,
                            None => 1,
                        };
                        let column_num = match span.get("column_start").and_then(|v| v.as_u64()) {
                            Some(v) => v as i64,
                            None => 1,
                        };
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
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}
