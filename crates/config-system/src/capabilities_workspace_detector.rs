use async_trait::async_trait;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use shared::common::FilePath;
use shared::config_system::{IWorkspaceDetectorProtocol, WorkspaceType};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WorkspaceDetector;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IWorkspaceDetectorProtocol for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::PathBuf::from(&path.value);

        // Single-pass directory scan for marker files (single syscall instead of up to 10)
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
            .any(|dir| filesystem::utility_filesystem_io::path_exists(root.join(dir)))
    }

    async fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath> {
        let root_path = std::path::Path::new(&root.value).to_path_buf();
        // FR-004: Move sync FS work off async runtime via spawn_blocking
        let path = root_path.clone();
        tokio::task::spawn_blocking(move || Self::scan_workspace_dirs_sync(&path))
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
        // Priority: Rust > Python > TypeScript (first match in scan order wins)
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

    fn collect_subdirs_sync(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();
        let entries = match std::fs::read_dir(dir) {
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
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
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

    /// FR-004: Synchronous workspace dir scanning — runs via spawn_blocking off async runtime.
    /// Uses rayon parallel iterator for concurrent filesystem operations.
    fn scan_workspace_dirs_sync(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs_sync(root);
        }

        if let Some(parent) = root.parent()
            && let Some(parent_name) = parent.file_name()
        {
            let parent_str = parent_name.to_string_lossy();
            if workspace_dirs.contains(&parent_str.as_ref())
                && let Ok(meta) = std::fs::metadata(root)
                    && meta.is_dir()
                        && let Ok(fp) = FilePath::new(root.to_string_lossy().to_string())
            {
                return vec![fp];
            }
        }

        // FR-004: Collect workspace member directories, then scan concurrently with rayon
        let mut member_dirs: Vec<std::path::PathBuf> = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            if let Ok(meta) = std::fs::metadata(&dir_path)
                && meta.is_dir()
            {
                if let Ok(entries) = std::fs::read_dir(&dir_path) {
                    for entry in entries.flatten() {
                        if let Ok(ft) = entry.file_type() {
                            if ft.is_dir() {
                                member_dirs.push(entry.path());
                            }
                        }
                    }
                } else {
                    eprintln!(
                        "Warning: Failed to read workspace dir '{}': {}",
                        dir_path.display(),
                        "I/O error"
                    );
                }
            }
        }

        // FR-004: Concurrent filesystem scan with rayon, bounded to 8 via rayon thread pool
        member_dirs
            .into_par_iter()
            .filter_map(|path| {
                if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                    Some(fp)
                } else {
                    None
                }
            })
            .collect()
    }
}
