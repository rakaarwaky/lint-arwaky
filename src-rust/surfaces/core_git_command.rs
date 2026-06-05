use std::sync::Arc;
/// Git-related CLI commands for auto-linter.
use std::collections::HashMap;

use crate::taxonomy::{AccessDeniedError,
ActionArgs,
ActionName,
ActualValue,
AdapterClassMap,
AdapterEntry,
AdapterError,
AdapterMetadata,
AdapterMetadataList,
AdapterName,
AdapterNameList,
AdapterRegistered,
AdapterStatus,
AgentStatus,
AgentStatusVO};

use crate::taxonomy::{AggregatedResults,
AppConfig,
ArchitectureConfig,
ArchitectureRule,
BooleanVO,
CallChainError,
CallChainList,
CapabilityReference,
CapabilityReferenceList,
CapabilityRoutingContext,
Cause,
ClassDefinitionMap,
ClassFileMap,
ClassMethodsVO,
ClassNameVO};

use crate::taxonomy::{ClassPath,
ClassUsageItem,
ClassUsageItemList,
ClassUsageMap,
ColumnNumber,
CommandArgs,
CommandMetadataVO,
ComplianceStatus,
ConfigError,
ConfigKey,
Constraint,
ContentString,
Count,
CustomMessageVO,
DataFlowList};

use crate::taxonomy::{DescriptionVO,
DirectoryPath,
DiscoveryError,
DoctorResultVO,
Duration,
EnvContentVO,
ErrorCode,
ErrorMessage,
ExitCode,
ExpectedValue,
FieldName,
FileContentVO,
FileDefinitionMap,
FileFormat,
FilePath};

use crate::taxonomy::{FilePathList,
FileSystemError,
FixApplied,
FixResult,
GitDiffResultVO,
GitHookError,
GitRef,
GovernanceReport,
GraphAnalysisContext,
HookInstalled,
HookRemoved,
Identity,
ImportGraph,
ImportInfo,
ImportInfoList};

use crate::taxonomy::{ImportNameList,
InboundLinkMap,
InheritanceMap,
IntoPatternListValues,
JobError,
JobId,
JobIdList,
JobStatus,
LayerDefinition,
LayerMapVO,
LayerNameVO,
LegacyLayerRule,
LegacyLayerRuleList,
LineContentList,
LineContentVO};

use crate::taxonomy::{LineNumber,
LintMessage,
LintResult,
LintResultList,
LintStatusActionArgs,
LinterOperationError,
Location,
LocationList,
LogOutput,
MaintenanceStatsVO,
MandatoryImportRuleVO,
McpConfigVO,
MetadataVO,
MetricsError,
ModuleName};

use crate::taxonomy::{ModuleToFileMap,
NameVariants,
NamingConfig,
NamingError,
OrphanIndicatorResult,
PathNotFoundError,
PatternList,
PluginError,
PluginGroup,
Position,
PrimitiveTypeList,
PrimitiveTypeName,
PrimitiveViolation,
PrimitiveViolationList,
ProjectConfig};

use crate::taxonomy::{ProjectResult,
ReachabilityResult,
RegistrationError,
RenamedFile,
RenamedFileList,
ResponseData,
ResponseDataList,
ScanCompleted,
ScanError,
ScanFailed,
ScanStarted,
ScopeBounds,
ScopeRef,
ScopeResolutionError,
Score};

use crate::taxonomy::{SemanticError,
Severity,
SourceParserError,
StdError,
StdOutput,
SuccessStatus,
SuffixPolicyVO,
SuffixVO,
Suggestion,
SymbolName,
SymbolNameList,
SyntaxErrorVO,
Thresholds,
Timeout,
Timestamp};

