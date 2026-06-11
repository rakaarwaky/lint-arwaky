// PURPOSE: FileContainer — wiring for file-system feature (root layer, wiring only)
use std::sync::Arc;
use crate::infrastructure_filesystem_adapter::OSFileSystemAdapter;
use shared::contract_system_port::IFileSystemPort;

pub struct FileContainer {
    filesystem: Arc<dyn IFileSystemPort>,
}

impl FileContainer {
    pub fn new() -> Self {
        Self {
            filesystem: Arc::new(OSFileSystemAdapter::new()),
        }
    }

    pub fn filesystem(&self) -> Arc<dyn IFileSystemPort> {
        self.filesystem.clone()
    }
}
impl Default for FileContainer {
    fn default() -> Self {
        Self::new()
    }
}
