// PURPOSE: FixRequest — request payload for the fix aggregate

use crate::common::taxonomy_path_vo::FilePath;

pub enum FixRequest {
    /// Run linter + apply fixes for all fixable violation types.
    Execute { path: FilePath, dry_run: bool },
}

impl FixRequest {
    pub fn execute(path: &FilePath, dry_run: bool) -> Self {
        Self::Execute {
            path: path.clone(),
            dry_run,
        }
    }
}
