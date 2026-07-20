// PURPOSE: naming filesystem utility — standalone functions for directory walking
//
// Utility layer: stateless, no contract, no I/O abstraction.
// Agent or Capabilities call these directly when low-level technical operations are needed.

use crate::common::taxonomy_path_utils_vo::PathUtils;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_common_vo::PatternList;

/// Walk directory recursively, returning all file paths (skipping ignored patterns).
pub fn walk_recursive(path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList {
    let root = std::path::Path::new(&path.value);
    let ignored_refs: Vec<&str> = match ignored_patterns {
        Some(p) => p.values.iter().map(|s| s.as_str()).collect(),
        None => Vec::new(),
    };
    let results = PathUtils::walk_recursive(root, &ignored_refs);
    FilePathList {
        values: results
            .into_iter()
            .filter_map(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .collect(),
    }
}
