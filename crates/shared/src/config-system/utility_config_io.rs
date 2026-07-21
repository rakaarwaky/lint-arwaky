// PURPOSE: Config I/O utility — file read and path existence helpers
use crate::common::utility_file;
use std::path::Path;

pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20; // 1 MiB

/// Check if a path exists (blocking).
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    utility_file::path_exists(path)
}

/// Check if a path is a file (blocking).
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    utility_file::is_file_generic(path)
}

/// Sync read file to string.
pub fn read_file_sync<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    utility_file::read_file_generic(path)
}

/// Async read file to string.
pub async fn read_file_async<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    tokio::fs::read_to_string(path).await
}

/// Read a file within the canonical root, enforcing path confinement and max file size.
pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> std::io::Result<String> {
    let path = path.as_ref();
    let canonical_path = tokio::fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }
    let meta = tokio::fs::metadata(&canonical_path).await?;
    if !utility_file::is_file_generic(&canonical_path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }
    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }
    tokio::fs::read_to_string(&canonical_path).await
}