use crate::taxonomy::{TransportEndpoint,
TransportError,
TransportProtocol,
TransportUrlVO,
ValidationError,
ViolationConstraint,
WatchEventError,
WatchResult,
WatchServiceError,
WatchSubscriptionError};
use crate::contract::{AdapterContainerAggregate,
AgentLifecycleAggregate,
AnalysisOrchestratorAggregate,
ArchCoordinatorAggregate,
CapabilityContainerAggregate,
CheckCommandsAggregate,
ContainerRegistryAggregate,
DevCommandsAggregate,
DirectoryWatchAggregate,
FixCommandsAggregate,
GitCommandsAggregate,
GitDiffResultAggregate,
HookManagementOrchestratorAggregate,
IAnalyzer,
IArchAnalyzerProtocol};
use crate::contract::{IArchCompliancePort,
IArchComplianceProtocol,
IArchImportProcessorProtocol,
IArchImportProtocol,
IArchInheritanceProtocol,
IArchLintProtocol,
IArchOrphanProtocol,
IArchRuleEngineProtocol,
IArchRuleProtocol,
IArchStructureProtocol,
ICodeQualityProtocol,
ICodeTransformationProtocol,
ICommandExecutorPort,
IConfigDiscoveryPort,
IConfigParserPort};
use crate::contract::{IConfigProviderPort,
IConfigRulesProtocol,
IConfigValidationPort,
ICycleAnalysisProtocol,
IDataFlowProtocol,
IDispatchRoutingParserProtocol,
IDispatchRoutingProtocol,
IDomainTypeProtocol,
IHookManagerPort,
IHttpProviderPort,
IInternalCheckerProtocol,
IJavascriptFlowPort,
IJavascriptScopePort,
IJobRegistryPort,
ILintReportingProtocol};
use crate::contract::{IMcpServerPort,
IMetricAnalyzerProtocol,
IMetricCheckerProtocol,
IMetricsProviderPort,
INamingCheckerProtocol,
INamingProviderPort,
INamingRuleProtocol,
INamingVariantPort,
INamingVariantProtocol,
IOrphanGraphProtocol,
IOrphanIndicatorProtocol,
IPathNormalizationPort,
IPluginManagerPort,
IRoleCheckerProtocol,
IScannerProviderPort};
use crate::contract::{IScopeBoundaryProtocol,
ISemanticTracerPort,
ISemanticTracerProtocol,
ISetupManagementProtocol,
ISourceParserPort,
IUnusedProtocol,
IWatchProviderPort,
InfrastructureContainerAggregate,
JobRegistryAggregate,
LintFixOrchestratorAggregate,
LintPipelineOrchestratorAggregate,
MaintenanceCommandsAggregate,
MultiProjectAggregate,
MultiProjectOrchestratorAggregate,
OrchestratorContainerAggregate};
use crate::contract::{OutputClientAggregate,
PipelineActionDispatcherAggregate,
PipelineExecutionOrchestratorAggregate,
PipelineExtendedOrchestratorAggregate,
PluginCommandsAggregate,
ProjectContainerAggregate,
ReportCommandsAggregate,
ServiceContainerAggregate,
SetupManagementAggregate,
ToolHandler,
WatchCommandsAggregate,
WatchExecutionOrchestratorAggregate};
pub struct GitCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl GitCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>, _cli: Option<&str>) {
        self.container = Some(container);
    }

    pub fn print_section<F, T>(&self, title: &str, items: &[T], item_fmt: F)
    where
        T: std::fmt::Display,
        F: Fn(&T),
    {
        if !items.is_empty() {
            println!("  {title} ({}):", items.len());
            for item in items {
                item_fmt(item);
            }
        }
    }

    pub fn print_diff_text(&self, base_ref: &str) {
        println!(" Changed files since {base_ref}:");
        println!("  No changed files detected.");
    }

    pub fn git_diff(&self, base: &str, output_format: &str) {
        if output_format == "json" {
            println!("{{\"added\": [], \"modified\": [], \"deleted\": [], \"lintable_files\": [], \"total_changed\": 0}}");
        } else {
            self.print_diff_text(base);
        }
    }
}

pub fn register_git_commands(container: Arc<dyn ServiceContainerAggregate>) -> GitCommandsSurface {
    let mut surface = GitCommandsSurface::new();
    surface.register_all(container, None);
    surface
}
