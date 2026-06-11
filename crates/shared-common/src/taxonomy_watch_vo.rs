// PURPOSE: DirectoryWatchVO — value object representing directory watch config parameters
use shared_common::taxonomy_common_vo::BooleanVO;
/* UNKNOWN: PatternList */ use shared_common::taxonomy_common_vo::PatternList;
use source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Default)]
pub struct DirectoryWatchVO {
    pub path: FilePath,
    pub recursive: BooleanVO,
    pub ignore_patterns: Option<PatternList>,
}
