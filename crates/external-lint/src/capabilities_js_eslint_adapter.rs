// PURPOSE: ESLintAdapter — ILinterAdapterProtocol implementation for ESLint integration
//
// Executes `npx eslint --format=json` as a subprocess and parses the
// JSON output. ESLint outputs a JSON array of per-file results, each
// containing an array of messages with rule IDs, severity, and location.
//
// Key handling:
//   - Resolves the correct working directory (package.json parent)
//   - Uses npx to find eslint (works for both local and global installs)
//   - Returns empty results for non-JS/TS files (no error)
//   - Maps ESLint severity (1=warning, 2=error) to AES severity levels

use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_path_normalization::resolve_capabilities_path;
use shared::common::{ErrorMessage, ScanError};
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use shared::quality_rules::LinterOperationError;
use shared::quality_rules::contract_adapter_protocol::ILinterAdapterProtocol;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ESLintAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for ESLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("eslint")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = path.value();
        if self.filesystem.is_file(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = self.filesystem.resolve_js_working_dir(path);
        let abs_path = self.filesystem.canonicalize_path_str(path);

        let eslint_name = ToolName::new("eslint").unwrap();
        let cmd = match self.filesystem.resolve_js_cmd(
            &eslint_name,
            vec![abs_path, "--format".to_string(), "json".to_string()],
            &wd,
        ) {
            Some(c) => c,
            None => return Ok(LintResultList::default()),
        };

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .await
            .map_err(crate::convert_executor_error)?;

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
                let filename = file_data["filePath"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                let filename_vo = resolve_capabilities_path(
                    FilePath::new(filename).unwrap_or_else(|_| path.clone()),
                    Some(path.clone()),
                );

                if let Some(messages) = file_data["messages"].as_array() {
                    for msg in messages {
                        let line_num = msg["line"].as_u64().unwrap_or(1) as usize;
                        let col_num = msg["column"].as_u64().unwrap_or(0) as usize;
                        let rule_id = msg["ruleId"].as_str().unwrap_or("ESLINT").to_string();
                        let message_text = msg["message"].as_str().unwrap_or("").to_string();
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
                            related_locations: LocationList::new(),
                        });
                    }
                }
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.lint_executor
            .js_apply_fix(path, "eslint", "--fix")
            .await
            .map_err(crate::convert_executor_error)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ESLintAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            lint_executor,
            filesystem,
        }
    }
}
