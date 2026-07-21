// PURPOSE: capabilities_external_lint_executor — command execution functions using contract traits
// These functions use ICommandExecutorProtocol and belong in capabilities layer.

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

use crate::utility_external_lint_helper::{
    canonicalize_path, resolve_js_cmd, resolve_js_working_dir,
};

/// Execute a command, mapping execution failures to `LinterOperationError::Scan`.
pub async fn exec_cmd_scan(
    executor: &dyn ICommandExecutorProtocol,
    args: Vec<String>,
    working_dir: FilePath,
    timeout_secs: f64,
    adapter_name: Option<AdapterName>,
    path: &FilePath,
) -> Result<ResponseData, LinterOperationError> {
    executor
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

/// Execute a command, mapping execution failures to `LinterOperationError::Adapter`.
pub async fn exec_cmd_adapter(
    executor: &dyn ICommandExecutorProtocol,
    args: Vec<String>,
    working_dir: FilePath,
    timeout_secs: f64,
    adapter_name: AdapterName,
) -> Result<ResponseData, LinterOperationError> {
    executor
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

/// Applies a JS tool's fix command, returning `Ok(ComplianceStatus::new(true))` on success.
pub async fn js_apply_fix(
    executor: &dyn ICommandExecutorProtocol,
    path: &FilePath,
    tool: &str,
    fix_arg: &str,
) -> Result<ComplianceStatus, LinterOperationError> {
    let wd = resolve_js_working_dir(path);
    let abs_path = canonicalize_path(&path.value);
    let cmd = resolve_js_cmd(tool, vec![abs_path, fix_arg.to_string()], &wd.value);
    let response = exec_cmd_adapter(executor, cmd, wd, 60.0, AdapterName::raw(tool)).await?;
    Ok(ComplianceStatus::new(response.returncode == 0))
}
