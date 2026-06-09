/// python_bandit_adapter — Bandit adapter for Python security scanning.
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use crate::di_containers::contract_service_aggregate::{
    ICommandExecutorPort, ILinterAdapterPort, IPathNormalizationPort,
};
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_error::AdapterError;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_operation_error::LinterOperationError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct BanditAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl BanditAdapter {
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
            .unwrap_or_else(|| "bandit".to_string())
    }

    fn map_severity(&self, severity: &str) -> Severity {
        match severity {
            "HIGH" => Severity::HIGH,
            "MEDIUM" => Severity::MEDIUM,
            "LOW" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[async_trait]
impl ILinterAdapterPort for BanditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("bandit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "-r".to_string(),
            path.value.clone(),
            "--format".to_string(),
            "json".to_string(),
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
                    120.0,
                )),
            )
            .await
        {
            Ok(response) => {
                let stdout = &response.stdout;
                let parsed: Value =
                    serde_json::from_str(stdout).unwrap_or(Value::Object(serde_json::Map::new()));
                let findings = parsed
                    .get("results")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut results = Vec::new();

                for f in findings {
                    let filename = f.get("filename").and_then(|v| v.as_str()).unwrap_or("");
                    let line_number = f.get("line_number").and_then(|v| v.as_i64()).unwrap_or(0);
                    let line_range = f
                        .get("line_range")
                        .and_then(|v| v.as_array())
                        .and_then(|a| a.first())
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    let test_id = f.get("test_id").and_then(|v| v.as_str()).unwrap_or("B000");
                    let issue_text = f.get("issue_text").and_then(|v| v.as_str()).unwrap_or("");
                    let issue_severity = f
                        .get("issue_severity")
                        .and_then(|v| v.as_str())
                        .unwrap_or("MEDIUM");

                    let resolved = self.path_norm.resolve_infrastructure_path(
                        FilePath::new(filename.to_string()).unwrap_or_else(|_| path.clone()),
                        Some(path.clone()),
                    );

                    results.push(LintResult {
                        file: resolved,
                        line: LineNumber::new(line_number),
                        column: ColumnNumber::new(line_range),
                        code: ErrorCode::raw(test_id),
                        message: LintMessage::new(issue_text),
                        source: Some(self.name()),
                        severity: self.map_severity(issue_severity),
                        enclosing_scope: None,
                        related_locations: LocationList::new(),
                    });
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("Bandit execution failed: {}", e)),
            ))),
        }
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}
