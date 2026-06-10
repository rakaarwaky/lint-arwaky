// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::shared_common::taxonomy_fix_vo::FixResult;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
