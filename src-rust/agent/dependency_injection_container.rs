/// dependency_injection_container — Implementation of the DI container.
use crate::contract::{
    IArchLintProtocol, ICommandExecutorPort, IFileSystemPort, IJobRegistryPort, ILinterAdapterPort,
    IMetricsProviderPort, IPathNormalizationPort, ISourceParserPort, LintFixOrchestratorAggregate,
    ServiceContainerAggregate,
};
use crate::infrastructure::{
    ASTJSParserAdapter, ASTPythonParserAdapter, ASTRustParserAdapter, BanditAdapter,
    ComplexityAdapter, DependencyAdapter, DuplicateAdapter, ESLintAdapter,
    MemoryJobRegistryAdapter, MetricsProvider, MyPyAdapter, OSFileSystemAdapter,
    PathNormalizationProvider, PrettierAdapter, RuffAdapter, RustLinterAdapter,
    SourceParserOrchestrator, StdioClient, TSCAdapter, TrendsAdapter,
};
use crate::taxonomy::source_path_vo::DirectoryPath;
use crate::taxonomy::AdapterName;
use crate::taxonomy::{Count, FilePath};
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
}

impl DependencyInjectionContainer {
    pub fn new(_root: DirectoryPath) -> Self {
        let fs: Arc<dyn IFileSystemPort> = Arc::new(OSFileSystemAdapter::new());
        let executor: Arc<dyn ICommandExecutorPort> =
            Arc::new(StdioClient::new(std::time::Duration::from_secs(60)));
        let path_norm: Arc<dyn IPathNormalizationPort> = Arc::new(PathNormalizationProvider);
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(SourceParserOrchestrator::new(
            Box::new(ASTPythonParserAdapter::new()),
            Box::new(ASTRustParserAdapter::new()),
            Box::new(ASTJSParserAdapter::new()),
        ));
        let arch_linter: Arc<dyn IArchLintProtocol> = Arc::new(
            crate::agent::architecture_lint_orchestrator::ArchitectureLintOrchestrator::new(),
        );

        let mut linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();

        let ruff = Arc::new(RuffAdapter::new(executor.clone(), path_norm.clone(), None));
        linter_adapters.insert("ruff".to_string(), ruff);

        let bandit = Arc::new(BanditAdapter::new(
            executor.clone(),
            path_norm.clone(),
            None,
        ));
        linter_adapters.insert("bandit".to_string(), bandit);

        let mypy = Arc::new(MyPyAdapter::new(executor.clone(), path_norm.clone(), None));
        linter_adapters.insert("mypy".to_string(), mypy);

        let eslint = Arc::new(ESLintAdapter::new(executor.clone(), path_norm.clone()));
        linter_adapters.insert("eslint".to_string(), eslint);

        let prettier = Arc::new(PrettierAdapter::new(executor.clone(), path_norm.clone()));
        linter_adapters.insert("prettier".to_string(), prettier);

        let tsc = Arc::new(TSCAdapter::new(executor.clone(), path_norm.clone()));
        linter_adapters.insert("tsc".to_string(), tsc);

        let clippy = Arc::new(RustLinterAdapter::new(
            executor.clone(),
            path_norm.clone(),
            None,
        ));
        linter_adapters.insert("clippy".to_string(), clippy);

        let complexity = Arc::new(ComplexityAdapter::new(
            executor.clone(),
            path_norm.clone(),
            None,
            Count::new(10),
        ));
        linter_adapters.insert("complexity".to_string(), complexity);

        let duplicate = Arc::new(DuplicateAdapter::new(
            executor.clone(),
            path_norm.clone(),
            None,
        ));
        linter_adapters.insert("duplicate".to_string(), duplicate);

        let trends = Arc::new(TrendsAdapter::new(
            executor.clone(),
            path_norm.clone(),
            FilePath::new(".lint-trends.json".to_string()).unwrap_or_default(),
        ));
        linter_adapters.insert("trends".to_string(), trends);

        let dependency = Arc::new(DependencyAdapter::new(
            executor.clone(),
            path_norm.clone(),
            None,
        ));
        linter_adapters.insert("dependency".to_string(), dependency);

        let metrics_provider: Arc<dyn IMetricsProviderPort> = Arc::new(MetricsProvider::new(
            path_norm.clone(),
            ".lint-history.json",
        ));

        Self {
            file_system: fs,
            command_executor: executor,
            path_normalization: path_norm,
            source_parser,
            architecture_linter: arch_linter,
            linter_adapters,
            metrics_provider,
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
                .get_or_init(|| Arc::new(MemoryJobRegistryAdapter::new()))
                .clone(),
        )
    }

    fn metrics_provider(&self) -> Option<Arc<dyn IMetricsProviderPort>> {
        Some(self.metrics_provider.clone())
    }

    fn get_fix_orchestrator(&self, dry_run: bool) -> Option<Arc<dyn LintFixOrchestratorAggregate>> {
        Some(Arc::new(
            crate::agent::lint_fix_orchestrator::LintFixOrchestrator::with_dry_run(
                dry_run,
                self.architecture_linter.clone(),
            ),
        ))
    }
}
