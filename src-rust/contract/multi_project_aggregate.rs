use crate::taxonomy::{FilePathList, FilePath};

#[derive(Debug, Clone, Default)]
pub struct MultiProjectAggregate {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<bool>,
    pub config_path: Option<FilePath>,
}
