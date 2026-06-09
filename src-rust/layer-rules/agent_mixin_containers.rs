//! Mixin container stubs — DI initialization stubs wired via DependencyInjectionContainer.

use crate::di_containers::contract_infra_aggregate::InfrastructureContainerAggregate;
use crate::di_containers::contract_orchestrator_aggregate::OrchestratorContainerAggregate;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct InfrastructureMixinContainer {}

impl InfrastructureMixinContainer {
    pub fn init_infrastructure(&self) {}
}

impl InfrastructureContainerAggregate for InfrastructureMixinContainer {
    fn _init_infrastructure(&mut self) {}

    fn root_path(&self) -> Option<&FilePath> {
        None
    }
}

pub struct OrchestratorMixinContainer {}

impl OrchestratorContainerAggregate for OrchestratorMixinContainer {
    fn _init_orchestrators(&mut self) {}
}

impl OrchestratorMixinContainer {
    pub fn init_orchestrators(&self) {}
}
