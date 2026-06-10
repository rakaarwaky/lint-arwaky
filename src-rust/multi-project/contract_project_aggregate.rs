// PURPOSE: ProjectAggregate — aggregate trait for project-level operations (summary, compare)
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Default)]
pub struct MultiProjectAggregate {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<BooleanVO>,
    pub config_path: Option<FilePath>,
}
