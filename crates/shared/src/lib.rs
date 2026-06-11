// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

#[path = "source-parsing/mod.rs"]
pub mod source_parsing;

// Explicit re-exports for commonly used types (no wildcards)
// All from common module
pub use common::taxonomy_source_vo::{ContentString, SourceContentVO, ConfigResult, ConfigSource};
pub use common::taxonomy_path_vo::{DirectoryPath, FilePath};
pub use common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
pub use common::taxonomy_parser_error::SourceParserError;
pub use common::taxonomy_filesystem_error::{AccessDeniedError, FileSystemError, PathNotFoundError};
pub use common::taxonomy_import_source_vo::{ImportInfo, ImportInfoList, PrimitiveViolation, PrimitiveViolationList};
pub use common::taxonomy_naming_list_vo::{primitive_type_list, CallChainList, ImportNameList, PrimitiveTypeList, SymbolNameList};
pub use common::taxonomy_job_vo::{AdapterMetadata, EnvContentVO, JobStatus, LintStatusActionArgs, McpConfigVO, ResponseData, SuccessStatus};

pub use common::taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, ErrorMessage, JobIdList, LineContentList,
    LineNumber, PatternList, Score, Timestamp,
};
pub use common::taxonomy_common_error::{
    Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName,
};
pub use common::taxonomy_error_vo::ErrorCode;
pub use common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
pub use common::taxonomy_lint_vo::{
    CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint,
};
pub use common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
pub use common::taxonomy_name_vo::{NameVariants, SymbolName};
pub use common::taxonomy_suggestion_vo::{
    ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion,
};
pub use common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};
pub use common::taxonomy_adapter_name_vo::AdapterName;
pub use common::taxonomy_duration_vo::{Duration, Timeout};
pub use common::taxonomy_severity_vo::Severity;
pub use common::taxonomy_result_vo::{LintResult, LintResultList};
pub use common::taxonomy_score_vo::{compute_score, FileFormat};
pub use common::taxonomy_score_constant::{FORMAT_JSON, FORMAT_JUNIT, FORMAT_SARIF, FORMAT_TEXT};
pub use common::taxonomy_position_vo::Position;
pub use common::taxonomy_config_vo::{default_aes_config, default_config_for_language, ArchitectureConfig};
pub use common::taxonomy_setting_vo::{
    ActualValue, AdapterEntry, AdapterStatus, ExpectedValue, ProjectConfig, Thresholds,
};
pub use common::taxonomy_validation_vo::ValidationResult;
pub use common::taxonomy_identifier_vo::ConfigKey;
pub use common::taxonomy_config_error::ConfigError;
pub use common::taxonomy_adapter_vo::{AdapterClassMap, AdapterMetadataList, AdapterNameList};
pub use common::taxonomy_app_vo::AppConfig;
pub use common::taxonomy_rule_vo::{
    ArchitectureRule, CustomMessageVO, LegacyLayerRule, LegacyLayerRuleList, MandatoryImportRuleVO,
};
pub use common::taxonomy_suffix_vo::{SuffixPolicyVO, SuffixVO};
pub use common::taxonomy_doctor_vo::DoctorResultVO;
pub use common::taxonomy_language_vo::{LanguageSource, ProjectLanguage};
pub use common::taxonomy_stats_vo::MaintenanceStatsVO;
pub use common::taxonomy_operation_error::LinterOperationError;
pub use common::taxonomy_governance_entity::ArchitectureGovernanceEntity;
pub use common::taxonomy_analysis_vo::{
    FileDefinitionMap, FilePathSet, GraphAnalysisContext, ImportGraph, InboundLinkMap,
    InheritanceMap, ModuleToFileMap, OrphanIndicatorResult, ReachabilityResult,
};
pub use common::taxonomy_violation_message_rs_error::AesViolationRs;
pub use common::taxonomy_violation_message_py_error::AesViolationPy;
pub use common::taxonomy_violation_message_js_error::AesViolationJs;
pub use common::taxonomy_fix_vo::FixResult;
pub use common::taxonomy_fix_applied_event::FixApplied;
pub use common::taxonomy_symbol_renamer_utility::SymbolRenamer;
pub use common::taxonomy_multi_project_vo::MultiProjectVO;
pub use common::taxonomy_summary_vo::{AggregatedResults, ProjectResult};
pub use common::taxonomy_agent_status_vo::{AgentStatus, AgentStatusVO};
pub use common::taxonomy_watch_vo::DirectoryWatchVO;
pub use common::taxonomy_service_error::{WatchEventError, WatchServiceError, WatchSubscriptionError};
pub use common::taxonomy_hook_error::GitHookError;
pub use common::taxonomy_installed_event::HookInstalled;
pub use common::taxonomy_removed_event::HookRemoved;
pub use common::taxonomy_diff_result_vo::GitDiffResultVO;
pub use common::taxonomy_transport_error::TransportError;
pub use common::taxonomy_protocol_vo::{TransportProtocol, TransportUrlVO};
pub use common::taxonomy_metadata_vo::CommandMetadataVO;
pub use common::taxonomy_catalog_constant::{command_catalog, CommandCatalogVO};
pub use common::taxonomy_group_vo::PluginGroup;
pub use common::taxonomy_manager_error::{DiscoveryError, PluginError, RegistrationError};
pub use common::taxonomy_server_constant::{DEFAULT_HOST, DEFAULT_PORT, HEALTH_PATH};
pub use common::taxonomy_metrics_error::MetricsError;
pub use common::taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_GLOBAL, LAYER_INFRASTRUCTURE,
    LAYER_ROOT, LAYER_SURFACES, LAYER_TAXONOMY,
};
pub use common::taxonomy_layer_names_vo::{
    layer_agent, layer_capabilities, layer_contract, layer_global, layer_infrastructure,
    layer_root, layer_surfaces, layer_taxonomy, LayerNames,
};

