// PURPOSE: ServiceContainerAggregate — top-level DI aggregate bundling all container aggregates
use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::code_analysis::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;
use crate::output_report::contract_output_aggregate::IReportFormatterProtocol;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
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
    fn get_report_formatter(&self) -> Option<Box<dyn IReportFormatterProtocol>> {
        None
    }
}

pub struct DefaultServiceContainer {}

impl ServiceContainerAggregate for DefaultServiceContainer {
    fn file_system(&self) -> Arc<dyn IFileSystemPort> {
        todo!()
    }
    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort> {
        todo!()
    }
    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        todo!()
    }
    fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        todo!()
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
