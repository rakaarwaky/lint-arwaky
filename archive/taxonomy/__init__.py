"""taxonomy — Shared language for all domains (VOs, entities, errors, events)."""

__all__ = [
    # Value Objects: Core
    "Severity",
    "ErrorCode",
    "Position",
    "Score",
    "FileFormat",
    "ComplianceStatus",
    "LineNumber",
    "ColumnNumber",
    "LintMessage",
    "Count",
    "FORMAT_TEXT",
    "FORMAT_JSON",
    "FORMAT_SARIF",
    "FORMAT_JUNIT",
    "ALL_FORMATS",
    "SuccessStatus",
    "BooleanVO",
    "ErrorMessage",
    "Suggestion",
    "LogOutput",
    "StdOutput",
    "StdError",
    "Timestamp",
    "Duration",
    "Timeout",
    "ExitCode",
    "ExpectedValue",
    "ActualValue",
    "FieldName",
    "Constraint",
    "Cause",
    "MetadataVO",
    "ClassPath",
    "DescriptionVO",
    "ModuleName",
    "PrimitiveTypeName",
    "ContentString",
    # Value Objects: Identifiers
    "AdapterName",
    "FilePath",
    "FilePathList",
    "FilePathSet",
    "SymbolName",
    "NameVariants",
    "DirectoryPath",
    "GitRef",
    "LayerNameVO",
    "Identity",
    "JobId",
    "ActionName",
    "ActionArgs",
    "VALID_PIPELINE_ACTIONS",
    "FileContentVO",
    "PluginGroup",
    "LineContentVO",
    "AdapterMetadata",
    "AdapterMetadataList",
    "CallChainList",
    "DataFlowList",
    "LintResultList",
    "SymbolNameList",
    "ImportNameList",
    "AdapterNameList",
    "LineContentList",
    "AdapterClassMap",
    "EnvContentVO",
    "TransportUrlVO",
    "McpConfigVO",
    "SuffixVO",
    "SuffixPolicyVO",
    # Value Objects: Domain
    "ScopeRef",
    "Location",
    "LocationList",
    "CommandArgs",
    "ScopeBounds",
    # Value Objects: Transport
    "TransportProtocol",
    "TransportEndpoint",
    # Value Objects: Config
    "Thresholds",
    "AdapterStatus",
    "AdapterEntry",
    "ProjectConfig",
    "DEFAULT_THRESHOLDS",
    "ConfigKey",
    "ArchitectureConfig",
    "LayerMapVO",
    "CustomMessageVO",
    "MandatoryImportRuleVO",
    "ArchitectureRule",
    "LegacyLayerRule",
    "LegacyLayerRuleList",
    "LayerDefinition",
    "NamingConfig",
    "ImportInfo",
    "ImportInfoList",
    "PrimitiveViolation",
    "PrimitiveViolationList",
    "ImportGraph",
    "InboundLinkMap",
    "InheritanceMap",
    "FileDefinitionMap",
    "ReachabilityResult",
    "ModuleToFileMap",
    "GraphAnalysisContext",
    "OrphanIndicatorResult",
    "CapabilityReference",
    "CapabilityReferenceList",
    "ClassMethodsVO",
    "ClassDefinitionMap",
    "ClassFileMap",
    "CapabilityRoutingContext",
    "ClassUsageMap",
    "ClassUsageItem",
    "ClassUsageItemList",
    "ClassNameVO",
    # Note: BraceDepthVO, MethodArgsVO, ScopeNameVO, IndentSizeVO, ClassParsingStateVO
    # have been moved to capabilities/dispatch_parser_types.py (parser implementation state)
    "ProjectResult",
    "AggregatedResults",
    "GitDiffResultVO",
    "AgentStatus",
    "AgentStatusVO",
    "CommandMetadataVO",
    # Entities
    "JobStatus",
    "LintResult",
    "LintResultList",
    "GovernanceReport",
    "FixResult",
    "WatchResult",
    "AppConfig",
    "MaintenanceStatsVO",
    "DoctorResultVO",
    "FileSystemError",
    "SourceParserError",
    "WatchServiceError",
    "SemanticError",
    "PluginError",
    "MetricsError",
    "NamingError",
    "GitHookError",
    # Constants: Layer Names
    "LAYER_AGENT",
    "LAYER_CAPABILITIES",
    "LAYER_TAXONOMY",
    "LAYER_CONTRACT",
    "LAYER_INFRASTRUCTURE",
    "LAYER_SURFACES",
    "LAYER_ROOT",
    "LAYER_GLOBAL",
    "CORE_LAYER_NAMES",
    "CORE_PRIMITIVE_TYPES",
    "PRIMITIVE_TYPE_LIST",
    # Note: MCP_SERVER_VERSION, MCP_PROTOCOL_MIN, MCP_PROTOCOL_MAX, AUTO_LINT_VERSION,
    # MAX_STRING_LENGTH, MAX_PATH_LENGTH, MAX_BATCH_SIZE, MAX_PATH_DEPTH
    # have been moved to infrastructure/mcp_server_constants.py (technical protocol constants)
    "PatternList",
    "PrimitiveTypeList",
    "RenamedFile",
    "RenamedFileList",
    "ResponseData",
    "ResponseDataList",
    "AdapterError",
    "ScanError",
]

