// PURPOSE: PyMypyAdapter — ILinterAdapterPort implementation for MyPy type checker integration
//
// Runs `mypy <path>` on Python files and parses its structured output with
// two regex patterns (with/without column numbers). Severity is mapped
// heuristically: notes → LOW, warnings → MEDIUM, errors → HIGH,
// syntax/parse errors → CRITICAL.
//
// Key details:
//   - `--no-error-summary` avoids summary lines, keeping output parseable
//   - `--pretty false` ensures machine-parseable single-line output
//   - Falls back to column-less regex if column-full regex doesn't match
//   - apply_fix always returns false (mypy is a type checker, not a formatter)

use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

use shared::external_lint::infrastructure_external_lint_helper::{
    default_working_dir, exec_cmd_adapter, noop_apply_fix,
};

pub struct MyPyAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl MyPyAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "mypy".to_string(),
        }
    }

    fn map_severity(msg_type: &str, msg: &str) -> Severity {
        let m = msg.to_lowercase();
        if msg_type == "note" {
            return Severity::LOW;
        }
        if m.contains("syntax") || m.contains("parse") {
            return Severity::CRITICAL;
        }
        if msg_type == "warning" {
            return Severity::MEDIUM;
        }
        Severity::HIGH
    }
}

#[async_trait]
impl ILinterAdapterPort for MyPyAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("mypy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            path.value.clone(),
            "--no-error-summary".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let response =
            exec_cmd_adapter(self.executor.as_ref(), cmd, working_dir, 120.0, self.name()).await?;

        let stdout = &response.stdout;
        let re = match Regex::new(r"^([^:]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$") {
            Ok(r) => r,
            Err(_) => match Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$") {
                Ok(r) => r,
                Err(_) => return Ok(LintResultList::new(vec![])),
            },
        };
        let re_simple = match Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$") {
            Ok(r) => r,
            Err(_) => return Ok(LintResultList::new(vec![])),
        };
        let mut results = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(caps) = re.captures(line) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let column: i64 = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = match caps.get(4) {
                    Some(m) => m.as_str(),
                    None => "error",
                };
                let message = match caps.get(5) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let code = match caps.get(6) {
                    Some(m) => m.as_str(),
                    None => "",
                };

                let resolved = self.path_norm.resolve_infrastructure_path(
                    match FilePath::new(filename.to_string()) {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    },
                    Some(path.clone()),
                );

                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(line_number),
                    column: ColumnNumber::new(column),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: Some(self.name()),
                    severity: Self::map_severity(msg_type, message),
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            } else if let Some(caps) = re_simple.captures(line) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = match caps.get(3) {
                    Some(m) => m.as_str(),
                    None => "error",
                };
                let message = match caps.get(4) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let code = match caps.get(5) {
                    Some(m) => m.as_str(),
                    None => "",
                };

                let resolved = self.path_norm.resolve_infrastructure_path(
                    match FilePath::new(filename.to_string()) {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    },
                    Some(path.clone()),
                );

                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(line_number),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: Some(self.name()),
                    severity: Self::map_severity(msg_type, message),
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }
        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        noop_apply_fix().await
    }
}
