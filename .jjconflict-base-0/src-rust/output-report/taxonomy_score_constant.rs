//! Constants for lint output format identifiers.
//!
//! Defines the supported format strings used when generating
//! lint reports (text, JSON, SARIF, JUnit).
//!
//! Each constant maps to a file extension or MIME type used
//! by the report formatter when writing output files.

pub const FORMAT_TEXT: &str = "text";
pub const FORMAT_JSON: &str = "json";
pub const FORMAT_SARIF: &str = "sarif";
pub const FORMAT_JUNIT: &str = "junit";

