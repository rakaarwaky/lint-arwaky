use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
/* UNKNOWN: PatternList */ use crate::shared_common::taxonomy_common_vo::PatternList;

#[derive(Debug, Clone, Default)]
pub struct DirectoryWatchAggregate {
    pub path: FilePath,
    pub recursive: BooleanVO,
    pub ignore_patterns: Option<PatternList>,
}
