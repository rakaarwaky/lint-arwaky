// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with utility adapters
//
// The DI container that assembles the external lint subsystem:
//   1. Creates a StdioClient (ICommandExecutorProtocol) for subprocess execution
//   2. Creates three single-protocol capabilities over that executor:
//      capabilities_command_executor (ICommandExecutorProtocol),
//      capabilities_js_tool_resolver (IJsToolResolutionProtocol), and
//      capabilities_cargo_dir_resolver (ICargoDirProtocol)
//   3. Registers all 10 adapters (ruff, bandit, mypy, eslint, prettier, tsc,
//      markdownlint, clippy, rustfmt, cargo-audit)
//
// Each adapter follows the same pattern: Arc<dyn ILinterAdapterProtocol> in a HashMap keyed by name.
use std::collections::HashMap;
use std::sync::Arc;

use crate::agent_external_lint_orchestrator::{ExternalLintDeps, ExternalLintOrchestrator};
use crate::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared_common::taxonomy_duration_vo::Timeout;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::{IExternalLintAggregate, IExternalLintSelectorProtocol};
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(
            crate::capabilities_stdio_client::StdioClient::new(Timeout::new(60.0)),
        );

        let command_executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(
            crate::capabilities_command_executor::ExternalLintExecutor::new(executor.clone()),
        );
        let js_executor: Arc<dyn IJsToolResolutionProtocol> = Arc::new(
            crate::capabilities_js_tool_resolver::ExternalLintExecutor::new(
                executor.clone(),
                io.clone(),
                tool_resolution.clone(),
            ),
        );
        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::capabilities_py_ruff_adapter::RuffAdapter::new(
                command_executor.clone(),
                None,
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::capabilities_py_bandit_adapter::BanditAdapter::new(
                command_executor.clone(),
                None,
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::capabilities_py_mypy_adapter::MyPyAdapter::new(
                command_executor.clone(),
                None,
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::capabilities_js_eslint_adapter::ESLintAdapter::new(
                command_executor.clone(),
                js_executor.clone(),
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::capabilities_js_prettier_adapter::PrettierAdapter::new(
                    command_executor.clone(),
                    js_executor.clone(),
                    io.clone(),
                    tool_resolution.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::capabilities_js_tsc_adapter::TSCAdapter::new(
                command_executor.clone(),
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "markdownlint".to_string(),
            Arc::new(
                crate::capabilities_md_markdownlint_adapter::MarkdownLintAdapter::new(
                    command_executor.clone(),
                    js_executor.clone(),
                    io.clone(),
                    tool_resolution.clone(),
                ),
            ),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::capabilities_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    None,
                    tool_resolution.clone(),
                ),
            ),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::capabilities_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                None,
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(
                crate::capabilities_rs_audit_adapter::CargoAuditAdapter::new(
                    executor.clone(),
                    tool_resolution.clone(),
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
                filesystem_io: io,
                selector,
            })),
        }
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}
