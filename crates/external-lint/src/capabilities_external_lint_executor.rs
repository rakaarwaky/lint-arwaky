// PURPOSE: ExternalLintExecutor — implements IExternalLintExecutorProtocol
// Wraps ICommandExecutorProtocol and adds error mapping for scan/adapter operations.

use std::sync::Arc;

use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_adapter_error::{AdapterError, ScanError};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;

use shared::external_lint::utility_external_lint::{
    canonicalize_path, resolve_js_cmd, resolve_js_working_dir,
};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
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
        let wd = resolve_js_working_dir(path);
        let abs_path = canonicalize_path(&path.value);
        let cmd = resolve_js_cmd(tool, vec![abs_path, fix_arg.to_string()], &wd.value);
        let response = self
            .exec_cmd_adapter(cmd, wd, 60.0, AdapterName::raw(tool))
            .await?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>) -> Self {
        Self { executor }
    }
}
