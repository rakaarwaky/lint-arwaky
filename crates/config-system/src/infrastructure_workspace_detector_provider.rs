// PURPOSE: WorkspaceDetector — IWorkspaceDetectorPort implementation for workspace type detection
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::contract_workspace_detector_port::WorkspaceType;
use shared::source_parsing::taxonomy_path_vo::FilePath;

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
        let mut current = std::path::Path::new(&path.value).to_path_buf();

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

        // Check workspace folder structure (fallback for root detection)
        let root = std::path::Path::new(&path.value);
        let has_workspace_folder = ["crates", "packages", "modules"]
            .iter()
            .any(|dir| root.join(dir).is_dir());

        if !has_workspace_folder {
            return WorkspaceType::Unknown;
        }

        // Check config files inside workspace folders
        for dir in &["crates", "packages", "modules"] {
            let dir_path = root.join(dir);
            if !dir_path.is_dir() {
                continue;
            }

            if dir_path.join("Cargo.toml").exists() {
                return WorkspaceType::Rust;
            }
            if dir_path.join("package.json").exists() {
                return WorkspaceType::TypeScript;
            }
            if dir_path.join("pyproject.toml").exists() {
                return WorkspaceType::Python;
            }

            // Scan subdirectories for config files
            if let Ok(entries) = std::fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    let sub = entry.path();
                    if sub.is_dir() {
                        if sub.join("Cargo.toml").exists() {
                            return WorkspaceType::Rust;
                        }
                        if sub.join("package.json").exists() {
                            return WorkspaceType::TypeScript;
                        }
                        if sub.join("pyproject.toml").exists() {
                            return WorkspaceType::Python;
                        }
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_detection_concept() {
        assert_eq!(WorkspaceType::Rust.as_str(), "rust");
        assert_eq!(WorkspaceType::TypeScript.as_str(), "typescript");
        assert_eq!(WorkspaceType::Python.as_str(), "python");
        assert_eq!(WorkspaceType::Unknown.as_str(), "unknown");
    }
}
