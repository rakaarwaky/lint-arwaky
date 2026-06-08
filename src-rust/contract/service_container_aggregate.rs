use crate::contract::command_executor_port::ICommandExecutorPort;
use crate::contract::file_system_port::IFileSystemPort;
use crate::contract::job_registry_port::IJobRegistryPort;
use crate::contract::lint_fix_aggregate::LintFixOrchestratorAggregate;
use crate::contract::linter_adapter_port::ILinterAdapterPort;
use crate::contract::metrics_provider_port::IMetricsProviderPort;
use crate::contract::path_normalization_port::IPathNormalizationPort;
use crate::contract::source_parser_port::ISourceParserPort;
use crate::contract::IArchLintProtocol;
use crate::taxonomy::AdapterName;
use std::sync::Arc;

pub trait ServiceContainerAggregate: Send + Sync {
    fn file_system(&self) -> Arc<dyn IFileSystemPort> {
        todo!("file_system not implemented")
    }
    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort> {
        todo!("command_executor not implemented")
    }
    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        todo!("path_normalization not implemented")
    }
    fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        todo!("source_parser not implemented")
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
}

pub struct DefaultServiceContainer {}
