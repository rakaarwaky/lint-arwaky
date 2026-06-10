// PURPOSE: InjectionContainer — DI container wiring capabilities and protocols via arc_swap references

use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::code_analysis::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;
use crate::output_report::contract_output_aggregate::IReportFormatterProtocol;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;

use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::collections::HashMap;
use std::sync::Arc;

pub type Container = Arc<dyn ServiceContainerAggregate>;

pub struct DependencyInjectionContainer {
    file_system: Arc<dyn IFileSystemPort>,
    command_executor: Arc<dyn ICommandExecutorPort>,
    path_normalization: Arc<dyn IPathNormalizationPort>,
    source_parser: Arc<dyn ISourceParserPort>,
    architecture_linter: Arc<dyn IArchLintProtocol>,
    linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
    metrics_provider: Arc<dyn IMetricsProviderPort>,
}

impl DependencyInjectionContainer {
    pub fn new(_root: DirectoryPath) -> Self {
        let fs: Arc<dyn IFileSystemPort> = Arc::new(
            crate::file_system::infrastructure_filesystem_adapter::OSFileSystemAdapter::new(),
        );
        let executor: Arc<dyn ICommandExecutorPort> = Arc::new(
            crate::cli_transport::infrastructure_transport_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ),
        );
        let path_norm: Arc<dyn IPathNormalizationPort> = Arc::new(
            crate::source_parsing::infrastructure_path_provider::PathNormalizationProvider {},
        );
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(
            crate::source_parsing::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(
                    crate::source_parsing::infrastructure_py_scanner::ASTPythonParserAdapter::new(),
                ),
                Box::new(
                    crate::source_parsing::infrastructure_rust_scanner::ASTRustParserAdapter::new(),
                ),
                Box::new(
                    crate::source_parsing::infrastructure_js_scanner::ASTJSParserAdapter::new(),
                ),
            ),
        );
        let arch_linter: Arc<dyn IArchLintProtocol> = Arc::new(
            crate::code_analysis::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new(),
        );

        let mut linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();

        let ruff = Arc::new(
            crate::language_adapters::infrastructure_py_ruff_adapter::RuffAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("ruff".to_string(), ruff);

        let bandit = Arc::new(
            crate::language_adapters::infrastructure_py_bandit_adapter::BanditAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("bandit".to_string(), bandit);

        let mypy = Arc::new(
            crate::language_adapters::infrastructure_py_mypy_adapter::MyPyAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("mypy".to_string(), mypy);

        let eslint = Arc::new(
            crate::language_adapters::infrastructure_js_linter_adapter::ESLintAdapter::new(
                executor.clone(),
                path_norm.clone(),
            ),
        );
        linter_adapters.insert("eslint".to_string(), eslint);

        let prettier = Arc::new(
            crate::language_adapters::infrastructure_js_linter_adapter::PrettierAdapter::new(
                executor.clone(),
                path_norm.clone(),
            ),
        );
        linter_adapters.insert("prettier".to_string(), prettier);

        let tsc = Arc::new(
            crate::language_adapters::infrastructure_js_linter_adapter::TSCAdapter::new(
                executor.clone(),
                path_norm.clone(),
            ),
        );
        linter_adapters.insert("tsc".to_string(), tsc);

        let clippy = Arc::new(
            crate::language_adapters::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("clippy".to_string(), clippy);

        let complexity = Arc::new(
            crate::language_adapters::infrastructure_py_quality_adapter::ComplexityAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
                Count::new(10),
            ),
        );
        linter_adapters.insert("complexity".to_string(), complexity);

        let duplicate = Arc::new(
            crate::language_adapters::infrastructure_py_quality_adapter::DuplicateAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("duplicate".to_string(), duplicate);

        let trends = Arc::new(
            crate::language_adapters::infrastructure_py_quality_adapter::TrendsAdapter::new(
                executor.clone(),
                path_norm.clone(),
                FilePath::new(".lint_trends.json".to_string()).unwrap_or_default(),
            ),
        );
        linter_adapters.insert("trends".to_string(), trends);

        let dependency = Arc::new(
            crate::language_adapters::infrastructure_py_quality_adapter::DependencyAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            ),
        );
        linter_adapters.insert("dependency".to_string(), dependency);

        let metrics_provider: Arc<dyn IMetricsProviderPort> = Arc::new(
            crate::language_adapters::infrastructure_py_metrics_adapter::MetricsProvider::new(
                path_norm.clone(),
                ".lint_history.json",
            ),
        );

        Self {
            file_system: fs,
            command_executor: executor,
            path_normalization: path_norm,
            source_parser,
            architecture_linter: arch_linter,
            linter_adapters,
            metrics_provider,
        }
    }
}

impl ServiceContainerAggregate for DependencyInjectionContainer {
    fn file_system(&self) -> Arc<dyn IFileSystemPort> {
        self.file_system.clone()
    }

    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort> {
        self.command_executor.clone()
    }

    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        self.path_normalization.clone()
    }

    fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        self.source_parser.clone()
    }

    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>> {
        self.linter_adapters.get(name.value()).cloned()
    }

    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>> {
        Some(self.architecture_linter.clone())
    }

    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>> {
        use std::sync::OnceLock;
        static REGISTRY: OnceLock<Arc<dyn IJobRegistryPort>> = OnceLock::new();
        Some(
            REGISTRY
                .get_or_init(|| Arc::new(crate::pipeline_jobs::infrastructure_registry_adapter::MemoryJobRegistryAdapter::new()))
                .clone(),
        )
    }

    fn metrics_provider(&self) -> Option<Arc<dyn IMetricsProviderPort>> {
        Some(self.metrics_provider.clone())
    }

    fn get_fix_orchestrator(&self, dry_run: bool) -> Option<Arc<dyn LintFixOrchestratorAggregate>> {
        Some(Arc::new(
            crate::code_analysis::capabilities_fix_processor::LintFixProcessor::with_dry_run(
                dry_run,
                self.architecture_linter.clone(),
            ),
        ))
    }

    fn get_report_formatter(&self) -> Option<Box<dyn IReportFormatterProtocol>> {
        Some(Box::new(
            crate::output_report::capabilities_reporting_formatter::ReportFormatterProcessor::new(),
        ))
    }
}
