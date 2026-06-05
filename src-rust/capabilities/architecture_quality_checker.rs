/// Architecture rule checking: bypass comments and unused imports.
use crate::contract::IFileSystemPort;
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
use async_trait::async_trait;

pub struct CodeQualityRuleChecker;

impl CodeQualityRuleChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn rule_name(&self) -> Identity {
        Identity::new("code_quality")
    }

    fn find_bypass_matches(
        &self,
        line: &str,
        forbidden: &[String],
        base_msg: &str,
    ) -> Option<(i64, String)> {
        let lower_line = line.to_lowercase();
        for bypass in forbidden {
            if lower_line.contains(&bypass.to_lowercase()) {
                let col = line.find('#').unwrap_or(0) as i64;
                return Some((col, base_msg.to_string()));
            }
        }
        None
    }

    pub async fn check_no_bypass_comments(
        &self,
        file_path: FilePath,
        fs: &dyn IFileSystemPort,
        results: &mut LintResultList,
        forbidden_words: Option<PatternList>,
        violation_message: Option<ErrorMessage>,
    ) {
        let forbidden = match forbidden_words {
            Some(fw) if !fw.values.is_empty() => fw.values,
            _ => return,
        };

        let content = match fs.read_text(file_path.clone()).await {
            Ok(c) => c.value,
            Err(_) => return,
        };

        let base_msg = violation_message
            .map(|m| m.value)
            .unwrap_or_else(|| "STOP CHEATING! You are strictly forbidden from using bypass comments.".to_string());

        for (i, line) in content.lines().enumerate() {
            if let Some((col, final_msg)) = self.find_bypass_matches(line, &forbidden, &base_msg) {
                results.push(LintResult {
                    file: file_path.clone(),
                    line: LineNumber::new((i + 1) as i64),
                    column: ColumnNumber::new(col),
                    code: ErrorCode::raw("AES014"),
                    message: LintMessage::new(final_msg),
                    source: AdapterName::raw("architecture"),
                    severity: Severity::CRITICAL,
                    enclosing_scope: crate::taxonomy::ScopeRef::default(),
                    related_locations: crate::taxonomy::LocationList::default(),
                });
            }
        }
    }
}
