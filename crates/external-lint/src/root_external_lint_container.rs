// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with utility adapters
//
// The DI container that assembles the external lint subsystem:
//   1. Registers all 10 adapters (ruff, bandit, mypy, eslint, prettier, tsc,
//      markdownlint, clippy, rustfmt, cargo-audit)
//   2. Wires default adapter groups into the orchestrator (no selector protocol)

use std::collections::HashMap;
use std::sync::Arc;

use crate::agent_external_lint_orchestrator::{ExternalLintDeps, ExternalLintOrchestrator};
use shared::external_lint::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_filesystem_protocol::IToolResolutionProtocol;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::capabilities_py_ruff_adapter::RuffAdapter::new(
                None,
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::capabilities_py_bandit_adapter::BanditAdapter::new(
                None,
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::capabilities_py_mypy_adapter::MyPyAdapter::new(
                None,
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::capabilities_js_eslint_adapter::ESLintAdapter::new(
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::capabilities_js_prettier_adapter::PrettierAdapter::new(
                    io.clone(),
                    tool_resolution.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::capabilities_js_tsc_adapter::TSCAdapter::new(
                io.clone(),
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "markdownlint".to_string(),
            Arc::new(
                crate::capabilities_md_markdownlint_adapter::MarkdownLintAdapter::new(
                    io.clone(),
                    tool_resolution.clone(),
                ),
            ),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::capabilities_rs_clippy_adapter::RustLinterAdapter::new(
                    None,
                    tool_resolution.clone(),
                ),
            ),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::capabilities_rs_fmt_adapter::RustFmtAdapter::new(
                None,
                tool_resolution.clone(),
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(
                crate::capabilities_rs_audit_adapter::CargoAuditAdapter::new(
                    tool_resolution.clone(),
                ),
            ),
        );

        // Default adapter groups — no separate selector protocol needed.
        let adapter_groups = Some(ExternalLintOrchestrator::new_default_groups());

        Self {
            aggregate: Arc::new(ExternalLintOrchestrator::new(ExternalLintDeps {
                adapters,
                filesystem,
                filesystem_io: io,
                adapter_groups,
            })),
        }
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}
