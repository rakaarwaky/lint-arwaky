// PURPOSE: WorkspaceDetector — IWorkspaceDetectorPort implementation for workspace type detection
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::contract_workspace_detector_port::WorkspaceType;

pub struct WorkspaceDetector;

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

impl IWorkspaceDetectorPort for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::Path::new(&path.value).to_path_buf();

        // 1. Check for explicit language markers in the workspace directory itself
        if path_buf.join("Cargo.toml").exists() {
            return WorkspaceType::Rust;
        }
        if path_buf.join("package.json").exists() {
            return WorkspaceType::TypeScript;
        }
        if path_buf.join("pyproject.toml").exists()
            || path_buf.join("setup.py").exists()
            || path_buf.join("requirements.txt").exists()
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

        // 3. Walk up parent chain looking for config files (fallback)
        let mut current = path_buf;
        while !current.as_os_str().is_empty() {
            if current.join("Cargo.toml").exists() {
                return WorkspaceType::Rust;
            }
            if current.join("package.json").exists() {
                return WorkspaceType::TypeScript;
            }
            if current.join("pyproject.toml").exists()
                || current.join("setup.py").exists()
                || current.join("requirements.txt").exists()
            {
                return WorkspaceType::Python;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        WorkspaceType::Unknown
    }

    fn is_workspace(&self, path: &FilePath) -> bool {
        let root = std::path::Path::new(&path.value);
        ["crates", "packages", "modules"]
            .iter()
            .any(|dir| root.join(dir).is_dir())
    }
}
