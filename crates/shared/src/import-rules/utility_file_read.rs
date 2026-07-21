// PURPOSE: File read utility — stateless file content reading helper
use std::path::Path;

/// Read a file to string, returning None on error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Read a file to string, returning Result for proper error handling.
pub fn read_file_result<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
