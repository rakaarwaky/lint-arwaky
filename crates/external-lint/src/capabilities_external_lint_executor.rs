// PURPOSE: ExternalLintExecutor — implements IExternalLintExecutorProtocol
// Wraps ICommandExecutorProtocol and adds error mapping for scan/adapter operations.

use std::sync::Arc;

use shared::common::ScanError;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{ErrorMessage, PatternList};
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    executor: Arc<dyn ICommandExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IExternalLintExecutorProtocol for ExternalLintExecutor {
    fn exec_cmd_scan(
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
                Some(shared::common::taxonomy_duration_vo::Timeout::new(
                    timeout_secs,
                )),
            )
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

    fn exec_cmd_adapter(
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
                Some(shared::common::taxonomy_duration_vo::Timeout::new(
                    timeout_secs,
                )),
            )
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    adapter_name,
                    ErrorMessage::new(e.to_string()),
                ))
            })
    }

    fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        let wd = self.filesystem.resolve_js_working_dir(path);
        let abs_path_str = self.filesystem.canonicalize_path_str(path);
        let tool_name = ToolName::new(tool).unwrap_or_else(|_| ToolName {
            value: tool.to_string(),
        });
        let cmd = match self.filesystem.resolve_js_cmd(
            &tool_name,
            vec![abs_path_str.value, fix_arg.to_string()],
            &wd,
        ) {
            Some(c) => c,
            None => {
                return Ok(ComplianceStatus::new(false));
            }
        };
        let response = self.exec_cmd_adapter(cmd, wd, 60.0, AdapterName::raw(tool))?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            executor,
            filesystem,
        }
    }
}
