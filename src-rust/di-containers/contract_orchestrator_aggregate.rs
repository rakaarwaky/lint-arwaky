// PURPOSE: OrchestratorContainerAggregate — DI aggregate wiring for orchestrator implementations
pub trait OrchestratorContainerAggregate: Send + Sync {
    fn _init_orchestrators(&mut self);
}