# Note: MCP version and bounds constants moved to infrastructure/mcp_server_constants.py

# -- Value Objects: Core --
from .lint_severity_vo import Severity as Severity
from .error_code_vo import ErrorCode as ErrorCode
from .lint_position_vo import (
    Position as Position,
    LineNumber as LineNumber,
    ColumnNumber as ColumnNumber,
)
from .score_format_vo import (
    Score as Score,
    FileFormat as FileFormat,
    FORMAT_TEXT as FORMAT_TEXT,
    FORMAT_JSON as FORMAT_JSON,
    FORMAT_SARIF as FORMAT_SARIF,
    FORMAT_JUNIT as FORMAT_JUNIT,
    ALL_FORMATS as ALL_FORMATS,
)
from .message_status_vo import (
    LintMessage as LintMessage,
    ComplianceStatus as ComplianceStatus,
    Count as Count,
)
from .time_duration_vo import (
    Duration as Duration,
    Timestamp as Timestamp,
    Timeout as Timeout,
)
from .error_value_vo import (
    ErrorMessage as ErrorMessage,
    ExitCode as ExitCode,
    ExpectedValue as ExpectedValue,
    ActualValue as ActualValue,
    FieldName as FieldName,
    Constraint as Constraint,
    Cause as Cause,
    ModuleName as ModuleName,
    PrimitiveTypeName as PrimitiveTypeName,
)
from .log_suggestion_vo import (
    BooleanVO as BooleanVO,
    LogOutput as LogOutput,
    Suggestion as Suggestion,
    StdOutput as StdOutput,
    StdError as StdError,
    MetadataVO as MetadataVO,
    ClassPath as ClassPath,
    DescriptionVO as DescriptionVO,
)
from .content_string_vo import ContentString as ContentString

# -- Value Objects: Identifiers --
from .adapter_name_vo import AdapterName as AdapterName
from .file_path_vo import (
    FilePath as FilePath,
    DirectoryPath as DirectoryPath,
)
from .symbol_name_vo import (
    SymbolName as SymbolName,
    NameVariants as NameVariants,
)
from .git_ref_vo import GitRef as GitRef
from .job_action_vo import (
    JobId as JobId,
    ActionName as ActionName,
    VALID_PIPELINE_ACTIONS as VALID_PIPELINE_ACTIONS,
)
from .layer_content_vo import (
    LayerNameVO as LayerNameVO,
    FileContentVO as FileContentVO,
    LineContentVO as LineContentVO,
    Identity as Identity,
)
from .layer_names_vo import (
    LAYER_AGENT as LAYER_AGENT,
    LAYER_CAPABILITIES as LAYER_CAPABILITIES,
    LAYER_TAXONOMY as LAYER_TAXONOMY,
    LAYER_CONTRACT as LAYER_CONTRACT,
    LAYER_INFRASTRUCTURE as LAYER_INFRASTRUCTURE,
    LAYER_SURFACES as LAYER_SURFACES,
    LAYER_ROOT as LAYER_ROOT,
    LAYER_GLOBAL as LAYER_GLOBAL,
    CORE_LAYER_NAMES as CORE_LAYER_NAMES,
)
from .plugin_group_vo import PluginGroup as PluginGroup

