// PURPOSE: taxonomy_external_lint_helper — shared utility functions for external linter adapters
// Pure functions: resolve working directories, canonicalize paths,
// execute commands with error mapping. Used by JS, Python, and RS adapters.

use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;
use std::path::{Path, PathBuf};

/// Canonicalize a path string, falling back to the original on error.
pub fn canonicalize_path(path_str: &str) -> String {
    match std::fs::canonicalize(path_str) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}

/// Execute a command, mapping execution failures to `LinterOperationError::Scan`.
pub async fn exec_cmd_scan(
    executor: &dyn ICommandExecutorPort,
    args: Vec<String>,
    working_dir: FilePath,
    timeout_secs: f64,
    adapter_name: Option<AdapterName>,
    path: &FilePath,
) -> Result<ResponseData, LinterOperationError> {
    executor
        .execute_command(
            PatternList::new(args),
            working_dir,
            Some(Timeout::new(timeout_secs)),
        )
        .await
        .map_err(|e| {
            LinterOperationError::Scan(ScanError {
                path: path.clone(),
                message: ErrorMessage::new(e.to_string()),
                error_code: None,
                adapter_name,
                cause: None,
            })
        })
}

/// Execute a command, mapping execution failures to `LinterOperationError::Adapter`.
pub async fn exec_cmd_adapter(
    executor: &dyn ICommandExecutorPort,
    args: Vec<String>,
    working_dir: FilePath,
    timeout_secs: f64,
    adapter_name: AdapterName,
) -> Result<ResponseData, LinterOperationError> {
    executor
        .execute_command(
            PatternList::new(args),
            working_dir,
            Some(Timeout::new(timeout_secs)),
        )
        .await
        .map_err(|e| {
            LinterOperationError::Adapter(AdapterError::new(
                adapter_name,
                ErrorMessage::new(e.to_string()),
            ))
        })
}

/// Create a default `"."` working directory, falling back to the given path if it fails.
pub fn default_working_dir(path: &FilePath) -> FilePath {
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// Applies a JS tool's fix command, returning `Ok(ComplianceStatus::new(true))` on success.
/// Combines resolve_js_working_dir + canonicalize_path + resolve_js_cmd + exec_cmd_adapter.
pub async fn js_apply_fix(
    executor: &dyn ICommandExecutorPort,
    path: &FilePath,
    tool: &str,
    fix_arg: &str,
) -> Result<ComplianceStatus, LinterOperationError> {
    let wd = resolve_js_working_dir(path);
    let abs_path = canonicalize_path(&path.value);
    let cmd = resolve_js_cmd(tool, vec![abs_path, fix_arg.to_string()], &wd.value);
    let response = exec_cmd_adapter(executor, cmd, wd, 60.0, AdapterName::raw(tool)).await?;
    Ok(ComplianceStatus::new(response.returncode == 0))
}

/// No-op apply_fix for linters that cannot auto-fix (scanners, type-checkers).
pub async fn noop_apply_fix() -> Result<ComplianceStatus, LinterOperationError> {
    Ok(ComplianceStatus::new(false))
}

/// Return true if the given path contains any Python (`.py`) files.
///
/// Recursively walks directories, short-circuiting at the first `.py` file found.
/// If `path` itself is a `.py` file, returns true immediately.
pub fn has_python_files(path: &FilePath) -> bool {
    let p = std::path::Path::new(&path.value);
    if !p.exists() {
        return false;
    }
    if p.is_file() {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    // Directory walk — short-circuit at first .py file
    has_py_in_dir(p)
}

fn has_py_in_dir(dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
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
/// Prefers local node_modules/.bin over npx/bunx.
pub fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);
    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }
    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

/// Walk up from the given path to find the JS project root
/// (detected by lint_arwaky.config*.yaml, package.json, or .git directory).
pub fn resolve_js_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            abs_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            abs_path.clone()
        };
        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.toml (for cargo fmt, cargo clippy).
pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.toml").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.toml").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.toml").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }
    FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.lock (for cargo-audit).
pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.lock").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.lock").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.lock").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }
    FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
}

fn is_bun_available() -> bool {
    std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
