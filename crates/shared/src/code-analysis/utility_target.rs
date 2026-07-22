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
pub fn detect_source_dir(project_root: &Path) -> std::path::PathBuf {
    for name in &["packages", "crates", "modules"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.to_path_buf()
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
