// PURPOSE: Aggregate: Capability aggregation/wiring
pub trait CapabilityContainerAggregate: Send + Sync {
    fn _init_capabilities(&mut self);
}
