use crate::taxonomy::Count;
use crate::taxonomy::FilePathList;
use crate::taxonomy::RenamedFile;


#[derive(Debug, Clone, Default)]
pub struct GitDiffResultAggregate {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: Vec<RenamedFile>,
    pub lintable_files: FilePathList,
    pub all_files: FilePathList,
    pub total_changed: Count,
}
