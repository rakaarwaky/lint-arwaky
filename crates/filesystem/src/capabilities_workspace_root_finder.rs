// FR-005: Workspace Detection — Capabilities layer
// Implements IWorkspaceProtocol by delegating to utility_workspace_detection stateless functions.
// 3-block structure per AES skill.

use crate::utility_workspace_detection;
use shared::common::taxonomy_common_vo::PatternList;
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

    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &PatternList) -> bool {
        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);
            if dir.is_dir() && utility_workspace_detection::check_dir_containers(&dir, identifiers.values())
            {
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
        utility_workspace_detection::confine_under_root(root, &candidate)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for CapabilitiesWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
