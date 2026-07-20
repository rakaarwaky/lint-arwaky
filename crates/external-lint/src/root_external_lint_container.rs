// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with infrastructure adapters
//
// The DI container that assembles the external lint subsystem:
//   1. Creates a StdioClient (ICommandExecutorProtocol) for subprocess execution
//   2. Registers all 9 adapters (ruff, bandit, mypy, eslint, prettier, tsc, clippy, rustfmt, cargo-audit)
//
// Each adapter follows the same pattern: Arc<dyn ILinterAdapterProtocol> in a HashMap keyed by name.
// Path normalization uses shared::common::utility_path_normalization free functions (no DI).
use std::collections::HashMap;
use std::sync::Arc;

use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new() -> Self {
        let executor: Arc<dyn shared::cli_commands::contract_executor_protocol::ICommandExecutorProtocol> =
            Arc::new(crate::capabilities_stdio_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ));
        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::capabilities_py_ruff_adapter::RuffAdapter::new(
                executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::capabilities_py_bandit_adapter::BanditAdapter::new(
                executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::capabilities_py_mypy_adapter::MyPyAdapter::new(
                executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::capabilities_js_eslint_adapter::ESLintAdapter::new(
                executor.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(crate::capabilities_js_prettier_adapter::PrettierAdapter::new(
                executor.clone(),
            )),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::capabilities_js_tsc_adapter::TSCAdapter::new(
                executor.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(crate::capabilities_rs_clippy_adapter::RustLinterAdapter::new(
                executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::capabilities_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(crate::capabilities_rs_audit_adapter::CargoAuditAdapter::new()),
        );

        Self {
            aggregate: Arc::new(
                crate::agent_external_lint_orchestrator::ExternalLintOrchestrator::new(adapters),
            ),
        }
    }

    pub fn new_default() -> Self {
        Self::new()
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}

impl Default for ExternalLintContainer {
    fn default() -> Self {
        Self::new()
    }
}
