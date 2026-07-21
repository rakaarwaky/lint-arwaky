use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::contract_workspace_detector_protocol::WorkspaceType;
use shared::config_system::utility_config_io as config_io;

// PURPOSE: WorkspaceDetector — IWorkspaceDetectorProtocol implementation for workspace type detection
use shared::common::taxonomy_path_vo::FilePath;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::PathBuf::from(&path.value);

        // 1. Check for explicit language markers in the workspace directory itself
        if config_io::path_exists(path_buf.join("Cargo.toml")) {
            return WorkspaceType::Rust;
        }
        if config_io::path_exists(path_buf.join("package.json")) {
            return WorkspaceType::TypeScript;
        }
        if config_io::path_exists(path_buf.join("pyproject.toml"))
            || config_io::path_exists(path_buf.join("setup.py"))
            || config_io::path_exists(path_buf.join("requirements.txt"))
        {
            return WorkspaceType::Python;
        }

        // 2. Check parent workspace folder context (crates/ → Rust, packages/ → TS, modules/ → Python)
        // This handles multi-language root dirs (e.g. test-workspaces/ which has all three).
        if let Some(parent) = path_buf.parent() {
            match parent.file_name().and_then(|n| n.to_str()) {
                Some("modules") => return WorkspaceType::Python,
                Some("packages") => return WorkspaceType::TypeScript,
                Some("crates") => return WorkspaceType::Rust,
                _ => {}
            }
        }

        // 3. Walk up parent chain looking for config files (fallback, max 2 levels)
        let mut current = path_buf;
        let mut depth = 0;
        while !current.as_os_str().is_empty() && depth < 2 {
            if config_io::path_exists(current.join("Cargo.toml")) {
                return WorkspaceType::Rust;
            }
            if config_io::path_exists(current.join("package.json")) {
                return WorkspaceType::TypeScript;
            }
            if config_io::path_exists(current.join("pyproject.toml"))
                || config_io::path_exists(current.join("setup.py"))
                || config_io::path_exists(current.join("requirements.txt"))
            {
                return WorkspaceType::Python;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
                depth += 1;
            } else {
                break;
            }
        }

        WorkspaceType::Unknown
    }

    fn is_workspace(&self, path: &FilePath) -> bool {
        let root = std::path::PathBuf::from(&path.value);
        ["crates", "packages", "modules"]
            .iter()
            .any(|dir| config_io::path_exists(root.join(dir)))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl WorkspaceDetector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkspaceDetector {
    fn default() -> Self {
        Self::new()
    }
}
