// PURPOSE: ExternalLintExecutor — implements IExternalLintExecutorProtocol
// Wraps ICommandExecutorProtocol and adds error mapping for scan/adapter operations.

use std::sync::Arc;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use shared::code_analysis::LinterOperationError;
use shared::common::ICommandExecutorProtocol;
use shared::common::{AdapterError, ScanError};
use shared::common::{
    AdapterName, ComplianceStatus, ErrorMessage, FilePath, PatternList, ResponseData, Timeout,
};

use shared::external_lint::IExternalLintExecutorProtocol;


// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
        pub filesystem: Arc<dyn IFilesystemAggregate>,
    executor: Arc<dyn ICommandExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IExternalLintExecutorProtocol for ExternalLintExecutor {
    async fn exec_cmd_scan(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        self.executor
            .execute_command(
                PatternList::new(args),
                working_dir,
                Some(Timeout::new(timeout_secs)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name,
                    cause: None,
                })
            })
    }

    async fn exec_cmd_adapter(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        self.executor
            .execute_command(
                PatternList::new(args),
                working_dir,
                Some(Timeout::new(timeout_secs)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    adapter_name,
                    ErrorMessage::new(e.to_string()),
                ))
            })
    }

    async fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        let wd = self.filesystem.resolve_js_working_dir(path);
        let abs_path = self.filesystem.canonicalize_path_str(&path.value);
        let cmd = match self.filesystem.resolve_js_cmd(tool, vec![abs_path.clone(), fix_arg.to_string()], &wd.value) {
            Some(c) => c,
            None => {
                return Ok(ComplianceStatus::new(false));
            }
        };
        let response = self
            .exec_cmd_adapter(cmd, wd, 60.0, AdapterName::raw(tool))
            .await?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self { executor, filesystem }
    }
}
