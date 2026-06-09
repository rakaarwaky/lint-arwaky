pub const AES001_FORBIDDEN_IMPORT: &str =
    "AES001 FORBIDDEN_IMPORT: Layer is importing from a forbidden module.";
pub const AES002_MANDATORY_IMPORT: &str = "AES002 MANDATORY_IMPORT: Missing required import.";
pub const AES020_FILE_TOO_LARGE_MSG: &str =
    "AES020 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
    WHY? Large files violate the Single Responsibility Principle.\n\
    FIX: Split the module into smaller, more focused files";
pub const AES021_FILE_TOO_SHORT_MSG: &str =
    "AES021 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
    WHY? Excessively small files clutter the project structure.\n\
    FIX: Expand the component or merge this logic into a related module";
pub const AES031_PRIMITIVE_USAGE: &str = "AES031 PRIMITIVE_USAGE: Direct primitive in taxonomy.";
pub const AES011_MANDATORY_CLASS_DEFINITION: &str =
    "AES011 MANDATORY_CLASS_DEFINITION: File is missing a struct, enum, or trait definition.\n\
    WHY? Encapsulation in structs/traits is required for proper modularization and contract adherence.\n\
    FIX: Group functions into a struct or implement a Trait that defines the module interface.";
pub const AES011_SUFFIX_FORBIDDEN: &str =
    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
    FIX: Rename the file to use an allowed suffix or move it to the correct layer.";
pub const AES035_STATELESS_EXECUTION: &str =
    "Non-stateless behavior detected: state assignment found outside __init__.";
pub const AES035_HIGH_LEVEL_POLICY: &str =
    "Low-level implementation details found (infrastructure import).";
pub const AES035_COORDINATES_MULTIPLE: &str = "Coordinator must manage multiple orchestrators.";
pub const AES035_NO_DOMAIN_LOGIC: &str = "Complex domain logic detected in a passive layer/role.";
pub const AES035_LAZY_EAGER_INIT: &str = "Complex initialization logic found in Container.";
pub const AES035_MUST_IMPLEMENT_CONTRACT: &str = "Class must implement ServiceContainerAggregate.";
pub const AES035_ANY_TYPE: &str = "Any type annotation found in agent orchestrator layer.";
pub const AES032_FORBIDDEN_INHERITANCE: &str =
    "AES032 FORBIDDEN_INHERITANCE: implemented from forbidden source.";
pub const AES031_CONSTANT_PURITY: &str =
    "AES031 CONSTANT_PURITY: _constant file contains non-constant declaration.";

pub use crate::shared_common::taxonomy_violationrs_vo::*;

pub const AES011_SUFFIX_MISMATCH: &str =
    "AES011 SUFFIX_MISMATCH: Contract file missing _port, _protocol, or _aggregate suffix.";
pub const AES022_BYPASS_COMMENT: &str = "AES022 BYPASS_COMMENT: Bypass comment detected.";
pub const AES022_UNWRAP_EXPECT: &str = "AES022 BYPASS_COMMENT: unwrap/expect call detected.";
pub const AES022_PANIC: &str = "AES022 BYPASS_COMMENT: panic call detected.";
pub const AES023_FIX_UNUSED_IMPORT: &str = "AES023 UNUSED_IMPORT: Fixing unused import.";
pub const AES024_DEAD_INHERITANCE: &str =
    "AES024 DEAD_INHERITANCE: Empty struct or trait detected.";
pub const AES030_ORPHAN_CODE: &str = "AES030 ORPHAN_CODE: File has no imports, not an entry point.";
pub const AES036_HIERARCHY_VIOLATION: &str =
    "AES036 SURFACE_HIERARCHY_VIOLATION: Surface file is not imported from the layer barrel.";
pub const AES036_PASSIVE_VIOLATION: &str =
    "AES036 PASSIVE_SURFACE_VIOLATION: Surface file contains active domain logic.";
pub const AES012_CIRCULAR_IMPORT: &str = "AES012 CIRCULAR_IMPORT: Circular dependencies detected.";
pub const AES036_SURFACE_ROLE_VIOLATION: &str =
    "AES036 SURFACE_ROLE: Surface file exceeds role mandate.";
pub const AES001_SURFACE_DEPENDENCY: &str =
    "AES001 SURFACE_DEPENDENCY: Surface imports from forbidden layer.";
pub const AES034_MANDATORY_INHERITANCE: &str =
    "AES034 MANDATORY_INHERITANCE: File imports contracts but no class implements them.";
pub const AES033_CAPABILITY_ROUTING: &str =
    "AES033 CAPABILITY_ROUTING: Capability method not found in dispatch.";
pub const AES033_SINGLE_BOTTLENECK: &str =
    "AES033 SINGLE_BOTTLENECK: All dispatch routes go to a single capability.";
pub const AES033_MISSING_VO: &str =
    "AES033 MISSING_VO: Capability method call missing required VO parameter.";
