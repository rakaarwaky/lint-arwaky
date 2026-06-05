use crate::contract::{crate::contract::linter_adapter_port::ILinterAdapterPort, crate::contract::command_executor_port::ICommandExecutorPort, crate::contract::path_normalization_port::IPathNormalizationPort};
use crate::taxonomy::*;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

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
            "error" => Severity::ERROR,
            "warning" => Severity::WARNING,
            "info" => Severity::INFO,
            _ => Severity::WARNING,
        }
    }
}

#[async_trait]
impl ILinterAdapterPort for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::new("ruff")
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
        let working_dir = FilePath::new("."); // Simplified

        match self.executor.execute_command(command, working_dir, Some(std::time::Duration::from_secs(60))).await {
            Ok(response) => {
                let stdout = response.stdout.value;
                let findings: Vec<Value> = serde_json::from_str(&stdout).unwrap_or_default();
                let mut results = Vec::new();

                for f in findings {
                    let filename = f.get("filename").and_then(|v| v.as_ref()).unwrap_or("");
                    let row = f.get("location").and_then(|l| l.get("row")).and_then(|v| v.as_i64()).unwrap_or(0);
                    let col = f.get("location").and_then(|l| l.get("column")).and_then(|v| v.as_i64()).unwrap_or(0);
                    let code = f.get("code").and_then(|v| v.as_ref()).unwrap_or("UNKNOWN");
                    let message = f.get("message").and_then(|v| v.as_ref()).unwrap_or("");
                    let severity_str = f.get("severity").and_then(|v| v.as_ref()).unwrap_or("");

                    results.push(LintResult {
                        file: FilePath::new(filename),
                        line: LineNumber::new(row),
                        column: ColumnNumber::new(col),
                        code: ErrorCode::new(code),
                        message: LintMessage::new(message),
                        source: Some(self.name()),
                        severity: self.map_severity(severity_str, code),
                        ..Default::default()
                    });
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError {
                message: ErrorMessage::new(format!("Ruff execution failed: {}", e)),
                ..Default::default()
            })),
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
        let working_dir = FilePath::new(".");

        match self.executor.execute_command(command, working_dir, Some(std::time::Duration::from_secs(60))).await {
            Ok(_) => Ok(ComplianceStatus::new(true)),
            Err(e) => Err(LinterOperationError::Adapter(AdapterError {
                message: ErrorMessage::new(format!("Ruff fix failed: {}", e)),
                ..Default::default()
            })),
        }
    }
}
