use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::{BooleanVO, PatternList};
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use crate::common::taxonomy_response_data_vo::ResponseData;
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintUtilityPort: Send + Sync {
    fn canonicalize_path(&self, path_str: &str) -> FilePath;
    fn default_working_dir(&self, path: &FilePath) -> FilePath;
    fn has_python_files(&self, path: &FilePath) -> BooleanVO;
    fn has_py_in_dir(&self, dir: &DirectoryPath) -> BooleanVO;
    fn is_in_path(&self, executable: &str) -> BooleanVO;
    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList;
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath;
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath;
    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath;
    async fn exec_cmd_scan(
        &self,
        executor: &dyn ICommandExecutorPort,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;
    async fn exec_cmd_adapter(
        &self,
        executor: &dyn ICommandExecutorPort,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;
    async fn js_apply_fix(
        &self,
        executor: &dyn ICommandExecutorPort,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError>;
    async fn noop_apply_fix(&self) -> Result<ComplianceStatus, LinterOperationError>;
}
