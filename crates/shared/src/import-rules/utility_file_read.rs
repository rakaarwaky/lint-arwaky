// PURPOSE: File read utility — stateless file content reading helper
use std::path::Path;

/// Read a file to string, returning None on error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
