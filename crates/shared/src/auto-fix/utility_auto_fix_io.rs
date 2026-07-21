// PURPOSE: Auto-fix I/O utility — stateless file read/write helpers
use crate::common::utility_file;
use std::path::Path;

/// Check if a path exists.
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    utility_file::path_exists(path)
}

/// Read a file to string, returning None on error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    utility_file::read_file_generic(path).ok()
}

/// Write content to a file.
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> bool {
    utility_file::write_file(path, contents).is_ok()
}
