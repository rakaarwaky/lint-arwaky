//! Constants for core primitive type names.
//!
//! Defines the set of built-in primitive types (str, int, float)
//! used for naming convention analysis and type detection.
//!
//! These are the types that trigger AES006 violations when found
//! in domain-level value objects and entities. Extend this list
//! when adding support for additional primitive types.

pub const CORE_PRIMITIVE_TYPES: &[&str] = &["str", "int", "float"];
