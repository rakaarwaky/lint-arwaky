// PURPOSE: Role I/O utility — stateless file reading helpers for role auditing

use std::fs;

/// Read a file's content. Returns Ok(content) or Err(io::Error).
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
