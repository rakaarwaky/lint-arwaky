// PURPOSE: external-lint-domain capability contracts (AES102 `_protocol`).
//
// One file for the external-lint feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::LintResultList;
use crate::quality_rules::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_adapter_list_vo::AdapterNameList;

pub trait ILinterAdapterProtocol: Send + Sync {
    fn name(&self) -> AdapterName;
    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}

pub trait ICommandExecutorProtocol: Send + Sync {
    /// Execute a command and return the response.
    fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    fn health_check(&self) -> anyhow::Result<ResponseData>;
}

/// Protocol for executing external linter commands.
///
/// Implementations wrap `ICommandExecutorProtocol` and add error mapping
/// for scan and adapter operations.
pub trait IExternalLintExecutorProtocol: Send + Sync {
    /// Execute a command, mapping failures to `LinterOperationError::Scan`.
    fn exec_cmd_scan(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Execute a command, mapping failures to `LinterOperationError::Adapter`.
    fn exec_cmd_adapter(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Apply a JS tool's fix command.
    fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError>;
}

/// Protocol for choosing which external-lint adapters to run.
///
/// Based on booleans indicating the presence of Rust, Python, or TypeScript
/// files in the project, the selector returns the list of adapter names
/// that should be invoked during the external linting phase.
pub trait IExternalLintSelectorProtocol: Send + Sync {
    fn select_adapters(&self, has_rs: bool, has_py: bool, has_js: bool) -> AdapterNameList;
}
