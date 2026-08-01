// PURPOSE: Filesystem checker utility — re-export shim
// All functions consolidated into filesystem::utility_filesystem_io

pub use crate::filesystem::utility_filesystem_io::{create_dir_all, find_cache_dirs, remove_dir_all, walk_py_files};
