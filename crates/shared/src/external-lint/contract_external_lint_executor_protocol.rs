// PURPOSE: IExternalLintExecutorProtocol — protocol for external lint command execution
// Defines the interface for executing linter commands with error mapping.

use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;

/// Protocol for executing external linter commands.
///
/// Implementations wrap `ICommandExecutorProtocol` and add error mapping
/// for scan and adapter operations.
#[async_trait::async_trait]
pub trait IExternalLintExecutorProtocol: Send + Sync {
    /// Execute a command, mapping failures to `LinterOperationError::Scan`.
    async fn exec_cmd_scan(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Execute a command, mapping failures to `LinterOperationError::Adapter`.
    async fn exec_cmd_adapter(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Apply a JS tool's fix command.
    async fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError>;
}
