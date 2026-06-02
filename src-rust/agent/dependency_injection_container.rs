/// dependency_injection_container — Implementation of the DI container.
use crate::contract::*;
use crate::infrastructure::*;
use crate::taxonomy::DirectoryPath;
use std::sync::Arc;
use std::collections::HashMap;

pub struct DependencyInjectionContainer {
    file_system: Arc<dyn IFileSystemPort>,
    command_executor: Arc<dyn ICommandExecutorPort>,
    path_normalization: Arc<dyn IPathNormalizationPort>,
    linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
}

impl DependencyInjectionContainer {
    pub fn new(root: DirectoryPath) -> Self {
        let fs = Arc::new(OSFileSystemAdapter::new());
        let executor = Arc::new(StdioClient::new(std::time::Duration::from_secs(60)));
        let path_norm = Arc::new(PathNormalizationProvider::new()); // Stub for now
        
        let mut linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        let ruff = Arc::new(RuffAdapter::new(executor.clone(), path_norm.clone(), None));
        linter_adapters.insert("ruff".to_string(), ruff);

        Self {
            file_system: fs,
            command_executor: executor,
            path_normalization: path_norm,
            linter_adapters,
        }
    }
}

impl ServiceContainerAggregate for DependencyInjectionContainer {
    fn file_system(&self) -> Arc<dyn IFileSystemPort> {
        self.file_system.clone()
    }

    fn command_executor(&self) -> Arc<dyn ICommandExecutorPort> {
        self.command_executor.clone()
    }

    fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        self.path_normalization.clone()
    }

    fn linter_adapter(&self, name: &str) -> Option<Arc<dyn ILinterAdapterPort>> {
        self.linter_adapters.get(name).cloned()
    }
}
