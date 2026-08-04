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
use crate::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol;
use shared::external_lint::{
    IExternalLintAggregate, IExternalLintExecutorProtocol, IExternalLintSelectorProtocol,
};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,

    ) -> Self {
        let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(
            crate::capabilities_stdio_client::StdioClient::new(Timeout::new(60.0)),
        );

        let lint_executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(
            crate::capabilities_external_lint_executor::ExternalLintExecutor::new(
                executor.clone(),
                filesystem.clone(),
            ),
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
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::capabilities_js_prettier_adapter::PrettierAdapter::new(
                    lint_executor.clone(),
                    filesystem.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::capabilities_js_tsc_adapter::TSCAdapter::new(
                lint_executor.clone(),
                filesystem.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::capabilities_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    None,
                    filesystem.clone(),
                ),
            ),
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
            Arc::new(
                crate::capabilities_rs_audit_adapter::CargoAuditAdapter::new(
                    executor.clone(),
                    filesystem.clone(),
                ),
            ),
        );

        // Create selector via DI (AES201: agent must not import capabilities directly)
        let selector: Arc<dyn IExternalLintSelectorProtocol> =
            Arc::new(CapabilitiesExternalLintSelector::with_defaults());

        Self {
            aggregate: Arc::new(ExternalLintOrchestrator::new(ExternalLintDeps {
                adapters,
                filesystem,
                selector,
            })),
        }
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}

impl Default for ExternalLintContainer {
    fn default() -> Self {
        // NOTE: In production, inject real filesystem and config_parser via new().
        // Default implementation is for testing/quick-start only.
        panic!(
            "ExternalLintContainer::default() requires injected dependencies. Use new() instead."
        )
    }
}
