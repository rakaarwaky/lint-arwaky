// PURPOSE: ActionName — value object for pipeline job actions
//
// `ActionName` identifies a single step within a pipeline (e.g. "lint",
// "build", "test"). It is a thin string wrapper produced by the
// `string_value_object!` macro.
//
// JobId is re-exported from common for backward compatibility.
use shared_common::string_value_object;
pub use shared_common::taxonomy_job_vo::JobId;

string_value_object!(ActionName);
