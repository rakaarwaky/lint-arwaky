// PURPOSE: naming filesystem utility — standalone functions for directory walking
//
// Utility layer: stateless, no contract, no I/O abstraction.
// Agent or Capabilities call these directly when low-level technical operations are needed.

use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::utility_file_handler::walk_source_files;
use crate::taxonomy_common_vo::PatternList;
use std::path::Path;

/// Walk directory recursively, returning all source file paths (skipping ignored patterns).
pub fn walk_recursive(path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList {
    let root = Path::new(&path.value);
    let mut files = Vec::new();
    if root.is_dir() {
        let ignored: Vec<String> = match ignored_patterns {
            Some(p) => p.values.clone(),
            None => Vec::new(),
        };
        walk_source_files(root, &mut files, &ignored);
    }
    FilePathList { values: files }
}
