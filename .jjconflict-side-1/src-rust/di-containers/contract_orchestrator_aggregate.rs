pub trait OrchestratorContainerAggregate: Send + Sync {
    fn _init_orchestrators(&mut self);
}
