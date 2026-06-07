use crate::taxonomy::FilePath;

pub trait InfrastructureContainerAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    fn _init_infrastructure(&mut self);
}
