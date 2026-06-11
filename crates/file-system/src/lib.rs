// PURPOSE: Module declarations and re-exports for file-system (port, adapter, error)
pub mod contract_system_port;
pub use contract_system_port::IFileSystemPort;
pub mod infrastructure_filesystem_adapter;
pub use infrastructure_filesystem_adapter::OSFileSystemAdapter;
pub mod taxonomy_filesystem_error;
pub use taxonomy_filesystem_error::{AccessDeniedError, FileSystemError, PathNotFoundError};
pub mod file_container;
