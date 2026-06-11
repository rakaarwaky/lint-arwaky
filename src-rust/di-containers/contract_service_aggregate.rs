// PURPOSE: ServiceContainerAggregate — top-level DI aggregate bundling all container aggregates
use crate::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
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
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::file_watch::contract_provider_port::IWatchProviderPort;
use crate::git_hooks::contract_commands_aggregate::GitCommandsAggregate;
use crate::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use crate::language_adapters::contract_flow_port::IJavascriptFlowPort;
use crate::language_adapters::contract_naming_port::INamingProviderPort;
use crate::language_adapters::contract_scope_port::IJavascriptScopePort;
use crate::language_adapters::contract_semantic_tracer_port::ISemanticTracerPort;
use crate::language_adapters::contract_variant_port::INamingVariantPort;
use crate::layer_rules::contract_import_parser_port::IImportParserPort;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
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
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use std::sync::Arc;

pub trait ServiceContainerAggregate: Send + Sync {
    fn file_system(&self) -> Arc<dyn IFileSystemPort> {
        panic!("file_system not implemented in this container")
    }
    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort> {
        panic!("command_executor not implemented in this container")
    }
    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        panic!("path_normalization not implemented in this container")
    }
    fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        panic!("source_parser not implemented in this container")
    }
    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort> {
        panic!("command_executor not implemented in this container")
    }
    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        panic!("path_normalization not implemented in this container")
    }
    fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        panic!("source_parser not implemented in this container")
    }
    fn linter_adapter(&self, _name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>> {
        None
    }
    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>> {
        None
    }
    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>> {
        None
    }
    fn metrics_provider(&self) -> Option<Arc<dyn IMetricsProviderPort>> {
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
    // AES030 orphan detection getters
    fn get_maintenance_aggregate(&self) -> Arc<dyn MaintenanceCommandsAggregate> {
        panic!("get_maintenance_aggregate not implemented in this container")
    }
    fn get_analysis_protocol(&self) -> Arc<dyn IAnalysisProtocol> {
        panic!("get_analysis_protocol not implemented in this container")
    }
    fn get_code_metric_protocol(&self) -> Arc<dyn ICodeMetricAnalyzerProtocol> {
        panic!("get_code_metric_protocol not implemented in this container")
    }
    fn get_target_resolver_protocol(&self) -> Arc<dyn ITargetResolverProtocol> {
        panic!("get_target_resolver_protocol not implemented in this container")
    }
    fn get_unused_protocol(&self) -> Arc<dyn IUnusedProtocol> {
        panic!("get_unused_protocol not implemented in this container")
    }
    fn get_config_discovery_port(&self) -> Arc<dyn IConfigDiscoveryPort> {
        panic!("get_config_discovery_port not implemented in this container")
    }
    fn get_config_orchestration_aggregate(&self) -> Arc<dyn IConfigOrchestrationAggregate> {
        panic!("get_config_orchestration_aggregate not implemented in this container")
    }
    fn get_config_parser_port(&self) -> Arc<dyn IConfigParserPort> {
        panic!("get_config_parser_port not implemented in this container")
    }
    fn get_config_validator_protocol(&self) -> Arc<dyn IConfigValidatorProtocol> {
        panic!("get_config_validator_protocol not implemented in this container")
    }
    fn get_watch_provider_port(&self) -> Arc<dyn IWatchProviderPort> {
        panic!("get_watch_provider_port not implemented in this container")
    }
    fn get_git_commands_aggregate(&self) -> Arc<dyn GitCommandsAggregate> {
        panic!("get_git_commands_aggregate not implemented in this container")
    }
    fn get_git_orchestrator_aggregate(&self) -> Arc<dyn HookManagementOrchestratorAggregate> {
        panic!("get_git_orchestrator_aggregate not implemented in this container")
    }
    fn get_import_parser_port(&self) -> Arc<dyn IImportParserPort> {
        panic!("get_import_parser_port not implemented in this container")
    }
    fn get_javascript_flow_port(&self) -> Arc<dyn IJavascriptFlowPort> {
        panic!("get_javascript_flow_port not implemented in this container")
    }
    fn get_naming_provider_port(&self) -> Arc<dyn INamingProviderPort> {
        panic!("get_naming_provider_port not implemented in this container")
    }
    fn get_javascript_scope_port(&self) -> Arc<dyn IJavascriptScopePort> {
        panic!("get_javascript_scope_port not implemented in this container")
    }
    fn get_semantic_tracer_port(&self) -> Arc<dyn ISemanticTracerPort> {
        panic!("get_semantic_tracer_port not implemented in this container")
    }
    fn get_naming_variant_port(&self) -> Arc<dyn INamingVariantPort> {
        panic!("get_naming_variant_port not implemented in this container")
    }
    fn get_agent_lifecycle_aggregate(&self) -> Arc<dyn AgentLifecycleAggregate> {
        panic!("get_agent_lifecycle_aggregate not implemented in this container")
    }
    fn get_mcp_server_port(&self) -> Arc<dyn IMcpServerPort> {
        panic!("get_mcp_server_port not implemented in this container")
    }
    fn get_multi_project_aggregate(&self) -> Arc<dyn MultiProjectOrchestratorAggregate> {
        panic!("get_multi_project_aggregate not implemented in this container")
    }
    fn get_pipeline_extended_aggregate(&self) -> Arc<dyn PipelineExtendedOrchestratorAggregate> {
        panic!("get_pipeline_extended_aggregate not implemented in this container")
    }
    fn get_pipeline_output_aggregate(&self) -> Arc<dyn PipelineOutputAggregate> {
        panic!("get_pipeline_output_aggregate not implemented in this container")
    }
    fn get_plugin_manager_port(&self) -> Arc<dyn IPluginManagerPort> {
        panic!("get_plugin_manager_port not implemented in this container")
    }
    fn get_setup_management_aggregate(&self) -> Arc<dyn SetupManagementAggregate> {
        panic!("get_setup_management_aggregate not implemented in this container")
    }
    fn get_setup_management_protocol(&self) -> Arc<dyn ISetupManagementProtocol> {
        panic!("get_setup_management_protocol not implemented in this container")
    }
    fn get_scanner_provider_port(&self) -> Arc<dyn IScannerProviderPort> {
        panic!("get_scanner_provider_port not implemented in this container")
    }
}
