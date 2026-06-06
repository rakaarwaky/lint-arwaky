use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffResultVO {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
    pub lintable_files: FilePathList,
    pub all_files: FilePathList,
    pub total_changed: Count,
}

impl GitDiffResultVO {
    pub fn new(added: FilePathList, modified: FilePathList, deleted: FilePathList, renamed: RenamedFileList, lintable_files: FilePathList, all_files: FilePathList, total_changed: Count,) -> Self {
        Self { added, modified, deleted, renamed, lintable_files, all_files, total_changed }
    }
}
