// PURPOSE: Role I/O utility — stateless file reading helpers for role auditing

use crate::common::utility_file;

/// Read a file's content. Returns Ok(content) or Err(io::Error).
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    utility_file::read_file(path)
}
