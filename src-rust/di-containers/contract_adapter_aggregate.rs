pub trait AdapterContainerAggregate: Send + Sync {
    fn _init_adapters(&mut self);
}
