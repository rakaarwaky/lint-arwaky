// PURPOSE: JobId — value object for pipeline job identifiers
//
// `JobId` is a thin wrapper around a `String` and is generated with the
// `string_value_object!` macro. It exists in its own file so that any
// crate needing job identifiers can `use` this type without pulling in the
// rest of the common VO namespace.
use crate::string_value_object;

string_value_object!(JobId);
