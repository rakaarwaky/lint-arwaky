// PURPOSE: InjectionContainer — DI container wiring capabilities and protocols via arc_swap references

use crate::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::cli_transport::contract_executor_port::ICommandExecutorPort;
use crate::cli_commands::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::code_analysis::contract_analysis_protocol::IAnalysisProtocol;
use crate::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use crate::code_analysis::contract_target_resolver_protocol::ITargetResolverProtocol;
use crate::code_analysis::contract_unused_protocol::IUnusedProtocol;
use crate::config_system::contract_discovery_port::IConfigDiscoveryPort;
use crate::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use crate::config_system::contract_parser_port::IConfigParserPort;
use crate::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::file_watch::contract_provider_port::IWatchProviderPort;
use crate::git_hooks::contract_commands_aggregate::GitCommandsAggregate;
use crate::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use crate::language_adapters::contract_flow_port::IJavascriptFlowPort;
use crate::language_adapters::contract_naming_port::INamingProviderPort;
use crate::language_adapters::contract_scope_port::IJavascriptScopePort;
use crate::language_adapters::contract_semantic_tracer_port::ISemanticTracerPort;
use crate::language_adapters::contract_variant_port::INamingVariantPort;
use crate::import_rules::contract_import_parser_port::IImportParserPort;
use crate::code_analysis::contract_lint_protocol::IArchLintProtocol;
use crate::lifecycle_state::contract_lifecycle_aggregate::AgentLifecycleAggregate;
use crate::mcp_server::contract_server_port::IMcpServerPort;
use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;
use crate::multi_project::contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use crate::output_report::contract_output_aggregate::IReportFormatterProtocol;
use crate::pipeline_jobs::contract_extended_aggregate::PipelineExtendedOrchestratorAggregate;
use crate::pipeline_jobs::contract_output_aggregate::PipelineOutputAggregate;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::plugin_system::contract_manager_port::IPluginManagerPort;
use crate::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;

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
    // Contract ports/protocols/aggregates wired for AES030 orphan detection
    maintenance_aggregate: Arc<dyn MaintenanceCommandsAggregate>,
    analysis_protocol: Arc<dyn IAnalysisProtocol>,
    code_metric_protocol: Arc<dyn ICodeMetricAnalyzerProtocol>,
    target_resolver_protocol: Arc<dyn ITargetResolverProtocol>,
    unused_protocol: Arc<dyn IUnusedProtocol>,
    config_discovery_port: Arc<dyn IConfigDiscoveryPort>,
    config_orchestration_aggregate: Arc<dyn IConfigOrchestrationAggregate>,
    config_parser_port: Arc<dyn IConfigParserPort>,
    config_validator_protocol: Arc<dyn IConfigValidatorProtocol>,
    watch_provider_port: Arc<dyn IWatchProviderPort>,
    git_commands_aggregate: Arc<dyn GitCommandsAggregate>,
    git_orchestrator_aggregate: Arc<dyn HookManagementOrchestratorAggregate>,
    import_parser_port: Arc<dyn IImportParserPort>,
    javascript_flow_port: Arc<dyn IJavascriptFlowPort>,
    naming_provider_port: Arc<dyn INamingProviderPort>,
    javascript_scope_port: Arc<dyn IJavascriptScopePort>,
    semantic_tracer_port: Arc<dyn ISemanticTracerPort>,
    naming_variant_port: Arc<dyn INamingVariantPort>,
    agent_lifecycle_aggregate: Arc<dyn AgentLifecycleAggregate>,
    mcp_server_port: Arc<dyn IMcpServerPort>,
    multi_project_aggregate: Arc<dyn MultiProjectOrchestratorAggregate>,
    pipeline_extended_aggregate: Arc<dyn PipelineExtendedOrchestratorAggregate>,
    pipeline_output_aggregate: Arc<dyn PipelineOutputAggregate>,
    plugin_manager_port: Arc<dyn IPluginManagerPort>,
    setup_management_aggregate: Arc<dyn SetupManagementAggregate>,
    setup_management_protocol: Arc<dyn ISetupManagementProtocol>,
    scanner_provider_port: Arc<dyn IScannerProviderPort>,
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

        // Wire contract ports/protocols/aggregates with real infrastructure implementations
        let maintenance_aggregate: Arc<dyn MaintenanceCommandsAggregate> = Arc::new(
            crate::cli_commands::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator::new(),
        );
        let analysis_protocol: Arc<dyn IAnalysisProtocol> =
            Arc::new(crate::code_analysis::capabilities_analysis_reporter::AnalysisReporter::new());
        let target_resolver_protocol: Arc<dyn ITargetResolverProtocol> = Arc::new(
            crate::code_analysis::capabilities_project_target_resolver::ProjectTargetResolver::new(
            ),
        );
        let code_metric_protocol: Arc<dyn ICodeMetricAnalyzerProtocol> = Arc::new(
            crate::code_analysis::capabilities_code_metric_analyzer::CodeMetricAnalyzer::new(
                target_resolver_protocol.clone(),
            ),
        );
        let unused_protocol: Arc<dyn IUnusedProtocol> = Arc::new(
            crate::code_analysis::capabilities_unused_checker::UnusedImportRuleChecker::new(),
        );
        let config_discovery_port: Arc<dyn IConfigDiscoveryPort> = Arc::new(
            crate::config_system::infrastructure_discovery_provider::ConfigDiscoveryProvider::new(),
        );
        let config_orchestration_aggregate: Arc<dyn IConfigOrchestrationAggregate> = Arc::new(
            crate::config_system::agent_config_loading_orchestrator::ConfigLoadingOrchestrator::new(
                Arc::new(crate::config_system::infrastructure_detector_provider::LanguageDetectorProvider::new()),
                Arc::new(crate::config_system::infrastructure_yaml_reader::ConfigYamlReader::new(path_norm.clone())),
            ),
        );
        let config_parser_port: Arc<dyn IConfigParserPort> = Arc::new(
            crate::config_system::infrastructure_parser_provider::ConfigParserProvider::new(),
        );
        let config_validator_protocol: Arc<dyn IConfigValidatorProtocol> = Arc::new(
            crate::config_system::capabilities_rules_validator::ConfigRulesValidator::new(
                crate::config_system::taxonomy_setting_vo::ProjectConfig::defaults(),
            ),
        );
        let watch_provider_port: Arc<dyn IWatchProviderPort> =
            Arc::new(crate::file_watch::infrastructure_watch_provider::WatchServiceProvider::new());
        let git_commands_aggregate: Arc<dyn GitCommandsAggregate> =
            Arc::new(crate::git_hooks::agent_commands_orchestrator::GitCommandsOrchestrator::new());
        let git_orchestrator_aggregate: Arc<dyn HookManagementOrchestratorAggregate> = Arc::new(
            crate::git_hooks::agent_management_orchestrator::HookManagementOrchestrator::new(),
        );
        let import_parser_port: Arc<dyn IImportParserPort> = Arc::new(
            crate::import_rules::infrastructure_import_parser_adapter::ImportParserAdapter::new(),
        );
        let javascript_flow_port: Arc<dyn IJavascriptFlowPort> =
            Arc::new(crate::language_adapters::infrastructure_js_flow_tracer::JSFlowAdapter::new());
        let naming_provider_port: Arc<dyn INamingProviderPort> = Arc::new(
            crate::language_adapters::infrastructure_js_naming_provider::JavascriptNamingProvider::new(),
        );
        let javascript_scope_port: Arc<dyn IJavascriptScopePort> = Arc::new(
            crate::language_adapters::infrastructure_js_scope_provider::JSScopeProvider::new(),
        );
        let naming_variant_port: Arc<dyn INamingVariantPort> = Arc::new(
            crate::language_adapters::infrastructure_py_variants::PythonNamingVariantProvider::new(
            ),
        );
        let semantic_tracer_port: Arc<dyn ISemanticTracerPort> = Arc::new(
            crate::language_adapters::infrastructure_py_ast_tracer::PythonTracer::new(
                Box::new(crate::language_adapters::infrastructure_py_variants::PythonNamingVariantProvider::new()),
            ),
        );
        let agent_lifecycle_aggregate: Arc<dyn AgentLifecycleAggregate> =
            Arc::new(crate::lifecycle_state::agent_status_lifecycle::LifecycleStateManager::new());
        let mcp_server_port: Arc<dyn IMcpServerPort> = Arc::new(
            crate::mcp_server::infrastructure_server_wrapper::McpServerWrapper::new(
                ".",
                "lint-arwaky",
            ),
        );
        let multi_project_aggregate: Arc<dyn MultiProjectOrchestratorAggregate> = Arc::new(
            crate::multi_project::agent_project_orchestrator::MultiProjectOrchestrator::new(),
        );
        let pipeline_extended_aggregate: Arc<dyn PipelineExtendedOrchestratorAggregate> = Arc::new(
            crate::pipeline_jobs::agent_pipeline_extended_orchestrator::PipelineExtendedOrchestrator::new(),
        );
        let pipeline_output_aggregate: Arc<dyn PipelineOutputAggregate> = Arc::new(
            crate::pipeline_jobs::agent_pipeline_extended_orchestrator::ExtendedPipelineOutput::new(
                crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(true),
                crate::pipeline_jobs::taxonomy_action_vo::JobId::new("default"),
                Some(crate::pipeline_jobs::taxonomy_job_vo::ResponseData {
                    value: None,
                    stdout: "default".to_string(),
                    stderr: String::new(),
                    returncode: 0,
                    metadata: std::collections::HashMap::new(),
                }),
                None,
            ),
        );
        let plugin_manager_port: Arc<dyn IPluginManagerPort> = Arc::new(
            crate::plugin_system::infrastructure_system_provider::PluginSystemProvider::new(),
        );
        let setup_management_aggregate: Arc<dyn SetupManagementAggregate> = Arc::new(
            crate::project_setup::agent_setup_orchestrator::SetupManagementOrchestrator::new(),
        );
        let setup_management_protocol: Arc<dyn ISetupManagementProtocol> = Arc::new(
            crate::project_setup::capabilities_setup_processor::SetupManagementProcessor::new(),
        );
        let scanner_provider_port: Arc<dyn IScannerProviderPort> = Arc::new(
            crate::source_parsing::infrastructure_file_collector::FileCollectorProvider::new(),
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
            // Contract ports/protocols/aggregates wired for AES030 orphan detection
            maintenance_aggregate,
            analysis_protocol,
            code_metric_protocol,
            target_resolver_protocol,
            unused_protocol,
            config_discovery_port,
            config_orchestration_aggregate,
            config_parser_port,
            config_validator_protocol,
            watch_provider_port,
            git_commands_aggregate,
            git_orchestrator_aggregate,
            import_parser_port,
            javascript_flow_port,
            naming_provider_port,
            javascript_scope_port,
            semantic_tracer_port,
            naming_variant_port,
            agent_lifecycle_aggregate,
            mcp_server_port,
            multi_project_aggregate,
            pipeline_extended_aggregate,
            pipeline_output_aggregate,
            plugin_manager_port,
            setup_management_aggregate,
            setup_management_protocol,
            scanner_provider_port,
        }
    }
}