// Contract ports from common
pub use common::contract_parser_port::ISourceParserPort;
pub use common::contract_path_normalization_port::IPathNormalizationPort;
pub use common::contract_scanner_provider_port::IScannerProviderPort;
pub use common::contract_system_port::IFileSystemPort;
pub use common::contract_detector_port::ILanguageDetectorPort;
pub use common::contract_reader_port::IConfigReaderPort;
pub use common::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
pub use common::contract_validator_protocol::IConfigValidatorProtocol;
pub use common::contract_linter_adapter_port::ILinterAdapterPort;
pub use common::contract_fix_protocol::IFixProtocol;
pub use common::contract_fix_aggregate::ILintFixOrchestratorAggregate;
pub use common::contract_import_parser_port::IImportParserPort;
pub use common::contract_import_runner_aggregate::IImportRunnerAggregate;
pub use common::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
pub use common::contract_naming_port::INamingProviderPort;
pub use common::contract_variant_port::INamingVariantPort;
pub use common::contract_flow_port::IJavascriptFlowPort;
pub use common::contract_scope_port::{IJavascriptScopePort, IJsTracerPort};
pub use common::contract_watch_provider_port::IWatchProviderPort;
pub use common::contract_hook_manager_port::IHookManagerPort;
pub use common::contract_mcp_server_port::IMcpServerPort;
pub use common::contract_metrics_port::IMetricsProviderPort;
pub use common::contract_plugin_manager_port::IPluginManagerPort;
pub use common::contract_setup_protocol::ISetupManagementProtocol;
pub use common::contract_setup_aggregate::ISetupManagementAggregate;
pub use common::contract_command_executor_port::ICommandExecutorPort;
pub use common::contract_job_registry_port::IJobRegistryPort;
pub use common::contract_dispatcher_aggregate::IActionDispatcherAggregate;
pub use common::contract_extended_aggregate::IExtendedOrchestratorAggregate;
pub use common::contract_output_aggregate::IOutputAggregate;
pub use common::contract_client_aggregate::IClientAggregate;
pub use common::contract_report_aggregate::IReportAggregate;
pub use common::contract_fix_runner_aggregate::IFixRunnerAggregate;
pub use common::contract_layer_detection_aggregate::ILayerDetectionAggregate;
pub use common::contract_lint_protocol::IArchLintProtocol;
pub use common::contract_analysis_protocol::IAnalysisProtocol;
pub use common::contract_class_protocol::IMandatoryClassProtocol;
pub use common::contract_line_protocol::ILineCheckerProtocol;
pub use common::contract_bypass_checker_protocol::IBypassCheckerProtocol;
pub use common::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
pub use common::contract_inline_unused_protocol::IInlineUnusedProtocol;
pub use common::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
pub use common::contract_cycle_protocol::ICycleAnalysisProtocol;
pub use common::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
pub use common::contract_target_resolver_protocol::ITargetResolverProtocol;
pub use common::contract_unused_protocol::IUnusedProtocol;
pub use common::contract_agent_role_protocol::IAgentRoleChecker;
pub use common::contract_role_protocol::IContractRoleChecker;
pub use common::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
pub use common::contract_surface_role_protocol::ISurfaceRoleChecker;
pub use common::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
pub use common::contract_role_aggregate::IRoleAggregate;
pub use common::contract_role_runner_aggregate::IRoleRunnerAggregate;