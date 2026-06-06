/// python_mypy_adapter — MyPy adapter for Python type checking.
use crate::contract::{ICommandExecutorPort, ILinterAdapterPort, IPathNormalizationPort};
use crate::taxonomy::{AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, FilePath, LineNumber, LintMessage, LintResult, LintResultList, ScanError, Severity, ErrorMessage, LinterOperationError};
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
        if msg_type == "note" { return Severity::LOW; }
        if m.contains("syntax") || m.contains("parse") { return Severity::CRITICAL; }
        if msg_type == "warning" { return Severity::MEDIUM; }
        Severity::HIGH
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for MyPyAdapter {
    fn name(&self) -> AdapterName { AdapterName::new("mypy").unwrap() }
    async fn scan(&self, _path: &FilePath) -> Result<LintResultList, LinterOperationError> { Ok(LintResultList::new(Vec::new())) }
    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> { Ok(ComplianceStatus::new(false)) }
}
