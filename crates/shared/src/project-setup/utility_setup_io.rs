// PURPOSE: Setup I/O utility — stateless filesystem and process helpers for project setup

use std::fs;
use std::path::Path;

/// Write file content. Returns Ok(()) or Err(io::Error).
pub fn write_file_content(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

/// Create directory recursively. Returns Ok(()) or Err(io::Error).
pub fn create_dir(path: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

/// Read directory entries. Returns vector of paths or Err on error.
pub fn read_dir_entries(dir: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut entries = Vec::new();
    for e in fs::read_dir(dir)?.flatten() {
        entries.push(e.path());
    }
    Ok(entries)
}
