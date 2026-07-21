// PURPOSE: File read utility — stateless file content reading helper
use crate::common::utility_file;
use std::path::Path;

/// Read a file to string, returning None on error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    utility_file::read_file_generic(path).ok()
}

/// Read a file to string, returning Result for proper error handling.
pub fn read_file_result<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    utility_file::read_file_generic(path)
}
