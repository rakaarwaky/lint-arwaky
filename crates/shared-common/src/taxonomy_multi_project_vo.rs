// PURPOSE: MultiProjectVO — value object containing parameters for multi-project analysis
use shared_common::taxonomy_common_vo::BooleanVO;
use source_parsing::taxonomy_path_vo::FilePath;
use source_parsing::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Default)]
pub struct MultiProjectVO {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<BooleanVO>,
    pub config_path: Option<FilePath>,
}
