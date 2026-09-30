// PURPOSE: ExternalLintExecutor — implements IJsToolResolutionProtocol.
// Resolves JS/TS tool commands and working directories via node_modules/.bin.

use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_tool_name_vo::ToolName;
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

// ─── Block 2: FR-007 JS tool resolution ──────────────────

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
        let response =
            self.executor
                .exec_cmd_adapter(cmd, wd, 60.0, AdapterName::raw(tool.value()))?;
        Ok(ComplianceStatus::new(response.returncode == 0))
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
