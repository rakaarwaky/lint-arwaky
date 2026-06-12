// PURPOSE: FileContainer — wiring for file-system feature (root layer, wiring only)
use std::sync::Arc;
use shared::file_system::contract_system_port::IFileSystemPort;

pub struct FileContainer {
    filesystem: Arc<dyn IFileSystemPort>,
}

impl FileContainer {
    pub fn new() -> Self {
        Self {
            filesystem: Arc::new(
                crate::infrastructure_filesystem_adapter::OSFileSystemAdapter::new(),
            ),
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


