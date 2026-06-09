//! python_ruff_adapter — Ruff linter adapter for Python files.

use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_error::AdapterError;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_operation_error::LinterOperationError;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct RuffAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl RuffAdapter {
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
        self.bin_path
            .as_ref()
            .map(|p| p.value.clone())
            .unwrap_or_else(|| "ruff".to_string())
    }

    fn map_severity(&self, severity: &str, _code: &str) -> Severity {
        match severity {
            "error" => Severity::HIGH,
            "warning" => Severity::MEDIUM,
            "info" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[async_trait]
impl ILinterAdapterPort for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("ruff")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--output-format=json".to_string(),
            "--exit-zero".to_string(),
            "--no-cache".to_string(),
        ];
        let command = PatternList::new(cmd);
        let working_dir = FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone());

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
            )
            .await
        {
            Ok(response) => {
                let stdout = &response.stdout;
                let findings: Vec<Value> = serde_json::from_str(stdout).unwrap_or_default();
                let mut results = Vec::new();

                for f in findings {
                    let filename = f.get("filename").and_then(|v| v.as_str()).unwrap_or("");
                    let row = f
                        .get("location")
                        .and_then(|l| l.get("row"))
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    let col = f
                        .get("location")
                        .and_then(|l| l.get("column"))
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    let code = f.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
                    let message = f.get("message").and_then(|v| v.as_str()).unwrap_or("");
                    let severity_str = f.get("severity").and_then(|v| v.as_str()).unwrap_or("");

                    let resolved = self.path_norm.resolve_infrastructure_path(
                        FilePath::new(filename).unwrap_or_else(|_| path.clone()),
                        Some(path.clone()),
                    );

                    results.push(LintResult {
                        file: resolved,
                        line: LineNumber::new(row),
                        column: ColumnNumber::new(col),
                        code: ErrorCode::raw(code),
                        message: LintMessage::new(message),
                        source: Some(self.name()),
                        severity: self.map_severity(severity_str, code),
                        enclosing_scope: None,
                        related_locations: LocationList::new(),
                    });
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("Ruff execution failed: {}", e)),
            ))),
        }
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--fix".to_string(),
            "--exit-zero".to_string(),
        ];
        let command = PatternList::new(cmd);
        let working_dir = FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone());

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
            )
            .await
        {
            Ok(_) => Ok(ComplianceStatus::new(true)),
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("Ruff fix failed: {}", e)),
            ))),
        }
    }
}
