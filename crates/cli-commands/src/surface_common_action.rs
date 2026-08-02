// PURPOSE: Shared utilities for CLI command surfaces
//
// Provides path resolution helpers. Tokio runtime factories removed —
// all surface methods are now sync. Async calls in shared contracts
// are handled at the binary entry point level.

use shared::common::FilePath;

pub fn resolve_file_path(path: &str) -> FilePath {
    FilePath::new(path.to_string()).unwrap_or_default()
}

pub fn canonicalize_path(path: &str) -> String {
    match std::path::Path::new(path).canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    }
}

pub fn current_dir() -> std::path::PathBuf {
    match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => std::path::PathBuf::new(),
    }
}
