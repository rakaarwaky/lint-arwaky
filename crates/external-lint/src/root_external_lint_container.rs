// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with utility adapters
//
// The DI container that assembles the external lint subsystem:
//   1. Creates a StdioClient (ICommandExecutorProtocol) for subprocess execution
//   2. Creates ExternalLintExecutor (IExternalLintExecutorProtocol) for command execution
//   3. Registers all 9 adapters (ruff, bandit, mypy, eslint, prettier, tsc, clippy, rustfmt, cargo-audit)
//
// Each adapter follows the same pattern: Arc<dyn ILinterAdapterProtocol> in a HashMap keyed by name.
use std::collections::HashMap;
use std::sync::Arc;

use crate::agent_external_lint_orchestrator::{ExternalLintDeps, ExternalLintOrchestrator};
use shared::code_analysis::ILinterAdapterProtocol;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::external_lint::{IExternalLintAggregate, IExternalLintExecutorProtocol};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use shared::common::Timeout;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        config_parser: Arc<dyn IConfigParserProtocol>,
    ) -> Self {
        let executor: Arc<
            dyn shared::common::contract_executor_protocol::ICommandExecutorProtocol,
        > = Arc::new(crate::capabilities_stdio_client::StdioClient::new(
            Timeout::new(60.0),
        ));

        let lint_executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(
            crate::capabilities_external_lint_executor::ExternalLintExecutor::new(executor.clone(), filesystem.clone()),
        );

        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::capabilities_py_ruff_adapter::RuffAdapter::new(
                lint_executor.clone(),
                None,
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::capabilities_py_bandit_adapter::BanditAdapter::new(
                lint_executor.clone(),
                None,
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::capabilities_py_mypy_adapter::MyPyAdapter::new(
                lint_executor.clone(),
                None,
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::capabilities_js_eslint_adapter::ESLintAdapter::new(
                lint_executor.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::capabilities_js_prettier_adapter::PrettierAdapter::new(
                    lint_executor.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::capabilities_js_tsc_adapter::TSCAdapter::new(
                lint_executor.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(crate::capabilities_rs_clippy_adapter::RustLinterAdapter::new(
                executor.clone(),
                None,
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::capabilities_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                None,
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(crate::capabilities_rs_audit_adapter::CargoAuditAdapter::new(filesystem.clone())),
        );

        Self {
            aggregate: Arc::new(ExternalLintOrchestrator::new(ExternalLintDeps {
                adapters,
                filesystem,
                config_parser,
            })),
        }
    }

    pub fn new_default() -> Self {
        let filesystem: Arc<dyn IFilesystemAggregate> =
            Arc::new(filesystem::FilesystemOrchestrator::new());
        let config_parser: Arc<dyn IConfigParserProtocol> =
            Arc::new(config_system::ConfigParserProvider::new());
        Self::new(filesystem, config_parser)
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}

impl Default for ExternalLintContainer {
    fn default() -> Self {
        Self::new_default()
    }
}
