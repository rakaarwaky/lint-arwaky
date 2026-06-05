/// dependency_injection_container — Implementation of the DI container.
use crate::contract::{
    AdapterContainerAggregate, AgentLifecycleAggregate, AnalysisOrchestratorAggregate,
    ArchCoordinatorAggregate, CapabilityContainerAggregate, CheckCommandsAggregate,
    ContainerRegistryAggregate, DevCommandsAggregate, DirectoryWatchAggregate,
    FixCommandsAggregate, GitCommandsAggregate, GitDiffResultAggregate,
    HookManagementOrchestratorAggregate, IAnalyzer, IArchAnalyzerProtocol, IFileSystemPort,
    ILinterAdapterPort,
};
use crate::contract::{
    IArchCompliancePort, IArchComplianceProtocol, IArchImportProcessorProtocol,
    IArchImportProtocol, IArchInheritanceProtocol, IArchLintProtocol, IArchOrphanProtocol,
    IArchRuleEngineProtocol, IArchRuleProtocol, IArchStructureProtocol, ICodeQualityProtocol,
    ICodeTransformationProtocol, ICommandExecutorPort, IConfigDiscoveryPort, IConfigParserPort,
};
use crate::contract::{
    IConfigProviderPort, IConfigRulesProtocol, IConfigValidationPort, ICycleAnalysisProtocol,
    IDataFlowProtocol, IDispatchRoutingParserProtocol, IDispatchRoutingProtocol,
    IDomainTypeProtocol, IHookManagerPort, IHttpProviderPort, IInternalCheckerProtocol,
    IJavascriptFlowPort, IJavascriptScopePort, IJobRegistryPort, ILintReportingProtocol,
};
use crate::contract::{
    IMcpServerPort, IMetricAnalyzerProtocol, IMetricCheckerProtocol, IMetricsProviderPort,
    INamingCheckerProtocol, INamingProviderPort, INamingRuleProtocol, INamingVariantPort,
    INamingVariantProtocol, IOrphanGraphProtocol, IOrphanIndicatorProtocol, IPathNormalizationPort,
    IPluginManagerPort, IRoleCheckerProtocol, IScannerProviderPort,
};
use crate::contract::{
    IScopeBoundaryProtocol, ISemanticTracerPort, ISemanticTracerProtocol, ISetupManagementProtocol,
    ISourceParserPort, IUnusedProtocol, IWatchProviderPort, InfrastructureContainerAggregate,
    JobRegistryAggregate, LintFixOrchestratorAggregate, LintPipelineOrchestratorAggregate,
    MaintenanceCommandsAggregate, MultiProjectAggregate, MultiProjectOrchestratorAggregate,
    OrchestratorContainerAggregate,
};
use crate::contract::{
    OutputClientAggregate, PipelineActionDispatcherAggregate,
    PipelineExecutionOrchestratorAggregate, PipelineExtendedOrchestratorAggregate,
    PluginCommandsAggregate, ProjectContainerAggregate, ReportCommandsAggregate,
    ServiceContainerAggregate, SetupManagementAggregate, ToolHandler, WatchCommandsAggregate,
    WatchExecutionOrchestratorAggregate,
};
use crate::infrastructure::*;
use crate::taxonomy::source_path_vo::DirectoryPath;
use crate::capabilities::ArchLintHandler;
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
}

impl DependencyInjectionContainer {
    pub fn new(_root: DirectoryPath) -> Self {
        let fs: Arc<dyn IFileSystemPort> = Arc::new(OSFileSystemAdapter::new());
        let executor: Arc<dyn ICommandExecutorPort> = Arc::new(StdioClient::new(std::time::Duration::from_secs(60)));
        let path_norm: Arc<dyn IPathNormalizationPort> = Arc::new(PathNormalizationProvider::new());
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(SourceParserOrchestrator::new());
        let arch_linter: Arc<dyn IArchLintProtocol> = Arc::new(ArchLintHandler::new(fs.clone(), source_parser.clone()));

        let mut linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        let ruff = Arc::new(RuffAdapter::new(executor.clone(), path_norm.clone(), None));
        linter_adapters.insert("ruff".to_string(), ruff);

        Self {
            file_system: fs,
            command_executor: executor,
            path_normalization: path_norm,
            source_parser,
            architecture_linter: arch_linter,
            linter_adapters,
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

    fn linter_adapter(&self, name: &str) -> Option<Arc<dyn ILinterAdapterPort>> {
        self.linter_adapters.get(name).cloned()
    }

    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>> {
        Some(self.architecture_linter.clone())
    }

    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>> {
        use std::sync::OnceLock;
        static REGISTRY: OnceLock<Arc<dyn IJobRegistryPort>> = OnceLock::new();
        Some(REGISTRY.get_or_init(|| Arc::new(MemoryJobRegistryAdapter::new())).clone())
    }
}
