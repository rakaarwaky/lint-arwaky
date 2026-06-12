// PURPOSE: file-system — file system port, adapter, and error types
// Depends on: shared (taxonomy/contract types)

pub mod infrastructure_filesystem_adapter;
pub use infrastructure_filesystem_adapter::OSFileSystemAdapter;

pub mod root_file_container;
