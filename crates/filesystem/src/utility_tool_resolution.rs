// FR-004: Tool Resolution
// Produces: tool paths, availability, config presence
// Consumers: external-lint, maintenance
//
// Utility: stateless standalone functions

use std::path::{Path, PathBuf};

use crate::utility_filesystem_io::{canonicalize_path, is_file, path_exists};

// ═══════════════════════════════════════════════════════════════
// PATH Detection
// ═══════════════════════════════════════════════════════════════

/// Check if an executable exists in PATH environment variable.
pub fn is_executable_in_path(executable: &str) -> bool {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|dir| {
            let p = Path::new(dir).join(executable);
            p.exists() && p.metadata().is_ok_and(|m| m.is_file())
        })
}

/// Check if a binary is available in the system PATH.
pub fn is_binary_available(bin_name: &str) -> bool {
    if bin_name.is_empty()
        || bin_name
            .chars()
            .any(|c| !c.is_alphanumeric() && c != '_' && c != '-')
    {
        return false;
    }

    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .is_none_or(|dir| {
            let path = dir.join(bin_name);
            path_exists(path) || find_in_path(bin_name)
        })
}

fn find_in_path(bin_name: &str) -> bool {
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let path = dir.join(bin_name);
            if path_exists(path) {
                return true;
            }
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════
// Local Bin Detection
// ═══════════════════════════════════════════════════════════════

/// Check if an executable exists in local node_modules/.bin directory.
pub fn has_local_bin(working_dir: &Path, executable: &str) -> bool {
    let local_bin = working_dir
        .join("node_modules")
        .join(".bin")
        .join(executable);
    local_bin.exists()
}

// ═══════════════════════════════════════════════════════════════
// JS Tool Resolution
// ═══════════════════════════════════════════════════════════════

/// Resolve JS tool command from local node_modules/.bin.
pub fn resolve_js_cmd(
    executable: &str,
    args: Vec<String>,
    working_dir: &str,
) -> Option<Vec<String>> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);
    if path_exists(&local_bin) {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return Some(cmd);
    }
    None
}

/// Walk up to find JS project root.
pub fn resolve_js_working_dir(path: &std::path::Path) -> PathBuf {
    let abs_path = canonicalize_path(path.to_string_lossy().to_string().as_str());
    let mut current = if is_file(&abs_path) {
        abs_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        abs_path.clone()
    };
    for _ in 0..10 {
        if is_file(current.join("lint_arwaky.config.yaml"))
            || is_file(current.join("package.json"))
            || current.join(".git").is_dir()
        {
            return current;
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }
    current
}

// ═══════════════════════════════════════════════════════════════
// Cargo Tool Resolution
// ═══════════════════════════════════════════════════════════════

/// Find parent dir with Cargo.toml.
pub fn resolve_cargo_working_dir(path_str: &str) -> PathBuf {
    if path_str.is_empty() {
        return PathBuf::from(".");
    }
    let current = Path::new(path_str);
    if current.is_dir() && path_exists(current.join("Cargo.toml")) {
        return current.to_path_buf();
    }
    if let Some(parent) = current.parent() {
        if path_exists(parent.join("Cargo.toml")) {
            return parent.to_path_buf();
        }
        if let Some(grandparent) = parent.parent()
            && path_exists(grandparent.join("Cargo.toml"))
        {
            return grandparent.to_path_buf();
        }
    }
    PathBuf::from(".")
}

/// Find parent dir with Cargo.lock.
pub fn resolve_cargo_lock_working_dir(path_str: &str) -> PathBuf {
    if path_str.is_empty() {
        return PathBuf::from(".");
    }
    let current = Path::new(path_str);
    if current.is_dir() && path_exists(current.join("Cargo.lock")) {
        return current.to_path_buf();
    }
    if let Some(parent) = current.parent() {
        if path_exists(parent.join("Cargo.lock")) {
            return parent.to_path_buf();
        }
        if let Some(grandparent) = parent.parent()
            && path_exists(grandparent.join("Cargo.lock"))
        {
            return grandparent.to_path_buf();
        }
    }
    PathBuf::from(".")
}

// ═══════════════════════════════════════════════════════════════
// Config File Detection
// ═══════════════════════════════════════════════════════════════

/// Check if directory contains a config file.
pub fn has_config_file(dir_path: &Path) -> bool {
    const CONFIG_NAMES: [&str; 6] = [
        ".eslintrc",
        ".prettierrc",
        "tsconfig.json",
        "pyproject.toml",
        "setup.cfg",
        ".flake8",
    ];
    crate::utility_filesystem_io::scan_directory(dir_path)
        .iter()
        .any(|path| {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            CONFIG_NAMES.contains(&name)
                || name.ends_with(".config.js")
                || name.ends_with(".config.ts")
        })
}

/// Check if a Cargo.toml exists and return its directory.
pub fn has_cargo_toml(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    if path.join("Cargo.toml").exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

/// Check if a Cargo.lock exists and return its directory.
pub fn has_cargo_lock(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    if path.join("Cargo.lock").exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

// ═══════════════════════════════════════════════════════════════
// Python Detection
// ═══════════════════════════════════════════════════════════════

/// Check if path contains Python files (recursive).
pub fn has_python_files_recursive(path: &Path) -> bool {
    if !path_exists(path) {
        return path.extension().map(|e| e == "py").unwrap_or(false);
    }
    if is_file(path) {
        return path.extension().map(|e| e == "py").unwrap_or(false);
    }
    has_py_in_dir_recursive(path)
}

fn has_py_in_dir_recursive(dir: &Path) -> bool {
    for path in crate::utility_filesystem_io::scan_directory(dir) {
        if path.is_dir() {
            if has_py_in_dir_recursive(&path) {
                return true;
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════
// Default Working Dir
// ═══════════════════════════════════════════════════════════════

/// Create default "." working directory.
pub fn default_working_dir(path: &Path) -> PathBuf {
    if path.exists() {
        path.to_path_buf()
    } else {
        PathBuf::from(".")
    }
}
