// PURPOSE: Aggregate: Watch aggregation/wiring
use crate::shared_common::taxonomy_common_vo::BooleanVO;
/* UNKNOWN: PatternList */ use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Default)]
pub struct DirectoryWatchAggregate {
    pub path: FilePath,
    pub recursive: BooleanVO,
    pub ignore_patterns: Option<PatternList>,
}
