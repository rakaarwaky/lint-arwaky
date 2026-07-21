// PURPOSE: Config I/O utility — file read and path existence helpers
use std::path::Path;

/// Check if a path exists (blocking).
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

/// Check if a path is a file (blocking).
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).is_file()
}

/// Sync read file to string.
pub fn read_file_sync<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Async read file to string.
pub async fn read_file_async<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    tokio::fs::read_to_string(path).await
}
