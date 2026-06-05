/// CLI check and scan commands (Surface).
use std::collections::HashMap;
use std::path::PathBuf;

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
use crate::surfaces::cli_output_controller::{get_output_dir, write_output, tee_stdout};

pub struct CheckCommandsSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl CheckCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate) {
        self.container = Some(container);
    }

    pub fn check(&self, path: &str, git_diff: bool) {
        let path_vo = FilePath { value: path.to_string() };
        let diff_vo = BooleanVO { value: git_diff };
        self.run_check(path_vo, diff_vo);
    }

    pub fn scan(&self, path: &str) {
        let path_vo = FilePath { value: path.to_string() };
        let diff_vo = BooleanVO { value: false };
        self.run_check(path_vo, diff_vo);
    }

    fn format_report(&self, report: &GovernanceReport) -> String {
        let mut lines = Vec::new();

        // Group results by source
        let mut source_results: HashMap<String, Vec<&LintResult>> = HashMap::new();
        for res in &report.results {
            let src = res.source.as_ref().map(|s| s.value.clone()).unwrap_or_else(|| "unknown".to_string());
            source_results.entry(src).or_default().push(res);
        }

        for (source, results) in &source_results {
            let status = if results.is_empty() {
                " CLEAN".to_string()
            } else {
                format!(" {} ISSUES", results.len())
            };
            lines.push(format!("[{source}]{status}"));
            for res in results {
                lines.push(format!(
                    " - {file}:{line} {code}: {msg}",
                    file = res.file.value,
                    line = res.line.value,
                    code = res.code.as_deref().unwrap_or(""),
                    msg = res.message.value,
                ));
            }
        }

        lines.push("-".repeat(40));
        lines.push(format!("total issues :  {}", report.results.len()));
        lines.push(format!("total score  :  {:.1}/100.0", report.score.value));
        lines.push("-".repeat(40));

        lines.join("\n")
    }

    async fn handle_git_diff(&self, container: &ServiceContainerAggregate, project_path: &FilePath) {
        // In real impl: call container.analysis_orchestrator.run
        let report = GovernanceReport {
            results: vec![],
            score: Score::new(100.0).unwrap(),
            is_passing: BooleanVO { value: true },
        };
        let report_text = self.format_report(&report);
        println!("{report_text}");
    }

    async fn handle_full_analysis(&self, container: &ServiceContainerAggregate, project_path: &FilePath) {
        println!(" Running analysis on {}...", project_path.value);
        let report = GovernanceReport {
            results: vec![],
            score: Score::new(100.0).unwrap(),
            is_passing: BooleanVO { value: true },
        };
        let report_text = self.format_report(&report);
        println!("{report_text}");
    }

    fn run_check(&self, project_path: FilePath, git_diff: BooleanVO) {
        let output_dir = get_output_dir(None);

        let output = tee_stdout(None, || {
            if git_diff.value {
                println!("[git-diff] Running analysis on {}", project_path.value);
            } else {
                println!(" Running analysis on {}...", project_path.value);
            }
            // Structural placeholder
            println!("{}", "-".repeat(40));
            println!("total issues :  0");
            println!("total score  :  100.0/100.0");
            println!("{}", "-".repeat(40));
        });

        if let Some(dir) = output_dir {
            write_output(None, &output, "check", Some("txt"));
        }
    }

    fn aggregate_source_counts(&self, reports: &[(String, GovernanceReport)]) -> HashMap<String, i32> {
        let mut counts = HashMap::new();
        for (_, report) in reports {
            for res in &report.results {
                let source = res.source.as_ref().map(|s| s.value.clone()).unwrap_or_else(|| "unknown".to_string());
                *counts.entry(source).or_insert(0) += 1;
            }
        }
        counts
    }

    fn print_source_summary(&self, source_counts: &HashMap<String, i32>) {
        for (source, count) in source_counts {
            let status = if *count == 0 {
                " CLEAN".to_string()
            } else {
                format!(" {count} ISSUES")
            };
            println!("[{source}]{status}");
        }
    }
}

pub fn register_check_commands(container: ServiceContainerAggregate) -> CheckCommandsSurface {
    let mut surface = CheckCommandsSurface::new();
    surface.register_all(container);
    surface
}
