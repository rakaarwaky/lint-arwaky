// PURPOSE: CompositionRoot — composition root container (root layer, wiring only)
//
// Delegates to feature containers for infrastructure instantiation.
// Only wires cross-cutting concerns: linter adapters (need executor + path_norm from containers).
use std::collections::HashMap;
use std::sync::Arc;

use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::pipeline_jobs::contract_registry_port::IJobRegistryPort;

use auto_fix::root_auto_fix_container::AutoFixContainer;
use code_analysis::root_code_analysis_container::AnalysisContainer;
use import_rules::root_import_rules_container::ImportContainer;
use pipeline_jobs::root_pipeline_jobs_container::PipelineContainer;
use source_parsing::root_source_parsing_container::SourceParsingContainer;

pub struct CompositionRoot {
    arch_linter: Arc<dyn IArchLintProtocol>,
    #[allow(dead_code)]
    import_container: ImportContainer,
    auto_fix_container: AutoFixContainer,
    pipeline_container: PipelineContainer,
    linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
}

impl CompositionRoot {
    pub fn new() -> Self {
        let source_parsing_container = SourceParsingContainer::new();
        let path_norm = source_parsing_container.path_normalization();

        let arch_linter = AnalysisContainer::new().architecture_linter();

        let auto_fix_container = AutoFixContainer::new(arch_linter.clone());
        let import_container = ImportContainer::new();
        let pipeline_container = PipelineContainer::new();

        let executor =
            Arc::new(cli_commands::infrastructure_transport_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ));

        let mut linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        linter_adapters.insert(
            "ruff".to_string(),
            Arc::new(
                language_adapters::infrastructure_py_ruff_adapter::RuffAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    None,
                ),
            ),
        );
        linter_adapters.insert(
            "bandit".to_string(),
            Arc::new(
                language_adapters::infrastructure_py_bandit_adapter::BanditAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    None,
                ),
            ),
        );
        linter_adapters.insert(
            "mypy".to_string(),
            Arc::new(
                language_adapters::infrastructure_py_mypy_adapter::MyPyAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    None,
                ),
            ),
        );
        linter_adapters.insert(
            "eslint".to_string(),
            Arc::new(
                language_adapters::infrastructure_js_linter_adapter::ESLintAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                ),
            ),
        );
        linter_adapters.insert(
            "prettier".to_string(),
            Arc::new(
                language_adapters::infrastructure_js_linter_adapter::PrettierAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                ),
            ),
        );
        linter_adapters.insert(
            "tsc".to_string(),
            Arc::new(
                language_adapters::infrastructure_js_linter_adapter::TSCAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                ),
            ),
        );
        linter_adapters.insert(
            "clippy".to_string(),
            Arc::new(
                language_adapters::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    None,
                ),
            ),
        );

        Self {
            arch_linter,
            import_container,
            auto_fix_container,
            pipeline_container,
            linter_adapters,
        }
    }

    #[allow(dead_code)]
    pub fn checker_container(&self) -> code_analysis::root_code_analysis_container::CheckerContainer {
        let analyzer = self.import_container.analyzer();
        code_analysis::root_code_analysis_container::CheckerContainer::new(analyzer)
    }
}

impl shared::common::contract_service_aggregate::ServiceContainerAggregate for CompositionRoot {
    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>> {
        self.linter_adapters.get(name.value()).cloned()
    }

    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>> {
        Some(self.arch_linter.clone())
    }

    fn get_fix_orchestrator(
        &self,
        dry_run: bool,
    ) -> Option<Arc<dyn LintFixOrchestratorAggregate>> {
        Some(self.auto_fix_container.orchestrator(dry_run))
    }

    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>> {
        Some(self.pipeline_container.job_registry())
    }
}
