use super::*;

pub trait ContainerRegistryAggregate: Send + Sync {
    fn get_container(project_root: Option<&FilePath>) -> Box<dyn ServiceContainerAggregate>;
    fn reset_container(project_root: Option<&FilePath>);
}
