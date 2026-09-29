// PURPOSE: ExternalLintExecutor — implements ICommandExecutorProtocol,
// IJsToolResolutionProtocol, and ICargoDirProtocol.
// Wraps the raw command executor and adds error mapping for scan/adapter operations.

use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::common::taxonomy_tool_name_vo::ToolName;
use shared::external_lint::ICargoDirProtocol;
use shared::external_lint::ICommandExecutorProtocol;
use shared::external_lint::IJsToolResolutionProtocol;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_filesystem_protocol::IToolResolutionProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
    io: Arc<dyn IFileSystemIOProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
    executor: Arc<dyn ICommandExecutorProtocol>,
}

// ─── Block 2: FR-006 command execution with error mapping ─

impl ICommandExecutorProtocol for ExternalLintExecutor {
    fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        self.executor.execute_command(command, working_dir, timeout)
    }

    fn health_check(&self) -> anyhow::Result<ResponseData> {
        self.executor.health_check()
    }

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
                Some(Timeout::new(timeout_secs)),
            )
            .map_err(|e| crate::map_scan_error(e, path.clone(), adapter_name))
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
                Some(Timeout::new(timeout_secs)),
            )
            .map_err(|e| crate::map_adapter_error(e, adapter_name))
    }
}

// ─── Block 2b: FR-007 JS tool resolution ──────────────────

impl IJsToolResolutionProtocol for ExternalLintExecutor {
    fn resolve_js_cmd(
        &self,
        tool_name: &ToolName,
        args: Vec<String>,
        working_dir: &FilePath,
    ) -> Option<Vec<String>> {
        self.tool_resolution
            .resolve_js_cmd(tool_name, args, working_dir)
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        self.tool_resolution.resolve_js_working_dir(path)
    }

    fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &ToolName,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        let wd = self.tool_resolution.resolve_js_working_dir(path);
        let abs_path_str = self.io.canonicalize_path_str(path);
        let cmd = match self.tool_resolution.resolve_js_cmd(
            tool,
            vec![abs_path_str.value, fix_arg.to_string()],
            &wd,
        ) {
            Some(c) => c,
            None => {
                return Ok(ComplianceStatus::new(false));
            }
        };
        let response = self.exec_cmd_adapter(cmd, wd, 60.0, AdapterName::raw(tool.value()))?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }
}

// ─── Block 2c: FR-008 cargo working directory resolution ──

impl ICargoDirProtocol for ExternalLintExecutor {
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        self.tool_resolution.resolve_cargo_working_dir(path)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            executor,
            io,
            tool_resolution,
        }
    }
}
