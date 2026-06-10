// PURPOSE: AdapterContainerAggregate — DI aggregate wiring for language adapter implementations
pub trait AdapterContainerAggregate: Send + Sync {
    fn _init_adapters(&mut self);
}
