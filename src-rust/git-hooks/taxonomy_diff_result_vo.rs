// PURPOSE: GitDiffResultVO — value object representing git diff results
use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::source_parsing::taxonomy_paths_vo::RenamedFileList;

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
    pub fn new(
        added: FilePathList,
        modified: FilePathList,
        deleted: FilePathList,
        renamed: RenamedFileList,
        lintable_files: FilePathList,
        all_files: FilePathList,
        total_changed: Count,
    ) -> Self {
        Self {
            added,
            modified,
            deleted,
            renamed,
            lintable_files,
            all_files,
            total_changed,
        }
    }
}
