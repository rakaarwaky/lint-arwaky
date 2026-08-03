use shared::common::FilePath;
use shared::config_system::{IWorkspaceDetectorProtocol, WorkspaceType};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// PURPOSE: WorkspaceDetector — thin wrapper around filesystem's IFilesystemAggregate
// Maps ConfigLanguage ↔ WorkspaceType and adds discover_workspace_members

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::PathBuf::from(&path.value);

        // BF-3: Check marker files directly — no unreachable fallback arms
        if has_rust_markers(&path_buf) {
            return WorkspaceType::Rust;
        }
        if has_python_markers(&path_buf) {
            return WorkspaceType::Python;
        }
        if has_typescript_markers(&path_buf) {
            return WorkspaceType::TypeScript;
        }

        WorkspaceType::Unknown
    }

    fn is_workspace(&self, path: &FilePath) -> bool {
        let root = std::path::PathBuf::from(&path.value);
        ["crates", "packages", "modules"]
            .iter()
            .any(|dir| root.join(dir).is_dir())
    }

    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath> {
        let root_path = std::path::Path::new(&root.value);
        let mut members = Vec::new();

        // If root IS a workspace dir (crates/packages/modules), scan its children
        if let Some(name) = root_path.file_name().and_then(|n| n.to_str())
            && matches!(name, "crates" | "packages" | "modules")
        {
            if let Ok(entries) = std::fs::read_dir(root_path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir()
                        && let Ok(fp) = FilePath::new(entry.path().to_string_lossy().to_string())
                    {
                        members.push(fp);
                    }
                }
            }
            return members;
        }

        // FR-004: If root's parent is a workspace directory, return root as single member
        if let Some(parent) = root_path.parent()
            && let Some(parent_name) = parent.file_name().and_then(|n| n.to_str())
            && matches!(parent_name, "crates" | "packages" | "modules")
        {
            members.push(root.to_owned());
            return members;
        }

        // Otherwise scan crates/, packages/, modules/ under root
        for dir_name in &["crates", "packages", "modules"] {
            let dir = root_path.join(dir_name);
            if !dir.is_dir() {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir()
                        && let Ok(fp) = FilePath::new(entry.path().to_string_lossy().to_string())
                    {
                        members.push(fp);
                    }
                }
            }
        }

        members
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl WorkspaceDetector {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}

/// TR-1: Single-pass directory scan — use one `read_dir` syscall per directory
/// instead of N `exists()` calls. Returns true if any of the target filenames
/// exist in the given directory.
fn dir_has_any_file(dir: &std::path::Path, targets: &[&str]) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if targets.contains(&name) {
                    return true;
                }
            }
        }
    }
    false
}

/// BF-4: Limit parent-dir matching to direct parent/grandparent, not arbitrary ancestors.
/// FR-003: Walks up to 2 parent directories if no marker found at target path.
fn has_rust_markers(path: &std::path::Path) -> bool {
    // Check the target path itself
    if dir_has_any_file(path, &["Cargo.toml"]) {
        return true;
    }
    // FR-003: Walk up to 2 parent directories
    if let Some(parent) = path.parent() {
        if dir_has_any_file(parent, &["Cargo.toml"]) {
            return true;
        }
        // Direct parent name match: crates/ → Rust
        if parent.file_name().map_or(false, |n| n == "crates") {
            return true;
        }
        if let Some(grandparent) = parent.parent() {
            if dir_has_any_file(grandparent, &["Cargo.toml"]) {
                return true;
            }
        }
    }
    false
}

fn has_python_markers(path: &std::path::Path) -> bool {
    if dir_has_any_file(
        path,
        &[
            "pyproject.toml",
            "setup.py",
            "requirements.txt",
            "__init__.py",
        ],
    ) {
        return true;
    }
    if let Some(parent) = path.parent() {
        if dir_has_any_file(
            parent,
            &[
                "pyproject.toml",
                "setup.py",
                "requirements.txt",
                "__init__.py",
            ],
        ) {
            return true;
        }
        // Direct parent name match: modules/ → Python
        if parent.file_name().map_or(false, |n| n == "modules") {
            return true;
        }
        if let Some(grandparent) = parent.parent() {
            if dir_has_any_file(
                grandparent,
                &[
                    "pyproject.toml",
                    "setup.py",
                    "requirements.txt",
                    "__init__.py",
                ],
            ) {
                return true;
            }
        }
    }
    false
}

fn has_typescript_markers(path: &std::path::Path) -> bool {
    if dir_has_any_file(path, &["package.json", "tsconfig.json"]) {
        return true;
    }
    if let Some(parent) = path.parent() {
        if dir_has_any_file(parent, &["package.json", "tsconfig.json"]) {
            return true;
        }
        // Direct parent name match: packages/ → TypeScript
        if parent.file_name().map_or(false, |n| n == "packages") {
            return true;
        }
        if let Some(grandparent) = parent.parent() {
            if dir_has_any_file(grandparent, &["package.json", "tsconfig.json"]) {
                return true;
            }
        }
    }
    false
}
