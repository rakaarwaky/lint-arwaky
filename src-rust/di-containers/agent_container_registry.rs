// aes: wired-by-dispatch
// project_container_registry — Registry and provider for project-specific DI containers.
use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::code_analysis::contract_fix_aggregate::LintFixOrchestratorAggregate;
use crate::di_containers::contract_registry_aggregate::ContainerRegistryAggregate;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::layer_rules::contract_lint_protocol::IArchLintProtocol;
use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

// We use a simplified version — Container will be built in dependency_injection_container.
// This registry provides the module-level get_container / reset_container functions.

static CONTAINER_REGISTRY: Lazy<Mutex<HashMap<String, ()>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct ProjectContainerRegistry {}

impl ProjectContainerRegistry {
    pub fn get_container(project_root: Option<&FilePath>) -> Box<dyn ServiceContainerAggregate> {
        let key = project_root
            .map(|p| p.value.clone())
            .unwrap_or_else(|| ".".to_string());
        let mut registry = CONTAINER_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        registry.insert(key.clone(), ());
        Box::new(StubContainer {})
    }

    pub fn reset_container(project_root: Option<&FilePath>) {
        let key = project_root.map(|p| p.value.clone());
        let mut registry = CONTAINER_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        match key {
            Some(root) => {
                registry.remove(&root);
            }
            None => {
                registry.clear();
            }
        }
    }
}

impl ContainerRegistryAggregate for ProjectContainerRegistry {
    fn get_container(project_root: Option<&FilePath>) -> Box<dyn ServiceContainerAggregate> {
        Self::get_container(project_root)
    }
    fn reset_container(project_root: Option<&FilePath>) {
        Self::reset_container(project_root)
    }
}

struct StubContainer {}
impl ServiceContainerAggregate for StubContainer {
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
