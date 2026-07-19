use crate::common::taxonomy_common_vo::bool;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Default)]
pub struct MultiProjectVO {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<bool>,
    pub config_path: Option<FilePath>,
}
