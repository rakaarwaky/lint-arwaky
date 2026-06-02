"""contract — Interface definitions (ports, protocols, and aggregates)."""

# === PORTS (Infrastructure Boundaries) ===
from .linter_adapter_port import ILinterAdapterPort
from . import linter_adapter_port as linter_adapter_port
from .semantic_tracer_port import ISemanticTracerPort
from .hook_manager_port import IHookManagerPort
from .file_system_port import IFileSystemPort
from .http_provider_port import IHttpProviderPort
from .source_parser_port import ISourceParserPort
from .command_executor_port import ICommandExecutorPort
from .config_provider_port import IConfigProviderPort
from .config_validation_port import IConfigValidatorPort
from .metrics_provider_port import IMetricsProviderPort
from .plugin_manager_port import IPluginManagerPort
from .scanner_provider_port import IScannerProviderPort
from .naming_provider_port import INamingProviderPort
from .watch_provider_port import IWatchProviderPort
from .config_discovery_port import IConfigDiscoveryPort
from .config_parser_port import IConfigParserPort
from .path_normalization_port import IPathNormalizationPort
from .job_registry_port import IJobRegistryPort
from .javascript_scope_port import IJSScopeProviderPort
from .js_tracer_port import IJSScopeTracerPort
from .naming_variant_port import INamingVariantPort
from .javascript_flow_port import IJSFlowTracerPort
from .arch_compliance_port import IArchCompliancePort
from .mcp_server_port import IMcpServerPort

# === PROTOCOLS (Capability Boundaries) ===
from .semantic_tracer_protocol import ISemanticTracerProtocol
from .arch_compliance_protocol import (
    IArchComplianceProtocol,
    IScopeBoundaryProtocol,
)
from .arch_orphan_protocol import (
    IArchOrphanProtocol,
    IOrphanGraphProtocol,
    IOrphanIndicatorProtocol,
)
from .naming_variant_protocol import INamingVariantProtocol
from .data_flow_protocol import IDataFlowProtocol
from .unused_import_protocol import IUnusedImportProtocol
from .scope_boundary_protocol import IScopeBoundaryResolverProtocol
from .domain_type_protocol import IDomainTypeProtocol
from .dispatch_routing_protocol import (
    IDispatchRoutingProtocol,
    IDispatchRoutingParserProtocol,
)
from .code_transformation_protocol import ISymbolRenamerProtocol
from .lint_reporting_protocol import ILintReportFormatterProtocol
from .project_governance_protocol import (
    IMetricAnalyzerProtocol,
    IConfigRulesProtocol,
    IArchRuleEngineProtocol,
)
from .arch_rule_protocol import (
    IArchRuleProtocol,
    INamingRuleProtocol,
    ICodeQualityProtocol,
    IArchStructureProtocol,
    IArchImportProtocol,
    INamingCheckerProtocol,
    IInternalCheckerProtocol,
    IMetricCheckerProtocol,
    IRoleCheckerProtocol,
    IArchImportProcessorProtocol,
)
from .cycle_analysis_protocol import ICycleAnalysisProtocol
from .setup_management_protocol import ISetupManagementProtocol
from .import_violation_protocol import IImportViolationProtocol
from .arch_analyzer_protocol import IArchAnalyzerProtocol
from .arch_rule_protocol import IAnalyzer

# === AGGREGATES (Domain Contracts) ===
from .service_container_aggregate import ServiceContainerAggregate
from .infrastructure_container_aggregate import InfrastructureContainerAggregate
from .capability_container_aggregate import CapabilityContainerAggregate
from .adapter_container_aggregate import AdapterContainerAggregate
from .orchestrator_container_aggregate import OrchestratorContainerAggregate
from .project_container_aggregate import ProjectContainerAggregate
from .container_registry_aggregate import ContainerRegistryAggregate
from .job_registry_aggregate import JobRegistryAggregate
from .job_registry_aggregate import JobRegistryAggregate as BackgroundJobAggregate
from .diff_result_aggregate import GitDiffResultAggregate
from .pipeline_input_aggregate import PipelineInputAggregate
from .pipeline_output_aggregate import PipelineOutputAggregate
from .analysis_orchestrator_aggregate import AnalysisOrchestratorAggregate
from .pipeline_orchestrator_aggregate import LintPipelineOrchestratorAggregate
from .fix_orchestrator_aggregate import LintFixOrchestratorAggregate
from .execution_orchestrator_aggregate import PipelineExecutionOrchestratorAggregate
from .project_orchestrator_aggregate import MultiProjectOrchestratorAggregate
from .watch_orchestrator_aggregate import WatchExecutionOrchestratorAggregate
from .hook_orchestrator_aggregate import HookManagementOrchestratorAggregate
from .pipeline_extended_aggregate import PipelineExtendedOrchestratorAggregate
from .pipeline_dispatcher_aggregate import PipelineActionDispatcherAggregate
from .agent_lifecycle_aggregate import AgentLifecycleAggregate
from .setup_management_aggregate import SetupManagementAggregate
from .multi_project_aggregate import MultiProjectAggregate
from .directory_watch_aggregate import DirectoryWatchAggregate
from .output_client_aggregate import OutputClientAggregate
from .arch_coordinator_aggregate import ArchCoordinatorAggregate

