use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait ContainerRegistryAggregate: Send + Sync {
    fn get_container(project_root: Option<&FilePath>) -> Box<dyn ServiceContainerAggregate>;
    fn reset_container(project_root: Option<&FilePath>);
}
