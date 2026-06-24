// PURPOSE: ActionName — value object for pipeline job actions
// JobId is re-exported from common for backward compatibility
pub use crate::common::taxonomy_job_id_vo::JobId;
use crate::string_value_object;

string_value_object!(ActionName);
