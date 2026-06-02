// git_diff_manager — Git diff result implementation (Agent Layer).
use crate::contract::DiffResultAggregate;
use crate::taxonomy::{Count, FilePathList, RenamedFileList};

#[derive(Debug, Clone)]
pub struct GitDiffResult {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
    pub lintable_files: FilePathList,
    pub all_files: FilePathList,
    pub total_changed: Count,
}

impl DiffResultAggregate for GitDiffResult {}
