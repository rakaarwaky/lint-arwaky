// project_container_registry — Registry and provider for project-specific DI containers.
use crate::contract::{ContainerRegistryAggregate, ServiceContainerAggregate};
use crate::taxonomy::FilePath;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// We use a simplified version — Container will be built in dependency_injection_container.
// This registry provides the module-level get_container / reset_container functions.

static CONTAINER_REGISTRY: Lazy<Mutex<HashMap<String, ()>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct ProjectContainerRegistry;

impl ProjectContainerRegistry {
    pub fn get_container(project_root: Option<&FilePath>) -> Box<dyn ServiceContainerAggregate> {
        let key = project_root
            .map(|p| p.value.clone())
            .unwrap_or_else(|| ".".to_string());
        let mut registry = CONTAINER_REGISTRY.lock().unwrap();
        registry.insert(key.clone(), ());
        Box::new(StubContainer)
    }

    pub fn reset_container(project_root: Option<&FilePath>) {
        let key = project_root.map(|p| p.value.clone());
        let mut registry = CONTAINER_REGISTRY.lock().unwrap();
        match key {
            Some(root) => {
                registry.remove(&root);
            }
            None => {
                registry.clear();
            }
        }
    }
}

impl ContainerRegistryAggregate for ProjectContainerRegistry {
    fn get_container(project_root: Option<&FilePath>) -> Box<dyn ServiceContainerAggregate> {
        Self::get_container(project_root)
    }
    fn reset_container(project_root: Option<&FilePath>) {
        Self::reset_container(project_root)
    }
}

struct StubContainer;
impl ServiceContainerAggregate for StubContainer {}
