// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
