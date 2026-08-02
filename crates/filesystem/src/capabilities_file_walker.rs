// FR-001: File Discovery
// Produces: Vec<FilePath> (lightweight) or Vec<FileEntry> (full)
// Consumers: naming-rules (lightweight), code-analysis (full)
//
// Capabilities: struct FileWalker — implements IFileWalkerProtocol
// Utility functions: is_ignored, filter by extension

use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_filesystem_protocol::IFileWalkerProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::path::{Path, PathBuf};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileWalker;

impl FileWalker {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IFileWalkerProtocol for FileWalker {
    fn walk(&self, root: &Path, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry> {
        self.discover_entries(root, ignored, extensions)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl FileWalker {
    /// Lightweight mode: discover source file paths only.
    /// Consumer: naming-rules
    pub fn discover_paths(
        &self,
        root: &Path,
        ignored: &[String],
        extensions: &[&str],
    ) -> Vec<FilePath> {
        let mut files = Vec::new();
        let builder = build_walker(root);
        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };
            if entry.file_type().is_some_and(|ft| ft.is_dir()) {
                continue;
            }
            let path = entry.path();
            if !matches_extension(path, extensions) {
                continue;
            }
            let rel = path.strip_prefix(root).unwrap_or(path);
            if is_ignored(&rel.to_string_lossy(), ignored) {
                continue;
            }
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        files
    }

    /// Full mode: discover source files with content.
    /// Consumer: code-analysis
    pub fn discover_entries(
        &self,
        root: &Path,
        ignored: &[String],
        extensions: &[&str],
    ) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        let builder = build_walker(root);
        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };
            if entry.file_type().is_some_and(|ft| ft.is_dir()) {
                continue;
            }
            let path = entry.path();
            if !matches_extension(path, extensions) {
                continue;
            }
            let rel = path.strip_prefix(root).unwrap_or(path);
            if is_ignored(&rel.to_string_lossy(), ignored) {
                continue;
            }
            let language = match Language::from_extension(
                path.extension().and_then(|e| e.to_str()).unwrap_or(""),
            ) {
                Some(l) => l,
                None => continue,
            };
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let content = std::fs::read_to_string(path).unwrap_or_default();

            entries.push(FileEntry {
                path: path.to_path_buf(),
                extension: path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string(),
                language,
                size,
                content,
                parse_ok: false,
                parse_metadata: None,
            });
        }
        entries
    }
}

impl Default for FileWalker {
    fn default() -> Self {
        Self::new()
    }
}

/// Build ignore::WalkBuilder with workspace subdirs.
fn build_walker(root: &Path) -> ignore::WalkBuilder {
    let workspace_subdirs = ["crates", "packages", "modules"];
    let found: Vec<PathBuf> = workspace_subdirs
        .iter()
        .map(|s| root.join(s))
        .filter(|p| p.is_dir())
        .collect();

    let mut builder = if !found.is_empty() {
        let mut b = ignore::WalkBuilder::new(&found[0]);
        for sub in &found[1..] {
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
    builder
}

/// Check if path matches any of the allowed extensions.
fn matches_extension(path: &Path, extensions: &[&str]) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| extensions.contains(&ext))
}

/// Check if a relative path should be ignored based on patterns.
pub fn is_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        if let Some(stripped) = pat.strip_prefix('/') {
            if !stripped.is_empty() && rel_path.contains(stripped) {
                return true;
            }
        } else if rel_path.contains(pat.as_str()) {
            return true;
        }
    }
    false
}
