// PURPOSE: taxonomy_target_utility — pure utility functions for path resolution and source detection
use crate::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use crate::common::utility_file_handler::walk_source_files;
use std::path::Path;

/// Resolve target path: normalize "crates" → parent, keep "." as-is, etc.
pub fn resolve_target(path: Option<String>) -> String {
    match path {
        Some(p) => p,
        None => ".".to_string(),
    }
}

/// Detect source directory from project root (packages/, crates/, modules/).
/// If the path itself contains source files, return it directly.
pub fn detect_source_dir(project_root: &Path) -> std::path::PathBuf {
    // If the path itself contains .rs/.py/.ts files, it's already a source directory
    if has_source_files(project_root) {
        return project_root.to_path_buf();
    }
    for name in &["packages", "crates", "modules"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.to_path_buf()
}

/// Check if a directory contains source files directly (not in subdirectories)
fn has_source_files(dir: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".rs") || name.ends_with(".py")
                    || name.ends_with(".ts") || name.ends_with(".js")
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Collect source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &Path,
    _dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    let mut files = Vec::new();
    if root_dir.is_dir() {
        walk_source_files(root_dir, &mut files, ignored);
    }
    files
}
