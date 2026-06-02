use super::*;

pub trait ServiceContainerAggregate: Send + Sync {
    fn get<T: 'static>(&self) -> Option<&T>;
    fn get_for_path(&self, path: &FilePath) -> Box<dyn ServiceContainerAggregate>;
}
