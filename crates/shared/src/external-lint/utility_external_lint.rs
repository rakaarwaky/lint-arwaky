// PURPOSE: utility_external_lint_helper — pure utility functions for external linter adapters
// No contract imports — only taxonomy types allowed in utility layer.

use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::utility_filesystem_io;
use std::path::{Path, PathBuf};

use crate::code_analysis::taxonomy_operation_error::LinterOperationError;

/// Canonicalize a path string, falling back to the original on error.
pub fn canonicalize_path(path_str: &str) -> String {
    utility_filesystem_io::canonicalize_path(path_str)
        .to_string_lossy()
        .to_string()
}

/// Create a default `"."` working directory, falling back to the given path if it fails.
pub fn default_working_dir(path: &FilePath) -> FilePath {
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// No-op apply_fix for linters that cannot auto-fix (scanners, type-checkers).
pub async fn noop_apply_fix() -> Result<ComplianceStatus, LinterOperationError> {
    Ok(ComplianceStatus::new(false))
}

/// Return true if the given path contains any Python (`.py`) files.
pub fn has_python_files(path: &FilePath) -> bool {
    let p = std::path::Path::new(&path.value);
    if !utility_filesystem_io::path_exists(p) {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    if utility_filesystem_io::is_file(p) {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    has_py_in_dir(p)
}

fn has_py_in_dir(dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if utility_filesystem_io::is_dir(&path) {
            if has_py_in_dir(&path) {
                return true;
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            return true;
        }
    }
    false
}

/// Resolve the executable command for a JS tool (eslint, prettier, tsc).
/// Only uses local binary from node_modules/.bin — never falls back to npx/bunx.
/// Returns None if the tool is not installed locally.
pub fn resolve_js_cmd(
    executable: &str,
    args: Vec<String>,
    working_dir: &str,
) -> Option<Vec<String>> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);
    if utility_filesystem_io::path_exists(&local_bin) {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return Some(cmd);
    }
    None
}

/// Walk up from the given path to find the JS project root.
pub fn resolve_js_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    let abs_path = utility_filesystem_io::canonicalize_path(path_str);
    let mut current = if utility_filesystem_io::is_file(&abs_path) {
        abs_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        abs_path.clone()
    };
    for _ in 0..10 {
        if utility_filesystem_io::is_file(&current.join("lint_arwaky.config.yaml"))
            || utility_filesystem_io::is_file(&current.join("lint_arwaky.config.python.yaml"))
            || utility_filesystem_io::is_file(&current.join("package.json"))
            || utility_filesystem_io::is_dir(&current.join(".git"))
        {
            return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }
    FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.toml (for cargo fmt, cargo clippy).
pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if utility_filesystem_io::is_dir(current) {
        if utility_filesystem_io::path_exists(&current.join("Cargo.toml")) {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if utility_filesystem_io::path_exists(&parent.join("Cargo.toml")) {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent()
            && utility_filesystem_io::path_exists(&grandparent.join("Cargo.toml"))
        {
            return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
    }
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// Find parent dir with Cargo.lock (for cargo-audit).
pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if utility_filesystem_io::is_dir(current) {
        if utility_filesystem_io::path_exists(&current.join("Cargo.lock")) {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if utility_filesystem_io::path_exists(&parent.join("Cargo.lock")) {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent()
            && utility_filesystem_io::path_exists(&grandparent.join("Cargo.lock"))
        {
            return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
    }
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}