impl ServiceContainerAggregate for DependencyInjectionContainer {
    fn file_system(&self) -> Option<Arc<dyn IFileSystemPort>> {
        Some(self.file_system.clone())
    }

    fn command_executor(&self) -> Option<Arc<dyn ICommandExecutorPort>> {
        Some(self.command_executor.clone())
    }

    fn path_normalization(&self) -> Option<Arc<dyn IPathNormalizationPort>> {
        Some(self.path_normalization.clone())
    }

    fn source_parser(&self) -> Option<Arc<dyn ISourceParserPort>> {
        Some(self.source_parser.clone())
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
        let fix_protocol =
            crate::auto_fix::capabilities_fix_processor::LintFixProcessor::with_dry_run(
                dry_run,
                self.architecture_linter.clone(),
            );
        Some(Arc::new(
            crate::auto_fix::agent_fix_orchestrator::FixOrchestrator::new(Arc::new(fix_protocol)),
        ))
    }

    fn get_report_formatter(&self) -> Option<Box<dyn IReportFormatterProtocol>> {
        Some(Box::new(
            crate::output_report::capabilities_reporting_formatter::ReportFormatterProcessor::new(),
        ))
    }

    // AES030 orphan detection getters
    fn get_maintenance_aggregate(&self) -> Option<Arc<dyn MaintenanceCommandsAggregate>> {
        Some(self.maintenance_aggregate.clone())
    }
    fn get_analysis_protocol(&self) -> Option<Arc<dyn IAnalysisProtocol>> {
        Some(self.analysis_protocol.clone())
    }
    fn get_code_metric_protocol(&self) -> Option<Arc<dyn ICodeMetricAnalyzerProtocol>> {
        Some(self.code_metric_protocol.clone())
    }
    fn get_target_resolver_protocol(&self) -> Option<Arc<dyn ITargetResolverProtocol>> {
        Some(self.target_resolver_protocol.clone())
    }
    fn get_unused_protocol(&self) -> Option<Arc<dyn IUnusedProtocol>> {
        Some(self.unused_protocol.clone())
    }
    fn get_config_discovery_port(&self) -> Option<Arc<dyn IConfigDiscoveryPort>> {
        Some(self.config_discovery_port.clone())
    }
    fn get_config_orchestration_aggregate(
        &self,
    ) -> Option<Arc<dyn IConfigOrchestrationAggregate>> {
        Some(self.config_orchestration_aggregate.clone())
    }
    fn get_config_parser_port(&self) -> Option<Arc<dyn IConfigParserPort>> {
        Some(self.config_parser_port.clone())
    }
    fn get_config_validator_protocol(&self) -> Option<Arc<dyn IConfigValidatorProtocol>> {
        Some(self.config_validator_protocol.clone())
    }
    fn get_watch_provider_port(&self) -> Option<Arc<dyn IWatchProviderPort>> {
        Some(self.watch_provider_port.clone())
    }
    fn get_git_commands_aggregate(&self) -> Option<Arc<dyn GitCommandsAggregate>> {
        Some(self.git_commands_aggregate.clone())
    }
    fn get_git_orchestrator_aggregate(
        &self,
    ) -> Option<Arc<dyn HookManagementOrchestratorAggregate>> {
        Some(self.git_orchestrator_aggregate.clone())
    }
    fn get_import_parser_port(&self) -> Option<Arc<dyn IImportParserPort>> {
        Some(self.import_parser_port.clone())
    }
    fn get_javascript_flow_port(&self) -> Option<Arc<dyn IJavascriptFlowPort>> {
        Some(self.javascript_flow_port.clone())
    }
    fn get_naming_provider_port(&self) -> Option<Arc<dyn INamingProviderPort>> {
        Some(self.naming_provider_port.clone())
    }
    fn get_javascript_scope_port(&self) -> Option<Arc<dyn IJavascriptScopePort>> {
        Some(self.javascript_scope_port.clone())
    }
    fn get_semantic_tracer_port(&self) -> Option<Arc<dyn ISemanticTracerPort>> {
        Some(self.semantic_tracer_port.clone())
    }
    fn get_naming_variant_port(&self) -> Option<Arc<dyn INamingVariantPort>> {
        Some(self.naming_variant_port.clone())
    }
    fn get_agent_lifecycle_aggregate(&self) -> Option<Arc<dyn AgentLifecycleAggregate>> {
        Some(self.agent_lifecycle_aggregate.clone())
    }
    fn get_mcp_server_port(&self) -> Option<Arc<dyn IMcpServerPort>> {
        Some(self.mcp_server_port.clone())
    }
    fn get_multi_project_aggregate(&self) -> Option<Arc<dyn MultiProjectOrchestratorAggregate>> {
        Some(self.multi_project_aggregate.clone())
    }
    fn get_pipeline_extended_aggregate(
        &self,
    ) -> Option<Arc<dyn PipelineExtendedOrchestratorAggregate>> {
        Some(self.pipeline_extended_aggregate.clone())
    }
    fn get_pipeline_output_aggregate(&self) -> Option<Arc<dyn PipelineOutputAggregate>> {
        Some(self.pipeline_output_aggregate.clone())
    }
    fn get_plugin_manager_port(&self) -> Option<Arc<dyn IPluginManagerPort>> {
        Some(self.plugin_manager_port.clone())
    }
    fn get_setup_management_aggregate(&self) -> Option<Arc<dyn SetupManagementAggregate>> {
        Some(self.setup_management_aggregate.clone())
    }
    fn get_setup_management_protocol(&self) -> Option<Arc<dyn ISetupManagementProtocol>> {
        Some(self.setup_management_protocol.clone())
    }
    fn get_scanner_provider_port(&self) -> Option<Arc<dyn IScannerProviderPort>> {
        Some(self.scanner_provider_port.clone())
    }
}
