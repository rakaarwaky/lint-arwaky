// FR-005: Workspace Detection — Capabilities layer
// Implements IWorkspaceProtocol by delegating to utility_workspace_detection stateless functions.
// 3-block structure per AES skill.

use crate::utility_filesystem_io;
use crate::utility_workspace_detection;
use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use std::path::{Path, PathBuf};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesWorkspace;

impl CapabilitiesWorkspace {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IWorkspaceProtocol for CapabilitiesWorkspace {
    fn workspace_root(&self, start: &FilePath) -> Option<PathBuf> {
        utility_workspace_detection::find_workspace_root(&start.value)
    }

    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error> {
        utility_workspace_detection::find_workspace_root_from_path(start)
    }

    fn is_member_path(&self, path: &FilePath) -> bool {
        utility_workspace_detection::is_member_path(&path.value)
    }

    fn is_leaf_member_path(&self, path: &FilePath) -> bool {
        utility_workspace_detection::is_leaf_member_path(&path.value)
    }

    fn detect_source_dir(&self, project_root: &Path) -> PathBuf {
        utility_workspace_detection::detect_source_dir(project_root)
    }

    fn detect_language_from_path(&self, path: &str) -> ConfigLanguage {
        utility_workspace_detection::detect_language_from_path(path)
    }

    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &[String]) -> bool {
        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);
            if dir.is_dir() && check_dir_containers(&dir, identifiers) {
                return true;
            }
        }
        false
    }

    fn resolve_orphan_module_path(
        &self,
        root: &Path,
        base_dir: &Path,
        module_path: &str,
    ) -> Option<PathBuf> {
        let candidate = if Path::new(module_path).is_absolute() {
            PathBuf::from(module_path)
        } else {
            base_dir.join(module_path)
        };
        confine_under_root(root, &candidate)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for CapabilitiesWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Private Helpers ──────────────────────────────────────

fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = utility_filesystem_io::canonicalize(root).ok()?;
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };
    if let Ok(canonical_candidate) = utility_filesystem_io::canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }
    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;
    let canonical_parent = utility_filesystem_io::canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);
    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}

fn check_dir_containers(dir: &Path, identifiers: &[String]) -> bool {
    for path in utility_filesystem_io::scan_directory(dir) {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if matches!(
            name,
            "target"
                | ".git"
                | "node_modules"
                | "dist"
                | "build"
                | "__pycache__"
                | ".venv"
                | "tests"
        ) {
            continue;
        }
        if path.is_dir() && check_dir_containers(&path, identifiers) {
            return true;
        } else if (name.ends_with("_container.rs")
            || name.ends_with("_container.py")
            || name.ends_with("_container.ts")
            || name.ends_with("_entry.rs")
            || name.ends_with("_entry.py")
            || name.ends_with("_entry.ts"))
            && let Ok(content) = utility_filesystem_io::read_to_string(&path)
        {
            for id in identifiers {
                if content.contains(id) {
                    return true;
                }
            }
        }
    }
    false
}
