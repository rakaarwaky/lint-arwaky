// PURPOSE: Aggregate: Result aggregation/wiring
use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
/* UNKNOWN: RenamedFile */ use crate::source_parsing::taxonomy_paths_vo::RenamedFile;

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
