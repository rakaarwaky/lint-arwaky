// PURPOSE: ESLintAdapter — ILinterAdapterProtocol implementation for ESLint integration
//
// Executes `npx eslint --format=json` as a subprocess and parses the
// JSON output. ESLint outputs a JSON array of per-file results, each
// containing an array of messages with rule IDs, severity, and location.
//
// Key handling:
//   - Resolves the correct working directory (package.json parent)
//   - Uses npx to find eslint (works for both local and global installs)
//   - Mirrors Ruff adapter's path-fallback logic for consistency
//   - Returns empty results for non-JS/TS files (no error)
//   - Maps ESLint severity (1=warning, 2=error) to AES severity levels

use serde_json::Value;
use shared::cli_commands::contract_executor_protocol::ICommandExecutorProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::taxonomy_external_lint_helper::{
    canonicalize_path, exec_cmd_scan, js_apply_fix, resolve_js_cmd,
    resolve_js_working_dir as resolve_working_dir,
};

pub struct ESLintAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
}

impl ESLintAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
    ) -> Self {
        Self {
            executor,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterProtocol for ESLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("eslint")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = canonicalize_path(path_str);

        let cmd = resolve_js_cmd(
            "eslint",
            vec![abs_path, "--format".to_string(), "json".to_string()],
            &wd.value,
        );

        let response = exec_cmd_scan(
            self.executor.as_ref(),
            cmd,
            wd.clone(),
            60.0,
            Some(self.name()),
            path,
        )
        .await?;

        let stdout_str = response.stdout.to_string();
        if stdout_str.trim().is_empty() {
            return Ok(LintResultList::default());
        }

        let parsed: Value = serde_json::from_str(&stdout_str).map_err(|e| {
            LinterOperationError::Scan(ScanError {
                path: path.clone(),
                message: ErrorMessage::new(format!("Failed to parse JSON: {}", e)),
                error_code: None,
                adapter_name: Some(self.name()),
                cause: None,
            })
        })?;

        let mut results = Vec::new();
        if let Some(files) = parsed.as_array() {
            for file_data in files {
                let filename = match file_data["filePath"].as_str() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };
                let filename_vo = shared::common::utility_path_normalization::resolve_capabilities_path(
                    FilePath::new(filename).unwrap_or(path.clone()),
                    Some(path.clone()),
                );

                if let Some(messages) = file_data["messages"].as_array() {
                    for msg in messages {
                        let line_num = match msg["line"].as_u64() {
                            Some(v) => v as usize,
                            None => 1,
                        };
                        let col_num = match msg["column"].as_u64() {
                            Some(v) => v as usize,
                            None => 0,
                        };
                        let rule_id = match msg["ruleId"].as_str() {
                            Some(s) => s.to_string(),
                            None => "ESLINT".to_string(),
                        };
                        let message_text = match msg["message"].as_str() {
                            Some(s) => s.to_string(),
                            None => String::new(),
                        };
                        let sev_code = msg["severity"].as_u64().unwrap_or(1);

                        let severity = if sev_code == 2 {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };

                        results.push(LintResult {
                            file: filename_vo.clone(),
                            line: LineNumber::new(line_num as i64),
                            column: ColumnNumber::new(col_num as i64),
                            code: ErrorCode::raw(rule_id),
                            message: LintMessage::new(message_text),
                            source: Some(self.name()),
                            severity,
                            enclosing_scope: Default::default(),
                            related_locations: Default::default(),
                        });
                    }
                }
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        js_apply_fix(self.executor.as_ref(), path, "eslint", "--fix").await
    }
}
