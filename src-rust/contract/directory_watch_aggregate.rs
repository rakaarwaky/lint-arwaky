use crate::taxonomy::BooleanVO;
use crate::taxonomy::FilePath;
use crate::taxonomy::PatternList;

#[derive(Debug, Clone, Default)]
pub struct DirectoryWatchAggregate {
    pub path: FilePath,
    pub recursive: BooleanVO,
    pub ignore_patterns: Option<PatternList>,
}
