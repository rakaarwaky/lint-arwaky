// PURPOSE: Module declarations and re-exports for file-system (port, adapter, error)
pub mod infrastructure_filesystem_adapter;
pub use infrastructure_filesystem_adapter::OSFileSystemAdapter;

pub use shared::contract_system_port::IFileSystemPort;
pub use shared::taxonomy_filesystem_error::{
    AccessDeniedError, FileSystemError, PathNotFoundError,
};

pub mod file_container;
