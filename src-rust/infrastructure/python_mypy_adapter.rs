/// python_mypy_adapter — MyPy adapter for Python type checking.
use crate::contract::{crate::contract::command_executor_port::ICommandExecutorPort, crate::contract::linter_adapter_port::ILinterAdapterPort, crate::contract::path_normalization_port::IPathNormalizationPort, LinterError};
use crate::taxonomy::{AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, FilePath, LineNumber, LintMessage, LintResult, LintResultList, ScanError, Severity, ErrorMessage};
use regex::Regex;
use std::sync::Arc;
use std::time::Duration;

pub struct MyPyAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl MyPyAdapter {
    pub fn new(executor: Arc<dyn ICommandExecutorPort>, path_norm: Arc<dyn IPathNormalizationPort>, bin_path: Option<FilePath>) -> Self {
        Self { executor, path_norm, bin_path }
    }

    fn map_severity(msg_type: &str, msg: &str) -> Severity {
        let m = msg.to_lowercase();
        if msg_type == "note" { return Severity::Low; }
        if m.contains("syntax") || m.contains("parse") { return Severity::Critical; }
        if msg_type == "warning" { return Severity::Medium; }
        Severity::High
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for MyPyAdapter {
    fn name(&self) -> AdapterName { AdapterName::new("mypy").unwrap() }
    async fn scan(&self, _path: &FilePath) -> Result<LintResultList, LinterError> { Ok(LintResultList::new(Vec::new())) }
    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterError> { Ok(ComplianceStatus::new(false)) }
}
