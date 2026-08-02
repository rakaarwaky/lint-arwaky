use shared::common::FilePath;
use shared::common::taxonomy_config_language_vo::ConfigLanguage;
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

        // Check for actual markers — filesystem defaults to Rust, we need Unknown
        if has_rust_markers(&path_buf) {
            return WorkspaceType::Rust;
        }
        if has_python_markers(&path_buf) {
            return WorkspaceType::Python;
        }
        if has_typescript_markers(&path_buf) {
            return WorkspaceType::TypeScript;
        }

        // Delegate to filesystem aggregate for parent directory name checks (crates/packages/modules)
        let lang = self.filesystem.detect_language_from_path(&path.value);
        match lang {
            ConfigLanguage::Rust if has_rust_markers(&path_buf) => WorkspaceType::Rust,
            ConfigLanguage::Python if has_python_markers(&path_buf) => WorkspaceType::Python,
            ConfigLanguage::TypeScript if has_typescript_markers(&path_buf) => {
                WorkspaceType::TypeScript
            }
            _ => WorkspaceType::Unknown,
        }
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

fn has_rust_markers(path: &std::path::Path) -> bool {
    path.join("Cargo.toml").exists()
        || path
            .components()
            .any(|c| matches!(c, std::path::Component::Normal(name) if name == "crates"))
}

fn has_python_markers(path: &std::path::Path) -> bool {
    path.join("pyproject.toml").exists()
        || path.join("setup.py").exists()
        || path.join("requirements.txt").exists()
        || path.join("__init__.py").exists()
        || path
            .components()
            .any(|c| matches!(c, std::path::Component::Normal(name) if name == "modules"))
}

fn has_typescript_markers(path: &std::path::Path) -> bool {
    path.join("package.json").exists()
        || path.join("tsconfig.json").exists()
        || path
            .components()
            .any(|c| matches!(c, std::path::Component::Normal(name) if name == "packages"))
}
