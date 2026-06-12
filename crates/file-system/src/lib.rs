// PURPOSE: file-system — file system port, adapter, and error types
// Depends on: shared (taxonomy/contract types)

pub use shared::file_system::contract_system_port::IFileSystemPort;
pub use shared::file_system::taxonomy_filesystem_error::{AccessDeniedError, FileSystemError, PathNotFoundError};

pub mod infrastructure_filesystem_adapter;
pub use infrastructure_filesystem_adapter::OSFileSystemAdapter;

pub mod root_file_container;
