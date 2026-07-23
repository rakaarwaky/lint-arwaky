use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::contract_workspace_detector_protocol::WorkspaceType;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::PathBuf::from(&path.value);

        // Batch directory scan for config files in the given path (single syscall instead of up to 10)
        if let Some(lang) = Self::check_dir_for_language(&path_buf) {
            return lang;
        }

        if let Some(parent) = path_buf.parent() {
            match parent.file_name().and_then(|n| n.to_str()) {
                Some("modules") => return WorkspaceType::Python,
                Some("packages") => return WorkspaceType::TypeScript,
                Some("crates") => return WorkspaceType::Rust,
                _ => {}
            }
        }

        let mut current = path_buf;
        let mut depth = 0;
        while !current.as_os_str().is_empty() && depth < 2 {
            if let Some(lang) = Self::check_dir_for_language(&current) {
                return lang;
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
            .any(|dir| shared::common::utility_file_handler::path_exists(root.join(dir)))
    }

    async fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath> {
        let root_path = std::path::Path::new(&root.value).to_path_buf();
        Self::scan_workspace_dirs(&root_path).await
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
        Self
    }

    fn check_dir_for_language(dir: &std::path::Path) -> Option<WorkspaceType> {
        let entries = std::fs::read_dir(dir).ok()?;
        let mut has_rust = false;
        let mut has_python = false;
        let mut has_typescript = false;
        for entry in entries.flatten() {
            let name = match entry.file_name().to_str() {
                Some(n) => n.to_lowercase(),
                None => continue,
            };
            match name.as_str() {
                "cargo.toml" => has_rust = true,
                "setup.py" | "pyproject.toml" | "requirements.txt" | "setup.cfg" => {
                    has_python = true
                }
                "package.json" | "tsconfig.json" => has_typescript = true,
                _ => {}
            }
        }
        // Priority: Rust > Python > TypeScript
        if has_rust {
            Some(WorkspaceType::Rust)
        } else if has_python {
            Some(WorkspaceType::Python)
        } else if has_typescript {
            Some(WorkspaceType::TypeScript)
        } else {
            None
        }
    }

    async fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();
        let mut entries = match tokio::fs::read_dir(dir).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read directory '{}': {}",
                    dir.display(),
                    e
                );
                return results;
            }
        };
        while let Some(entry) = match entries.next_entry().await {
            Ok(Some(e)) => Some(e),
            Ok(None) => None,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read directory entry in '{}': {}",
                    dir.display(),
                    e
                );
                None
            }
        } {
            if let Ok(ft) = entry.file_type().await {
                if ft.is_dir() {
                    let sub = entry.path();
                    if let Ok(fp) = FilePath::new(sub.to_string_lossy().to_string()) {
                        results.push(fp);
                    }
                }
            }
        }
        results
    }

    async fn scan_workspace_dirs(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs(root).await;
        }

        if let Some(parent) = root.parent() {
            if let Some(parent_name) = parent.file_name() {
                let parent_str = parent_name.to_string_lossy();
                if workspace_dirs.contains(&parent_str.as_ref()) {
                    if let Ok(meta) = tokio::fs::metadata(root).await {
                        if meta.is_dir() {
                            if let Ok(fp) = FilePath::new(root.to_string_lossy().to_string()) {
                                return vec![fp];
                            }
                        }
                    }
                }
            }
        }

        let mut results = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            if let Ok(meta) = tokio::fs::metadata(&dir_path).await {
                if meta.is_dir() {
                    results.extend(Self::collect_subdirs(&dir_path).await);
                }
            }
        }
        results
    }
}
