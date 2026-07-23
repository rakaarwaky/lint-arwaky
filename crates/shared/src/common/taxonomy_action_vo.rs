// PURPOSE: ActionName — value object for pipeline job actions
//
// `ActionName` identifies a single step within a pipeline (e.g. "lint",
// "build", "test"). It is a thin string wrapper produced by the
// `string_value_object!` macro.
//
// JobId is re-exported from common for backward compatibility.
pub use crate::common::taxonomy_job_id_vo::JobId;
use crate::string_value_object;

string_value_object!(ActionName);
