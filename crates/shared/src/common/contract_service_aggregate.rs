use std::sync::Arc;

use crate::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::code_analysis::contract_lint_protocol::IArchLintProtocol;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::taxonomy_adapter_name_vo::AdapterName;

pub trait ServiceContainerAggregate: Send + Sync {
    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>>;

    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>>;

    fn get_fix_orchestrator(&self, dry_run: bool) -> Option<Arc<dyn LintFixOrchestratorAggregate>>;

    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>>;
}
