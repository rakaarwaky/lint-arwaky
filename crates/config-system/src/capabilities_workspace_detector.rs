use shared::common::FilePath;
use shared::config_system::{IWorkspaceDetectorProtocol, WorkspaceType};

// PURPOSE: WorkspaceDetector — detects workspace type from marker files
// Maps ConfigLanguage ↔ WorkspaceType and adds discover_workspace_members

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector;

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
    pub fn new() -> Self {
        Self
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
/// FR-003: Walks up parent directories looking for marker files and workspace dir names.
/// Limits ancestor name matching to 2 levels (BF-4), but walks further for marker files.
fn has_rust_markers(path: &std::path::Path) -> bool {
    let marker_files = ["Cargo.toml"];
    let workspace_name = "crates";
    let mut current = Some(path);
    let mut levels = 0;

    while let Some(p) = current {
        if levels == 0 && dir_has_any_file(p, &marker_files) {
            return true;
        }
        if levels > 0 && levels <= 2 {
            // BF-4: Only check workspace dir name at parent/grandparent level
            if p.file_name().map_or(false, |n| n == workspace_name) {
                return true;
            }
        }
        if levels > 0 && dir_has_any_file(p, &marker_files) {
            return true;
        }
        levels += 1;
        if levels > 5 {
            break;
        }
        current = p.parent();
    }
    false
}

fn has_python_markers(path: &std::path::Path) -> bool {
    let marker_files = [
        "pyproject.toml",
        "setup.py",
        "requirements.txt",
        "__init__.py",
    ];
    let workspace_name = "modules";
    let mut current = Some(path);
    let mut levels = 0;

    while let Some(p) = current {
        if levels == 0 && dir_has_any_file(p, &marker_files) {
            return true;
        }
        if levels > 0 && levels <= 2 {
            if p.file_name().map_or(false, |n| n == workspace_name) {
                return true;
            }
        }
        if levels > 0 && dir_has_any_file(p, &marker_files) {
            return true;
        }
        levels += 1;
        if levels > 5 {
            break;
        }
        current = p.parent();
    }
    false
}

fn has_typescript_markers(path: &std::path::Path) -> bool {
    let marker_files = ["package.json", "tsconfig.json"];
    let workspace_name = "packages";
    let mut current = Some(path);
    let mut levels = 0;

    while let Some(p) = current {
        if levels == 0 && dir_has_any_file(p, &marker_files) {
            return true;
        }
        if levels > 0 && levels <= 2 {
            if p.file_name().map_or(false, |n| n == workspace_name) {
                return true;
            }
        }
        if levels > 0 && dir_has_any_file(p, &marker_files) {
            return true;
        }
        levels += 1;
        if levels > 5 {
            break;
        }
        current = p.parent();
    }
    false
}
