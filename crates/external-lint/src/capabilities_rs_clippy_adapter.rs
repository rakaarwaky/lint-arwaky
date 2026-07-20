// PURPOSE: RsClippyAdapter — ILinterAdapterProtocol implementation for Clippy linting integration
//
// Executes `cargo clippy --message-format=json` as a subprocess, then parses
// the JSON output line by line. Clippy outputs one JSON object per diagnostic
// message, each containing spans (source locations), severity levels, and
// lint codes.
//
// The adapter handles:
//   - Finding the correct Cargo.toml parent directory
//   - Parsing the JSON stream (filtering for compiler-message reasons)
//   - Resolving relative file paths to absolute across workspaces
//   - Converting Clippy severity levels to AES severity levels
//   - Falling back to stderr if stdout is empty (Clippy sometimes outputs to stderr)
//
// NOTE: apply_fix runs `cargo clippy --fix` which modifies files in place.
// This is the only adapter that supports auto-fix.
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::contract_executor_protocol::ICommandExecutorProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_path_normalization_protocol::IPathNormalizationProtocol;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_path_vo::FilePath;
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

use shared::external_lint::taxonomy_external_lint_helper::resolve_cargo_working_dir;

/// Adapter for Rust Clippy static analysis.
pub struct RustLinterAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    path_norm: Arc<dyn IPathNormalizationProtocol>,
    _bin_path: Option<FilePath>,
}

impl RustLinterAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        path_norm: Arc<dyn IPathNormalizationProtocol>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            _bin_path: bin_path,
        }
    }
}

#[async_trait]
impl ILinterAdapterProtocol for RustLinterAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("clippy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = resolve_cargo_working_dir(path);
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
                        let is_primary = span
                            .get("is_primary")
                            .and_then(|v| v.as_bool())
                            .unwrap_or_default();
                        if !is_primary {
                            continue;
                        }
                        let filename = match span.get("file_name").and_then(|f| f.as_str()) {
                            Some(f) if !f.is_empty() => f,
                            _ => continue,
                        };
                        let resolved_file = self.path_norm.resolve_capabilities_path(
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
        let working_dir = resolve_cargo_working_dir(path);
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
