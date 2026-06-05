// agent_container_registry — Registry and provider for project-specific DI containers.
use crate::contract::{crate::contract::service_container_aggregate::ServiceContainerAggregate, crate::contract::container_registry_aggregate::ContainerRegistryAggregate};
use crate::taxonomy::source_path_vo::DirectoryPath;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// We use a simplified version — Container will be built in dependency_injection_container.
// This registry provides the module-level get_container / reset_container functions.

static CONTAINER_REGISTRY: Lazy<Mutex<HashMap<String, ()>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub struct AgentContainerRegistry;

impl AgentContainerRegistry {
    pub fn get_container(project_root: Option<DirectoryPath>) -> Box<dyn ServiceContainerAggregate> {
        // Normalize path
        let root = project_root.unwrap_or_else(|| {
            DirectoryPath::new(".").unwrap()
        });
        let key = root.value.clone();

        let mut registry = CONTAINER_REGISTRY.lock().unwrap();
        if !registry.contains_key(&key) {
            registry.insert(key.clone(), ());
        }
        // Returns placeholder — real Container creation deferred to DI container module
        Box::new(StubContainer)
    }

    pub fn reset_container(project_root: Option<DirectoryPath>) {
        let mut registry = CONTAINER_REGISTRY.lock().unwrap();
        match project_root {
            Some(root) => {
                registry.remove(&root.value);
            }
            None => {
                registry.clear();
            }
        }
    }
}

impl ContainerRegistryAggregate for AgentContainerRegistry {}

struct StubContainer;
impl ServiceContainerAggregate for StubContainer {}

pub use AgentContainerRegistry as get_container;
pub use AgentContainerRegistry as reset_container;
