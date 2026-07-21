// PURPOSE: utility_orphan_io — stateless I/O utilities for orphan detection graph building
use std::path::Path;

/// Outcome of reading a file — either content or diagnostic info.
pub enum FileReadOutcome {
    Content(String),
    Unreadable { path: String, reason: String },
}

/// Read file contents, returning empty string on error (backward compatible).
pub fn read_file_safe(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Read file with diagnostic info — returns content or error details.
pub fn read_file_with_diagnostic(path: &str) -> FileReadOutcome {
    match std::fs::read_to_string(path) {
        Ok(content) => FileReadOutcome::Content(content),
        Err(err) => FileReadOutcome::Unreadable {
            path: path.to_string(),
            reason: err.to_string(),
        },
    }
}

/// List directory entries, skipping hidden files (starting with '.').
/// Returns vector of (file_name, file_path, is_dir) tuples.
pub fn list_directory_entries(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if name.starts_with('.') {
                    continue;
                }
                let path = dir_entry.path();
                let is_dir = path.is_dir();
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Check if a path exists and is a file.
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Check if a path exists and is a directory.
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Scan directory entries, returning vector of (file_name, file_path, is_dir) tuples.
/// Returns empty vec on error (same as list_directory_entries).
pub fn scan_directory(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                let path = dir_entry.path();
                let is_dir = path.is_dir();
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Recursively scan directory for files, returning vector of file paths.
/// Skips hidden directories and common heavy dependency/build directories.
pub fn scan_directory_recursive(dir_path: &Path) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = dir_path.read_dir() {
        for dir_entry in entries.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if name.starts_with('.') {
                    continue;
                }

                let path = dir_entry.path();

                if path.is_dir() {
                    if matches!(
                        name,
                        "target" | "node_modules" | "dist" | "build" | "__pycache__" | ".venv"
                    ) {
                        continue;
                    }

                    files.extend(scan_directory_recursive(&path));
                } else if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }

    files
}
