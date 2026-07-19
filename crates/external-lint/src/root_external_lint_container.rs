// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with infrastructure adapters
//
// The DI container that assembles the external lint subsystem:
//   1. Creates a StdioClient (ICommandExecutorPort) for subprocess execution
//   2. Creates an ExternalLintLanguageDetectorAdapter (IExternalLintLanguageDetectorPort)
//   3. Creates a CapabilitiesExternalLintSelector (IExternalLintSelectorProtocol)
//   4. Registers all 9 adapters (ruff, bandit, mypy, eslint, prettier, tsc, clippy, rustfmt, cargo-audit)
//   5. Wraps them in a ExternalLintOrchestrator
//   6. Provides a DefaultPathNormalization that passes paths through unchanged
use std::collections::HashMap;
use std::sync::Arc;

use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_language_detector_port::IExternalLintLanguageDetectorPort;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;
use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;

// Block 1: struct Definition
// ─── Block 1: Struct Definition ───────────────────────────
pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

// ─── Block 2: Public Contract ─────────────────────────────
// (No trait impl — root container is wiring only)

// Block 3: constructors & public API
// ─── Block 3: Constructors & Helpers ──────────────────────
impl ExternalLintContainer {
    pub fn new(
        path_norm: Arc<dyn IPathNormalizationPort>,
        language_detector: Arc<dyn IExternalLintLanguageDetectorPort>,
        selector: Arc<dyn IExternalLintSelectorProtocol>,
    ) -> Self {
        let executor: Arc<dyn shared::cli_commands::contract_executor_port::ICommandExecutorPort> =
            Arc::new(crate::infrastructure_stdio_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ));
        let utility: Arc<dyn IExternalLintUtilityPort> = Arc::new(
            crate::infrastructure_external_lint_adapter::ExternalLintUtilityAdapter::new(),
        );
        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::infrastructure_py_ruff_adapter::RuffAdapter::new(
                executor.clone(),
                path_norm.clone(),
                utility.clone(),
                None,
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::infrastructure_py_bandit_adapter::BanditAdapter::new(
                executor.clone(),
                path_norm.clone(),
                utility.clone(),
                None,
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::infrastructure_py_mypy_adapter::MyPyAdapter::new(
                executor.clone(),
                path_norm.clone(),
                utility.clone(),
                None,
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::infrastructure_js_eslint_adapter::ESLintAdapter::new(
                executor.clone(),
                path_norm.clone(),
                utility.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::infrastructure_js_prettier_adapter::PrettierAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    utility.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::infrastructure_js_tsc_adapter::TSCAdapter::new(
                executor.clone(),
                path_norm.clone(),
                utility.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    utility.clone(),
                    None,
                ),
            ),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::infrastructure_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                path_norm.clone(),
                utility.clone(),
                None,
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(
                crate::infrastructure_rs_audit_adapter::CargoAuditAdapter::new(
                    path_norm.clone(),
                    utility.clone(),
                ),
            ),
        );

        Self {
            aggregate: Arc::new(
                crate::agent_external_lint_orchestrator::ExternalLintOrchestrator::new(
                    adapters,
                    language_detector,
                    selector,
                ),
            ),
        }
    }

    pub fn new_default() -> Self {
        let path_norm = Arc::new(DefaultPathNormalization);
        let language_detector: Arc<dyn IExternalLintLanguageDetectorPort> =
            Arc::new(crate::infrastructure_language_detector_adapter::ExternalLintLanguageDetectorAdapter::new());
        let selector: Arc<dyn IExternalLintSelectorProtocol> =
            Arc::new(crate::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults());
        Self::new(path_norm, language_detector, selector)
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}

struct DefaultPathNormalization;
// ─── Block 2: Public Contract ─────────────────────────────
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
