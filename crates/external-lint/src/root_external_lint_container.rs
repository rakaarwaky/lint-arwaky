// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with infrastructure adapters
use std::collections::HashMap;
use std::sync::Arc;

use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>) -> Self {
        let executor: Arc<dyn shared::cli_commands::contract_executor_port::ICommandExecutorPort> =
            Arc::new(crate::infrastructure_stdio_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ));
        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::infrastructure_py_ruff_adapter::RuffAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::infrastructure_py_bandit_adapter::BanditAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::infrastructure_py_mypy_adapter::MyPyAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::infrastructure_js_eslint_adapter::ESLintAdapter::new(
                executor.clone(),
                path_norm.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::infrastructure_js_prettier_adapter::PrettierAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::infrastructure_js_tsc_adapter::TSCAdapter::new(
                executor.clone(),
                path_norm.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    None,
                ),
            ),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::infrastructure_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(
                crate::infrastructure_rs_audit_adapter::CargoAuditAdapter::new(path_norm.clone()),
            ),
        );

        Self {
            aggregate: Arc::new(
                crate::agent_external_lint_orchestrator::ExternalLintOrchestrator::new(adapters),
            ),
        }
    }

    pub fn new_default() -> Self {
        Self::new(Arc::new(DefaultPathNormalization))
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}

struct DefaultPathNormalization;
impl IPathNormalizationPort for DefaultPathNormalization {
    fn normalize_path(&self, path: FilePath) -> FilePath {
        path
    }
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        _context_path: Option<FilePath>,
    ) -> FilePath {
        path
    }
}
