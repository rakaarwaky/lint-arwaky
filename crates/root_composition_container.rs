// PURPOSE: CompositionRoot — composition root container (root_container layer)
use std::sync::Arc;
use std::collections::HashMap;

use crate::import_rules::import_container::ImportContainer;
use crate::naming_rules::naming_container::NamingContainer;
use crate::role_rules::role_container::RoleContainer;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::cli_transport::contract_executor_port::ICommandExecutorPort;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::code_analysis::contract_lint_protocol::IArchLintProtocol;
use crate::code_analysis::contract_analysis_protocol::IAnalysisProtocol;
use crate::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use crate::code_analysis::contract_target_resolver_protocol::ITargetResolverProtocol;
use crate::code_analysis::contract_unused_protocol::IUnusedProtocol;
use crate::config_system::contract_discovery_port::IConfigDiscoveryPort;
use crate::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use crate::config_system::contract_parser_port::IConfigParserPort;
use crate::config_system::contract_validator_protocol::IConfigValidatorProtocol;
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
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use crate::cli_commands::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use crate::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;

pub struct CompositionRoot {
    // Feature containers (new pattern)
    import: ImportContainer,
    naming: NamingContainer,
    role: RoleContainer,

    // Legacy fields (for ServiceContainerAggregate backward compat)
    file_system: Arc<dyn IFileSystemPort>,
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    source_parser: Arc<dyn ISourceParserPort>,
    arch_linter: Arc<dyn IArchLintProtocol>,
    linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
    metrics: Arc<dyn IMetricsProviderPort>,
    // ... other fields as needed
}

impl CompositionRoot {
    pub fn new() -> Self {
        let import = ImportContainer::new();
        let naming = NamingContainer::new();
        let role = RoleContainer::new();

        let file_system: Arc<dyn IFileSystemPort> = Arc::new(
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
        // Wire adapters...
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

        let metrics: Arc<dyn IMetricsProviderPort> = Arc::new(
            crate::language_adapters::infrastructure_py_metrics_adapter::MetricsProvider::new(
                path_norm.clone(),
                ".lint_history.json",
            ),
        );

        Self {
            import,
            naming,
            role,
            file_system,
            executor,
            path_norm,
            source_parser,
            arch_linter,
            linter_adapters,
            metrics,
        }
    }

    // NEW: Typed orchestrators (feature-specific)
    pub fn import_orchestrator(&self) -> Arc<dyn crate::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate> {
        self.import.orchestrator()
    }

    pub fn naming_orchestrator(&self) -> Arc<dyn crate::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate> {
        self.naming.orchestrator()
    }

    pub fn role_orchestrator(&self) -> Arc<dyn crate::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate> {
        self.role.orchestrator()
    }
}

// BACKWARD COMPAT: Implement old trait for existing surface commands
impl ServiceContainerAggregate for CompositionRoot {
    fn file_system(&self) -> Option<Arc<dyn IFileSystemPort>> {
        Some(self.file_system.clone())
    }
    fn command_executor(&self) -> Option<Arc<dyn ICommandExecutorPort>> {
        Some(self.executor.clone())
    }
    fn path_normalization(&self) -> Option<Arc<dyn IPathNormalizationPort>> {
        Some(self.path_norm.clone())
    }
    fn source_parser(&self) -> Option<Arc<dyn ISourceParserPort>> {
        Some(self.source_parser.clone())
    }
    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>> {
        self.linter_adapters.get(name.value()).cloned()
    }
    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>> {
        Some(self.arch_linter.clone())
    }
    fn metrics_provider(&self) -> Option<Arc<dyn IMetricsProviderPort>> {
        Some(self.metrics.clone())
    }
    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>> {
        None
    }
    fn get_fix_orchestrator(
        &self,
        _dry_run: bool,
    ) -> Option<Arc<dyn LintFixOrchestratorAggregate>> {
        None
    }
    fn get_report_formatter(&self) -> Option<Box<dyn IReportFormatterProtocol>> {
        None
    }
    // AES030 orphan detection getters — return None for unimplemented
    fn get_maintenance_aggregate(&self) -> Option<Arc<dyn MaintenanceCommandsAggregate>> {
        None
    }
    fn get_analysis_protocol(&self) -> Option<Arc<dyn IAnalysisProtocol>> {
        None
    }
    fn get_code_metric_protocol(&self) -> Option<Arc<dyn ICodeMetricAnalyzerProtocol>> {
        None
    }
    fn get_target_resolver_protocol(&self) -> Option<Arc<dyn ITargetResolverProtocol>> {
        None
    }
    fn get_unused_protocol(&self) -> Option<Arc<dyn IUnusedProtocol>> {
        None
    }
    fn get_config_discovery_port(&self) -> Option<Arc<dyn IConfigDiscoveryPort>> {
        None
    }
    fn get_config_orchestration_aggregate(&self) -> Option<Arc<dyn IConfigOrchestrationAggregate>> {
        None
    }
    fn get_config_parser_port(&self) -> Option<Arc<dyn IConfigParserPort>> {
        None
    }
    fn get_config_validator_protocol(&self) -> Option<Arc<dyn IConfigValidatorProtocol>> {
        None
    }
    fn get_watch_provider_port(&self) -> Option<Arc<dyn IWatchProviderPort>> {
        None
    }
    fn get_git_commands_aggregate(&self) -> Option<Arc<dyn GitCommandsAggregate>> {
        None
    }
    fn get_git_orchestrator_aggregate(
        &self,
    ) -> Option<Arc<dyn HookManagementOrchestratorAggregate>> {
        None
    }
    fn get_import_parser_port(&self) -> Option<Arc<dyn IImportParserPort>> {
        None
    }
    fn get_javascript_flow_port(&self) -> Option<Arc<dyn IJavascriptFlowPort>> {
        None
    }
    fn get_naming_provider_port(&self) -> Option<Arc<dyn INamingProviderPort>> {
        None
    }
    fn get_javascript_scope_port(&self) -> Option<Arc<dyn IJavascriptScopePort>> {
        None
    }
    fn get_semantic_tracer_port(&self) -> Option<Arc<dyn ISemanticTracerPort>> {
        None
    }
    fn get_naming_variant_port(&self) -> Option<Arc<dyn INamingVariantPort>> {
        None
    }
    fn get_agent_lifecycle_aggregate(&self) -> Option<Arc<dyn AgentLifecycleAggregate>> {
        None
    }
    fn get_mcp_server_port(&self) -> Option<Arc<dyn IMcpServerPort>> {
        None
    }
    fn get_multi_project_aggregate(&self) -> Option<Arc<dyn MultiProjectOrchestratorAggregate>> {
        None
    }
    fn get_pipeline_extended_aggregate(
        &self,
    ) -> Option<Arc<dyn PipelineExtendedOrchestratorAggregate>> {
        None
    }
    fn get_pipeline_output_aggregate(&self) -> Option<Arc<dyn PipelineOutputAggregate>> {
        None
    }
    fn get_plugin_manager_port(&self) -> Option<Arc<dyn IPluginManagerPort>> {
        None
    }
    fn get_setup_management_aggregate(&self) -> Option<Arc<dyn SetupManagementAggregate>> {
        None
    }
    fn get_setup_management_protocol(&self) -> Option<Arc<dyn ISetupManagementProtocol>> {
        None
    }
    fn get_scanner_provider_port(&self) -> Option<Arc<dyn IScannerProviderPort>> {
        None
    }
}