// PURPOSE: CapabilityContainerAggregate — DI aggregate wiring for capability checker implementations
pub trait CapabilityContainerAggregate: Send + Sync {
    fn _init_capabilities(&mut self);
}
