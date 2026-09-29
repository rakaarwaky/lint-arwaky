// PURPOSE: FixResponse — response payload for the fix aggregate

use crate::auto_fix::taxonomy_auto_fix_vo::FixResult;

pub enum FixResponse {
    Execute { result: FixResult },
}

impl FixResponse {
    /// Take the fix result.
    pub fn into_fix_result(self) -> FixResult {
        match self {
            Self::Execute { result } => result,
        }
    }
}
