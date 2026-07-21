// PURPOSE: utility_external_lint_io — stateless I/O utilities for external lint adapters
use std::path::{Path, PathBuf};

/// Canonicalize a path, returning the original path on error.
pub fn canonicalize_path(path_str: &str) -> PathBuf {
    match std::fs::canonicalize(path_str) {
        Ok(p) => p,
        Err(_) => PathBuf::from(path_str),
    }
}

/// Check if a path is a file.
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Check if a path is a directory.
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Check if a path exists.
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Scan directory entries, returning vector of (file_name, file_path, is_dir) tuples.
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

/// Recursively scan directory for Python files.
/// Returns true if any .py file is found.
pub fn has_python_files(dir_path: &Path) -> bool {
    if let Ok(entries) = dir_path.read_dir() {
        for dir_entry in entries.flatten() {
            let path = dir_entry.path();
            if path.is_dir() {
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

/// Read file contents, returning empty string on error.
pub fn read_file_safe(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => String::new(),
    }
}

/// Check if a configuration file exists at the given path.
pub fn has_config_file(dir_path: &Path) -> bool {
    dir_path.join("lint_arwaky.config.yaml").is_file()
        || dir_path.join("lint_arwaky.config.python.yaml").is_file()
        || dir_path.join("package.json").is_file()
        || dir_path.join(".git").is_dir()
}

/// Check if Cargo.toml exists at the given path (or parent/grandparent).
pub fn has_cargo_toml(path_str: &str) -> Option<String> {
    let current = Path::new(path_str);
    if current.is_dir() && current.join("Cargo.toml").is_file() {
        return Some(path_str.to_string());
    }
    if let Some(parent) = current.parent() {
        if parent.join("Cargo.toml").is_file() {
            return Some(parent.to_string_lossy().replace('\\', "/"));
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.toml").is_file() {
                return Some(grandparent.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    None
}

/// Check if Cargo.lock exists at the given path (or parent/grandparent).
pub fn has_cargo_lock(path_str: &str) -> Option<String> {
    let current = Path::new(path_str);
    if current.is_dir() && current.join("Cargo.lock").is_file() {
        return Some(path_str.to_string());
    }
    if let Some(parent) = current.parent() {
        if parent.join("Cargo.lock").is_file() {
            return Some(parent.to_string_lossy().replace('\\', "/"));
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.lock").is_file() {
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
            if path.is_file() {
                return true;
            }
        }
    }
    false
}

/// Check if a local bin executable exists.
pub fn has_local_bin(working_dir: &Path, executable: &str) -> bool {
    let local_bin = working_dir.join("node_modules").join(".bin").join(executable);
    local_bin.is_file()
}