# -- Value Objects: Status & Metadata --
from .lint_status_vo import (
    JobStatus as JobStatus,
    SuccessStatus as SuccessStatus,
    ActionArgs as ActionArgs,
    ResponseData as ResponseData,
    AdapterMetadata as AdapterMetadata,
    EnvContentVO as EnvContentVO,
    TransportUrlVO as TransportUrlVO,
    McpConfigVO as McpConfigVO,
)

# -- Value Objects: Collections --
from .path_collection_vo import (
    FilePathList as FilePathList,
    FilePathSet as FilePathSet,
    RenamedFile as RenamedFile,
    RenamedFileList as RenamedFileList,
    PatternList as PatternList,
)
from .symbol_collection_vo import (
    PrimitiveTypeList as PrimitiveTypeList,
    SymbolNameList as SymbolNameList,
    ImportNameList as ImportNameList,
    CallChainList as CallChainList,
    CORE_PRIMITIVE_TYPES as CORE_PRIMITIVE_TYPES,
    PRIMITIVE_TYPE_LIST as PRIMITIVE_TYPE_LIST,
)
from .adapter_collection_vo import (
    AdapterMetadataList as AdapterMetadataList,
    AdapterNameList as AdapterNameList,
    AdapterClassMap as AdapterClassMap,
)
from .generic_collection_vo import (
    JobIdList as JobIdList,
    DataFlowList as DataFlowList,
    ResponseDataList as ResponseDataList,
    LineContentList as LineContentList,
)

# -- Value Objects: Domain --
from .lint_domain_vo import (
    ScopeRef as ScopeRef,
    Location as Location,
    LocationList as LocationList,
    CommandArgs as CommandArgs,
    ScopeBounds as ScopeBounds,
)

# -- Value Objects: Transport --
from .transport_protocol_vo import (
    TransportProtocol as TransportProtocol,
    TransportEndpoint as TransportEndpoint,
)

# -- Value Objects: Config --
from .config_setting_vo import (
    Thresholds as Thresholds,
    AdapterStatus as AdapterStatus,
    AdapterEntry as AdapterEntry,
    ProjectConfig as ProjectConfig,
    DEFAULT_THRESHOLDS as DEFAULT_THRESHOLDS,
)

from .file_suffix_vo import (
    SuffixVO as SuffixVO,
    SuffixPolicyVO as SuffixPolicyVO,
)

from .architecture_config_vo import (
    ArchitectureConfig as ArchitectureConfig,
)

from .architecture_rule_vo import (
    ArchitectureRule as ArchitectureRule,
    CustomMessageVO as CustomMessageVO,
    LegacyLayerRule as LegacyLayerRule,
    LegacyLayerRuleList as LegacyLayerRuleList,
    MandatoryImportRuleVO as MandatoryImportRuleVO,
)

from .config_identifier_vo import (
    ConfigKey as ConfigKey,
)

from .project_summary_vo import (
    ProjectResult as ProjectResult,
    AggregatedResults as AggregatedResults,
)

from .git_diff_vo import (
    GitDiffResultVO as GitDiffResultVO,
)

# -- Value Objects: Source Analysis --
from .source_analysis_vo import (
    ImportInfo as ImportInfo,
    ImportInfoList as ImportInfoList,
    PrimitiveViolation as PrimitiveViolation,
    PrimitiveViolationList as PrimitiveViolationList,
)

