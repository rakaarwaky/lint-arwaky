// PURPOSE: utility_orphan_io — re-export shim
// All functions consolidated into filesystem::utility_filesystem_io

pub use crate::filesystem::utility_filesystem_io::{
    is_dir, is_file, list_directory_entries, read_file_safe, read_file_with_diagnostic,
    scan_directory, scan_directory_recursive,
};
