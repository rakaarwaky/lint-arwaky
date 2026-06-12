// PURPOSE: CompositionRoot — composition root container (root_container layer)
use std::collections::HashMap;
use std::sync::Arc;

// Contract/Port types from shared crate
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::metrics_service::contract_metrics_port::IMetricsProviderPort;
use shared::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_adapter_name_vo::AdapterName;

// Infrastructure implementations from feature crates

pub struct CompositionRoot {
    // Legacy fields (for ServiceContainerAggregate backward compat)
    #[allow(dead_code)]
    linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
    arch_linter: Arc<dyn IArchLintProtocol>,
    // Infrastructure fields (not in trait but needed for wiring)
    #[allow(dead_code)]
    file_system: Arc<dyn IFileSystemPort>,
    #[allow(dead_code)]
    executor: Arc<dyn ICommandExecutorPort>,
    #[allow(dead_code)]
    path_norm: Arc<dyn IPathNormalizationPort>,
    #[allow(dead_code)]
    source_parser: Arc<dyn ISourceParserPort>,
    #[allow(dead_code)]
    metrics: Arc<dyn IMetricsProviderPort>,
}

impl CompositionRoot {
    pub fn new() -> Self {
        let file_system: Arc<dyn IFileSystemPort> = Arc::new(
            file_system::infrastructure_filesystem_adapter::OSFileSystemAdapter::new(),
        );
        let executor: Arc<dyn ICommandExecutorPort> = Arc::new(
            cli_commands::infrastructure_transport_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ),
        );
        let path_norm: Arc<dyn IPathNormalizationPort> = Arc::new(
            source_parsing::infrastructure_path_provider::PathNormalizationProvider {},
        );
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(
            source_parsing::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(
                    source_parsing::infrastructure_py_scanner::ASTPythonParserAdapter::new(),
                ),
                Box::new(
                    source_parsing::infrastructure_rust_scanner::ASTRustParserAdapter::new(),
                ),
                Box::new(
                    source_parsing::infrastructure_js_scanner::ASTJSParserAdapter::new(),
                ),
            ),
        );
        let arch_linter: Arc<dyn IArchLintProtocol> = Arc::new(
            code_analysis::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new(),
        );

        let mut linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        // Wire adapters...
        let ruff = Arc::new(
            language_adapters::infrastructure_py_ruff_adapter::RuffAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("ruff".to_string(), ruff);
        let bandit = Arc::new(
            language_adapters::infrastructure_py_bandit_adapter::BanditAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("bandit".to_string(), bandit);
        let mypy = Arc::new(
            language_adapters::infrastructure_py_mypy_adapter::MyPyAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("mypy".to_string(), mypy);
        let eslint = Arc::new(
            language_adapters::infrastructure_js_linter_adapter::ESLintAdapter::new(
                executor.clone(),
                path_norm.clone(),
            ),
        );
        linter_adapters.insert("eslint".to_string(), eslint);
        let prettier = Arc::new(
            language_adapters::infrastructure_js_linter_adapter::PrettierAdapter::new(
                executor.clone(),
                path_norm.clone(),
            ),
        );
        linter_adapters.insert("prettier".to_string(), prettier);
        let tsc = Arc::new(
            language_adapters::infrastructure_js_linter_adapter::TSCAdapter::new(
                executor.clone(),
                path_norm.clone(),
            ),
        );
        linter_adapters.insert("tsc".to_string(), tsc);
        let clippy = Arc::new(
            language_adapters::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("clippy".to_string(), clippy);

        let metrics: Arc<dyn IMetricsProviderPort> = Arc::new(
            metrics_service::infrastructure_py_metrics_adapter::MetricsProvider::new(
                path_norm.clone(),
                ".lint_history.json",
            ),
        );

        Self {
            file_system,
            executor,
            path_norm,
            source_parser,
            arch_linter,
            linter_adapters,
            metrics,
        }
    }

    // Helper for CLI initialization
    #[allow(dead_code)]
    pub fn checker_container(&self) -> code_analysis::root_container::CheckerContainer {
        code_analysis::root_container::CheckerContainer::new()
    }
}

// BACKWARD COMPAT: Implement old trait for existing surface commands
impl shared::common::contract_service_aggregate::ServiceContainerAggregate for CompositionRoot {
    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>> {
        self.linter_adapters.get(name.value()).cloned()
    }

    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>> {
        Some(self.arch_linter.clone())
    }

    fn get_fix_orchestrator(
        &self,
        _dry_run: bool,
    ) -> Option<Arc<dyn LintFixOrchestratorAggregate>> {
        None
    }

    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>> {
        None
    }
}