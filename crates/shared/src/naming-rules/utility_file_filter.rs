// PURPOSE: Naming file filter — standalone function for source file extension filtering
//
// Utility layer: stateless, no contract, no I/O abstraction.
// Agent or Capabilities call these directly when low-level technical operations are needed.

use crate::naming_rules::taxonomy_naming_constant::SOURCE_EXTENSIONS;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use std::path::Path;

/// Filter a file list to only include files with recognized source extensions.
///
/// Recognized extensions: rs, py, js, ts, jsx, tsx
pub fn filter_source_files(files: &FilePathList) -> FilePathList {
    let filtered: Vec<FilePath> = files
        .values
        .iter()
        .filter(|f| {
            let path = Path::new(&f.value);
            path.extension()
                .and_then(|e| e.to_str())
                .map(|ext| SOURCE_EXTENSIONS.contains(&ext))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    FilePathList::new(filtered)
}
