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

use serde_json::Value;
use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_common::{ErrorMessage, ScanError};
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::utility_extension_guard::is_scannable_file;
use shared_external_lint::utility_path_normalization::resolve_or_fallback_with_context;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ESLintAdapter {
    lint_executor: Arc<dyn ICommandExecutorProtocol>,
    js_resolution: Arc<dyn IJsToolResolutionProtocol>,
    io: Arc<dyn IFileSystemIOProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for ESLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("eslint")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = path.value();
        if self.io.is_file(Path::new(path_str))
            && !is_scannable_file(path_str, &[".ts", ".tsx", ".js", ".jsx"])
        {
            return Ok(LintResultList::default());
        }

        let wd = self.tool_resolution.resolve_js_working_dir(path);
        let abs_path = self.io.canonicalize_path_str(path);

        let eslint_name = ToolName::new("eslint");
        let cmd = match self.tool_resolution.resolve_js_cmd(
            &eslint_name,
            vec![abs_path.value, "--format".to_string(), "json".to_string()],
            &wd,
        ) {
            Some(c) => c,
            None => return Ok(LintResultList::default()),
        };

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
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
                error_id: shared_common::ErrorId::raw(2),
            })
        })?;

        let mut results = Vec::new();
        if let Some(files) = parsed.as_array() {
            for file_data in files {
                let filename = file_data["filePath"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                let filename_vo =
                    resolve_or_fallback_with_context(&filename, path.clone(), Some(path.clone()));

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
    violation_name: String::new(),
    why: String::new(),
    fix: String::new(),
                        });
                    }
                }
            }
        }

        Ok(LintResultList::new(results))
    }

    fn fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.js_resolution
            .js_apply_fix(path, &ToolName::new("eslint"), "--fix")
            .map_err(crate::convert_executor_error)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ESLintAdapter {
    pub fn new(
        lint_executor: Arc<dyn ICommandExecutorProtocol>,
        js_resolution: Arc<dyn IJsToolResolutionProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            lint_executor,
            js_resolution,
            io,
            tool_resolution,
        }
    }
}
