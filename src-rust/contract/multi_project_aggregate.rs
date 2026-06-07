use crate::taxonomy::{BooleanVO, FilePath, FilePathList};

#[derive(Debug, Clone, Default)]
pub struct MultiProjectAggregate {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<BooleanVO>,
    pub config_path: Option<FilePath>,
}
