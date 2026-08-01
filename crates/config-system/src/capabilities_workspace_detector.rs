use async_trait::async_trait;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use shared::common::FilePath;
use shared::config_system::{IWorkspaceDetectorProtocol, WorkspaceType};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::PathBuf::from(&path.value);

        // Single-pass directory scan for marker files
        if let Some(lang) = self.check_dir_for_language(&path_buf) {
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

        // Walk up to 5 parent directories if no marker found
        let mut current = path_buf;
        let mut depth = 0;
        while !current.as_os_str().is_empty() && depth < 5 {
            if let Some(parent) = current.parent() {
                match parent.file_name().and_then(|n| n.to_str()) {
                    Some("modules") => return WorkspaceType::Python,
                    Some("packages") => return WorkspaceType::TypeScript,
                    Some("crates") => return WorkspaceType::Rust,
                    _ => {}
                }
            }
            if let Some(lang) = self.check_dir_for_language(&current) {
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
            .any(|dir| self.filesystem.path_exists(&root.join(dir)))
    }

    async fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath> {
        let root_path = std::path::Path::new(&root.value).to_path_buf();
        let fs = self.filesystem.clone();
        tokio::task::spawn_blocking(move || Self::scan_workspace_dirs_sync(&root_path, &*fs))
            .await
            .unwrap_or_default()
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
            filesystem: Arc::new(filesystem::FilesystemOrchestrator::new()),
        }
    }

    fn check_dir_for_language(&self, dir: &std::path::Path) -> Option<WorkspaceType> {
        let entries = self.filesystem.scan_directory(dir);
        if entries.is_empty() {
            return None;
        }
        let mut has_rust = false;
        let mut has_python = false;
        let mut has_typescript = false;
        for entry in &entries {
            let name = match entry.file_name().and_then(|n| n.to_str()) {
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

    fn collect_subdirs_sync(dir: &std::path::Path, fs: &dyn IFilesystemAggregate) -> Vec<FilePath> {
        let entries = fs.scan_directory(dir);
        entries
            .into_iter()
            .filter(|p| p.is_dir())
            .filter_map(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .collect()
    }

    fn scan_workspace_dirs_sync(
        root: &std::path::Path,
        fs: &dyn IFilesystemAggregate,
    ) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs_sync(root, fs);
        }

        if let Some(parent) = root.parent()
            && let Some(parent_name) = parent.file_name()
        {
            let parent_str = parent_name.to_string_lossy();
            if workspace_dirs.contains(&parent_str.as_ref())
                && let Ok(meta) = fs.metadata(root)
                && meta.is_dir()
                && let Ok(fp) = FilePath::new(root.to_string_lossy().to_string())
            {
                return vec![fp];
            }
        }

        let mut member_dirs: Vec<std::path::PathBuf> = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            if let Ok(meta) = fs.metadata(&dir_path)
                && meta.is_dir()
            {
                let entries = fs.scan_directory(&dir_path);
                for entry in entries {
                    if entry.is_dir() {
                        member_dirs.push(entry);
                    }
                }
            }
        }

        member_dirs
            .into_par_iter()
            .filter_map(|path| FilePath::new(path.to_string_lossy().to_string()).ok())
            .collect()
    }
}
