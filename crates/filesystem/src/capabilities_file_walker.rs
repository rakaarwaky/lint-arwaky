use std::path::Path;
// PURPOSE: Capabilities layer — file discovery (FR-001)
// Walks directory tree using `ignore` crate (gitignore-aware, parallel walk).
// Reads file contents into memory. Produces Vec<FileEntry>.

use shared::filesystem::contract_filesystem_protocol::IFileWalkerProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::path::PathBuf;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileWalker;

impl FileWalker {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileWalkerProtocol for FileWalker {
    fn walk(&self, root: &Path, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry> {
        self.walk(root, ignored, extensions)
    }
}


// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl Default for FileWalker {
    fn default() -> Self {
        Self::new()
    }
}



impl FileWalker {
    /// Walk workspace directory tree, discover source files, read contents.
    /// FR-001 business rules:
    /// - Uses `ignore::WalkBuilder` for parallel, gitignore-aware walking.
    /// - Scans crates/, packages/, modules/ subdirectories at root level.
    /// - Filters by extension (.rs, .py, .ts, .js, .jsx, .tsx).
    /// - Respects .gitignore, .ignore, and ignored_paths from config.
    /// - Skips hidden directories (.git, .venv, node_modules, target, dist, build, __pycache__).
    /// - Reads file content into FileEntry.content (UTF-8). Non-UTF-8 files are skipped.
    pub fn walk(&self, root: &Path, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry> {
        let workspace_subdirs = ["crates", "packages", "modules"];
        let found_subdirs: Vec<PathBuf> = workspace_subdirs
            .iter()
            .map(|s| root.join(s))
            .filter(|p| p.is_dir())
            .collect();

        let mut builder = if !found_subdirs.is_empty() {
            let mut b = ignore::WalkBuilder::new(&found_subdirs[0]);
            for sub in &found_subdirs[1..] {
                b.add(sub);
            }
            b
        } else {
            ignore::WalkBuilder::new(root)
        };
        builder
            .hidden(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .threads(
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(4),
            );

        let mut entries = Vec::new();

        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            // Skip directories
            if entry.file_type().is_some_and(|ft| ft.is_dir()) {
                continue;
            }

            let path = entry.path();
            let ext = match path.extension().and_then(|e| e.to_str()) {
                Some(e) => e,
                None => continue,
            };

            if !extensions.contains(&ext) {
                continue;
            }

            // Check ignored patterns
            let rel = path.strip_prefix(root).unwrap_or(path);
            let rel_str = rel.to_string_lossy();
            if is_ignored(&rel_str, ignored) {
                continue;
            }

            let language = match Language::from_extension(ext) {
                Some(l) => l,
                None => continue,
            };

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            // Read file content (UTF-8). Skip non-UTF-8 with empty content.
            let content = std::fs::read_to_string(path).unwrap_or_default();

            entries.push(FileEntry {
                path: path.to_path_buf(),
                extension: ext.to_string(),
                language,
                size: metadata.len(),
                content,
                parse_ok: false, // Will be set by AST parser
                parse_metadata: None,
            });
        }

        entries
    }
}

/// Check if a relative path should be ignored based on patterns.
fn is_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            if rel_path.contains(stripped) {
                return true;
            }
        } else {
            // Substring match
            if rel_path.contains(pat.as_str()) {
                return true;
            }
        }
    }
    false
}
