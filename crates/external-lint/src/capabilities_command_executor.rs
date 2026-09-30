// PURPOSE: ExternalLintExecutor — implements ICommandExecutorProtocol.
// Wraps the raw command executor and adds error mapping for scan/adapter operations.

use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_protocol::ICommandExecutorProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>) -> Self {
        Self { executor }
    }
}