from .architecture_orchestrator_aggregate import ArchitectureOrchestratorAggregate
from .async_bridge_aggregate import AsyncBridgeAggregate, run_async

# === COMMAND AGGREGATES (Surface Contracts) ===
from .check_commands_aggregate import CheckCommandsAggregate
from .fix_commands_aggregate import FixCommandsAggregate
from .dev_commands_aggregate import DevCommandsAggregate
from .git_commands_aggregate import GitCommandsAggregate
from .maintenance_commands_aggregate import MaintenanceCommandsAggregate
# DispatchCommandsAggregate might be needed if it exists
from .plugin_commands_aggregate import PluginCommandsAggregate
from .report_commands_aggregate import ReportCommandsAggregate
from .watch_commands_aggregate import WatchCommandsAggregate
# Backward-compatible alias after refactor
ArchComplianceCoordinatorAggregate = ArchCoordinatorAggregate


__all__ = [
    # Ports
    "ILinterAdapterPort", "ISemanticTracerPort", "IHookManagerPort", "IFileSystemPort",
    "IHttpProviderPort", "ISourceParserPort", "ICommandExecutorPort", "IConfigProviderPort",
    "IConfigValidatorPort", "IMetricsProviderPort", "IPluginManagerPort", "IScannerProviderPort",
    "INamingProviderPort", "IWatchProviderPort", "IConfigDiscoveryPort", "IConfigParserPort",
    "IPathNormalizationPort", "IJobRegistryPort", "IJSScopeProviderPort", "IJSScopeTracerPort",
    "INamingVariantPort", "IJSFlowTracerPort", "IArchCompliancePort", "IMcpServerPort",

    # Protocols
    "ISemanticTracerProtocol", "IArchAnalyzerProtocol", "IArchComplianceProtocol",
    "IArchOrphanProtocol", "IScopeBoundaryProtocol", "INamingVariantProtocol",
    "IDataFlowProtocol", "IUnusedImportProtocol", "IScopeBoundaryResolverProtocol",
    "IDomainTypeProtocol", "ISymbolRenamerProtocol", "ILintReportFormatterProtocol",
    "IMetricAnalyzerProtocol", "IConfigRulesProtocol", "IArchRuleEngineProtocol",
    "IArchRuleProtocol", "INamingRuleProtocol", "ICodeQualityProtocol",
    "IArchStructureProtocol", "IArchImportProtocol", "INamingCheckerProtocol",
    "IInternalCheckerProtocol", "IMetricCheckerProtocol", "IRoleCheckerProtocol",
    "IArchImportProcessorProtocol", "IOrphanGraphProtocol", "IOrphanIndicatorProtocol",
    "ICycleAnalysisProtocol", "ISetupManagementProtocol", "IImportViolationProtocol",
    "IDispatchRoutingProtocol", "IDispatchRoutingParserProtocol", "IAnalyzer",

    # Aggregates
    "ServiceContainerAggregate", "InfrastructureContainerAggregate", "CapabilityContainerAggregate",
    "AdapterContainerAggregate", "OrchestratorContainerAggregate", "ProjectContainerAggregate",
    "ContainerRegistryAggregate", "JobRegistryAggregate", "GitDiffResultAggregate",
    "PipelineInputAggregate", "PipelineOutputAggregate", "AnalysisOrchestratorAggregate",
    "LintPipelineOrchestratorAggregate", "LintFixOrchestratorAggregate",
    "PipelineExecutionOrchestratorAggregate", "MultiProjectOrchestratorAggregate",
    "WatchExecutionOrchestratorAggregate", "HookManagementOrchestratorAggregate",
    "PipelineExtendedOrchestratorAggregate", "PipelineActionDispatcherAggregate",
    "AgentLifecycleAggregate", "SetupManagementAggregate", "MultiProjectAggregate",
    "DirectoryWatchAggregate", "OutputClientAggregate", "ArchCoordinatorAggregate",
    "ArchitectureOrchestratorAggregate", "AsyncBridgeAggregate", "run_async",
    "ArchComplianceCoordinatorAggregate",

    # Commands
    "CheckCommandsAggregate", "FixCommandsAggregate", "DevCommandsAggregate",
    "GitCommandsAggregate", "MaintenanceCommandsAggregate", "PluginCommandsAggregate",
    "ReportCommandsAggregate", "WatchCommandsAggregate",
    "BackgroundJobAggregate",
]
