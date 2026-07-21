// PURPOSE: Auto-fix I/O utility — stateless file read/write helpers
use std::path::Path;

/// Check if a path exists.
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

/// Read a file to string, returning None on error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Write content to a file.
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> bool {
    std::fs::write(path, contents).is_ok()
}
