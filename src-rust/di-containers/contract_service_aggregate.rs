pub use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
pub use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
pub use crate::code_analysis::contract_fix_aggregate::LintFixOrchestratorAggregate;
pub use crate::di_containers::contract_infra_aggregate::InfrastructureContainerAggregate;
pub use crate::di_containers::contract_orchestrator_aggregate::OrchestratorContainerAggregate;
pub use crate::file_system::contract_system_port::IFileSystemPort;
pub use crate::file_watch::contract_commands_aggregate::WatchCommandsAggregate;
pub use crate::file_watch::contract_orchestrator_aggregate::WatchExecutionOrchestratorAggregate;
pub use crate::file_watch::contract_watch_aggregate::DirectoryWatchAggregate;
pub use crate::layer_rules::contract_compliance_port::IArchCompliancePort;
pub use crate::layer_rules::contract_compliance_protocol::IArchComplianceProtocol;
pub use crate::layer_rules::contract_coordinator_aggregate::ArchCoordinatorAggregate;
pub use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
pub use crate::layer_rules::contract_rule_protocol::IAnalyzer;
pub use crate::layer_rules::contract_rule_protocol::IArchRuleProtocol;
pub use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;
pub use crate::pipeline_jobs::contract_registry_aggregate::JobRegistryAggregate;
pub use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::shared_common::taxonomy_name_vo::AdapterName;
pub use crate::source_parsing::contract_normalization_port::IPathNormalizationPort;
pub use crate::source_parsing::contract_parser_port::ISourceParserPort;
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

impl ServiceContainerAggregate for DefaultServiceContainer {}