from .fix_result_vo import FixResult as FixResult
from .watch_result_vo import WatchResult as WatchResult
from .app_config_vo import AppConfig as AppConfig
from .maintenance_stats_vo import MaintenanceStatsVO as MaintenanceStatsVO
from .doctor_result_vo import DoctorResultVO as DoctorResultVO

# -- Value Objects: Architecture --
from .layer_definition_vo import (
    LayerDefinition as LayerDefinition,
    NamingConfig as NamingConfig,
    LayerMapVO as LayerMapVO,
)

# -- Value Objects: Architecture Analysis --
from .architecture_analysis_vo import (
    ImportGraph as ImportGraph,
    InboundLinkMap as InboundLinkMap,
    InheritanceMap as InheritanceMap,
    FileDefinitionMap as FileDefinitionMap,
    ReachabilityResult as ReachabilityResult,
    ModuleToFileMap as ModuleToFileMap,
    GraphAnalysisContext as GraphAnalysisContext,
    OrphanIndicatorResult as OrphanIndicatorResult,
)

from .capability_routing_vo import (
    CapabilityReference as CapabilityReference,
    CapabilityReferenceList as CapabilityReferenceList,
    ClassMethodsVO as ClassMethodsVO,
    ClassDefinitionMap as ClassDefinitionMap,
    ClassFileMap as ClassFileMap,
    CapabilityRoutingContext as CapabilityRoutingContext,
    ClassUsageMap as ClassUsageMap,
    ClassUsageItem as ClassUsageItem,
    ClassUsageItemList as ClassUsageItemList,
    ClassNameVO as ClassNameVO,
    # BraceDepthVO, MethodArgsVO, ScopeNameVO, IndentSizeVO, ClassParsingStateVO
    # moved to capabilities/dispatch_parser_types.py
)

# -- Entities --
from .governance_report_entity import (
    GovernanceReport as GovernanceReport,
)
from .lint_result_vo import (
    LintResult as LintResult,
    LintResultList as LintResultList,
)

from .agent_status_vo import (
    AgentStatus as AgentStatus,
    AgentStatusVO as AgentStatusVO,
)

from .command_metadata_vo import (
    CommandMetadataVO as CommandMetadataVO,
)

# -- Errors --
from .job_registry_error import (
    JobError as JobError,
)

from .lint_adapter_error import (
    AdapterError as AdapterError,
    ScanError as ScanError,
    ValidationError as ValidationError,
)

from .transport_client_error import (
    TransportError as TransportError,
)

from .config_provider_error import (
    ConfigError as ConfigError,
)

from .file_system_error import (
    FileSystemError as FileSystemError,
    PathNotFoundError as PathNotFoundError,
    AccessDeniedError as AccessDeniedError,
)

from .source_parser_error import (
    SourceParserError as SourceParserError,
    SyntaxErrorVO as SyntaxErrorVO,
)

from .watch_service_error import (
    WatchServiceError as WatchServiceError,
    WatchSubscriptionError as WatchSubscriptionError,
    WatchEventError as WatchEventError,
)

from .semantic_tracer_error import (
    SemanticError as SemanticError,
    ScopeResolutionError as ScopeResolutionError,
    CallChainError as CallChainError,
)

from .plugin_manager_error import (
    PluginError as PluginError,
    DiscoveryError as DiscoveryError,
    RegistrationError as RegistrationError,
)

from .metrics_provider_error import (
    MetricsError as MetricsError,
)

from .naming_provider_error import (
    NamingError as NamingError,
)

from .git_hook_error import (
    GitHookError as GitHookError,
)

# -- Events --
from .lint_scan_event import (
    ScanStarted as ScanStarted,
    ScanCompleted as ScanCompleted,
    ScanFailed as ScanFailed,
    FixApplied as FixApplied,
    AdapterRegistered as AdapterRegistered,
    HookInstalled as HookInstalled,
    HookRemoved as HookRemoved,
)
