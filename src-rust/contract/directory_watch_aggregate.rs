use crate::taxonomy::FilePath;


#[derive(Debug, Clone, Default)]
pub struct DirectoryWatchAggregate {
    pub path: FilePath,
    pub recursive: bool,
    pub ignore_patterns: Option<Vec<String>>,
}
