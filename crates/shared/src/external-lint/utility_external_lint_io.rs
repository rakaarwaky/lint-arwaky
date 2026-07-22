// PURPOSE: utility_external_lint_io — stateless I/O utilities for external lint adapters
use crate::common::utility_file;
use std::path::{Path, PathBuf};

/// Canonicalize a path, returning the original path on error.
pub fn canonicalize_path(path_str: &str) -> PathBuf {
    match std::fs::canonicalize(path_str) {
        Ok(p) => p,
        Err(_) => PathBuf::from(path_str),
    }
}

/// Scan directory entries, returning vector of (file_name, file_path, is_dir) tuples.
pub fn scan_directory(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                let path = dir_entry.path();
                let is_dir = utility_file_handler::is_dir(&path);
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Recursively scan directory for Python files.
/// Returns true if any .py file is found.
pub fn has_python_files(dir_path: &Path) -> bool {
    if let Ok(entries) = dir_path.read_dir() {
        for dir_entry in entries.flatten() {
            let path = dir_entry.path();
            if utility_file_handler::is_dir(&path) {
                if has_python_files(&path) {
                    return true;
                }
            } else if path.extension().map(|e| e == "py").unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

/// Check if a configuration file exists at the given path.
pub fn has_config_file(dir_path: &Path) -> bool {
    utility_file_handler::is_file_generic(dir_path.join("lint_arwaky.config.yaml"))
        || utility_file_handler::is_file_generic(dir_path.join("lint_arwaky.config.python.yaml"))
        || utility_file_handler::is_file_generic(dir_path.join("package.json"))
        || utility_file_handler::is_dir(dir_path.join(".git"))
}

/// Check if Cargo.toml exists at the given path (or parent/grandparent).
pub fn has_cargo_toml(path_str: &str) -> Option<String> {
    let current = Path::new(path_str);
    if utility_file_handler::is_dir(current)
        && utility_file_handler::is_file_generic(current.join("Cargo.toml"))
    {
        return Some(path_str.to_string());
    }
    if let Some(parent) = current.parent() {
        if utility_file_handler::is_file_generic(parent.join("Cargo.toml")) {
            return Some(parent.to_string_lossy().replace('\\', "/"));
        }
        if let Some(grandparent) = parent.parent() {
            if utility_file_handler::is_file_generic(grandparent.join("Cargo.toml")) {
                return Some(grandparent.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    None
}

/// Check if Cargo.lock exists at the given path (or parent/grandparent).
pub fn has_cargo_lock(path_str: &str) -> Option<String> {
    let current = Path::new(path_str);
    if utility_file_handler::is_dir(current)
        && utility_file_handler::is_file_generic(current.join("Cargo.lock"))
    {
        return Some(path_str.to_string());
    }
    if let Some(parent) = current.parent() {
        if utility_file_handler::is_file_generic(parent.join("Cargo.lock")) {
            return Some(parent.to_string_lossy().replace('\\', "/"));
        }
        if let Some(grandparent) = parent.parent() {
            if utility_file_handler::is_file_generic(grandparent.join("Cargo.lock")) {
                return Some(grandparent.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    None
}

/// Check if an executable exists in PATH directories.
pub fn is_executable_in_path(executable: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for path_dir in std::env::split_paths(&path_var) {
            let path = path_dir.join(executable);
            if utility_file_handler::is_file_generic(&path) {
                return true;
            }
        }
    }
    false
}

/// Check if a local bin executable exists.
pub fn has_local_bin(working_dir: &Path, executable: &str) -> bool {
    let local_bin = working_dir
        .join("node_modules")
        .join(".bin")
        .join(executable);
    utility_file_handler::is_file_generic(local_bin)
}
