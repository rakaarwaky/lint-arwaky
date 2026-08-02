use shared::common::FilePath;
use shared::config_system::{IWorkspaceDetectorProtocol, WorkspaceType};
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use std::sync::Arc;

// PURPOSE: WorkspaceDetector — thin wrapper around filesystem's IWorkspaceProtocol
// Maps ConfigLanguage ↔ WorkspaceType and adds discover_workspace_members

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector {
    workspace_protocol: Arc<dyn IWorkspaceProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let lang = self
            .workspace_protocol
            .detect_language_from_path(&path.value);
        WorkspaceType::from(lang)
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

        for dir_name in &["crates", "packages", "modules"] {
            let dir = root_path.join(dir_name);
            if !dir.is_dir() {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Ok(fp) = FilePath::new(entry.path().to_string_lossy().to_string()) {
                            members.push(fp);
                        }
                    }
                }
            }
        }

        members
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for WorkspaceDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceDetector {
    pub fn new() -> Self {
        Self {
            workspace_protocol: Arc::new(
                filesystem::capabilities_workspace::CapabilitiesWorkspace::new(),
            ),
        }
    }

    pub fn with_workspace_protocol(protocol: Arc<dyn IWorkspaceProtocol>) -> Self {
        Self {
            workspace_protocol: protocol,
        }
    }
}
