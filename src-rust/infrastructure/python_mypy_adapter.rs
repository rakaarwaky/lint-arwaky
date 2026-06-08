/// python_mypy_adapter — MyPy adapter for Python type checking.
use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

use crate::contract::{ICommandExecutorPort, ILinterAdapterPort, IPathNormalizationPort};
use crate::taxonomy::{
    AdapterError, AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, ErrorMessage, FilePath,
    LineNumber, LintMessage, LintResult, LintResultList, LinterOperationError, LocationList,
    PatternList, Severity,
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
        self.bin_path
            .as_ref()
            .map(|p| p.value.clone())
            .unwrap_or_else(|| "mypy".to_string())
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
        let command = PatternList::new(cmd);
        let working_dir = FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone());

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(crate::taxonomy::Timeout::new(120.0)),
            )
            .await
        {
            Ok(response) => {
                let stdout = &response.stdout;
                let re = Regex::new(r"^([^:]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\[(\w+)\]$")
                    .unwrap_or_else(|_| {
                        Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[(\w+)\]$")
                            .expect("valid regex")
                    });
                let re_simple = Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[(\w+)\]$")
                    .expect("valid regex");
                let mut results = Vec::new();

                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if let Some(caps) = re.captures(line) {
                        let filename = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                        let line_number: i64 = caps
                            .get(2)
                            .and_then(|m| m.as_str().parse().ok())
                            .unwrap_or(0);
                        let column: i64 = caps
                            .get(3)
                            .and_then(|m| m.as_str().parse().ok())
                            .unwrap_or(0);
                        let msg_type = caps.get(4).map(|m| m.as_str()).unwrap_or("error");
                        let message = caps.get(5).map(|m| m.as_str()).unwrap_or("");
                        let code = caps.get(6).map(|m| m.as_str()).unwrap_or("");

                        let resolved = self.path_norm.resolve_infrastructure_path(
                            FilePath::new(filename.to_string()).unwrap_or_else(|_| path.clone()),
                            Some(path.clone()),
                        );

                        results.push(LintResult::new(
                            resolved,
                            LineNumber::new(line_number),
                            ColumnNumber::new(column),
                            ErrorCode::raw(code),
                            LintMessage::new(message),
                            Some(self.name()),
                            Self::map_severity(msg_type, message),
                            None,
                            LocationList::new(),
                        ));
                    } else if let Some(caps) = re_simple.captures(line) {
                        let filename = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                        let line_number: i64 = caps
                            .get(2)
                            .and_then(|m| m.as_str().parse().ok())
                            .unwrap_or(0);
                        let msg_type = caps.get(3).map(|m| m.as_str()).unwrap_or("error");
                        let message = caps.get(4).map(|m| m.as_str()).unwrap_or("");
                        let code = caps.get(5).map(|m| m.as_str()).unwrap_or("");

                        let resolved = self.path_norm.resolve_infrastructure_path(
                            FilePath::new(filename.to_string()).unwrap_or_else(|_| path.clone()),
                            Some(path.clone()),
                        );

                        results.push(LintResult::new(
                            resolved,
                            LineNumber::new(line_number),
                            ColumnNumber::new(0),
                            ErrorCode::raw(code),
                            LintMessage::new(message),
                            Some(self.name()),
                            Self::map_severity(msg_type, message),
                            None,
                            LocationList::new(),
                        ));
                    }
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("MyPy execution failed: {}", e)),
            ))),
        }
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}
