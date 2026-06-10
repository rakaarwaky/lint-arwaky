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

pub const AES011_MANDATORY_CLASS_DEFINITION: &str =
    "AES011 MANDATORY_CLASS_DEFINITION: File is missing a struct, enum, or trait definition.\n\
    WHY? Encapsulation in structs/traits is required for proper modularization and contract adherence.\n\
    FIX: Group functions into a struct or implement a Trait that defines the module interface.";
pub const AES011_SUFFIX_FORBIDDEN: &str =
    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
    FIX: Rename the file to use an allowed suffix or move it to the correct layer.";
pub const AES0305_STATELESS_EXECUTION: &str =
    "Non-stateless behavior detected: state assignment found outside __init__.";
pub const AES0305_HIGH_LEVEL_POLICY: &str =
    "Low-level implementation details found (infrastructure import).";
pub const AES0305_COORDINATES_MULTIPLE: &str = "Orchestrator must manage multiple subsystems.";
pub const AES0305_NO_DOMAIN_LOGIC: &str = "Complex domain logic detected in a passive layer/role.";
pub const AES0305_LAZY_EAGER_INIT: &str = "Complex initialization logic found in Container.";
pub const AES0305_MUST_IMPLEMENT_CONTRACT: &str = "Class must implement ServiceContainerAggregate.";
pub const AES0305_ANY_TYPE: &str = "Any type annotation found in agent orchestrator layer.";
pub const AES013_FORBIDDEN_INHERITANCE: &str =
    "AES013 FORBIDDEN_INHERITANCE: implemented from forbidden source.";
pub const AES0301_CONSTANT_PURITY: &str =
    "AES0301 TAXONOMY_ROLE: _constant file contains non-constant declaration.";

pub use crate::shared_common::taxonomy_violation_rs_vo::*;

pub const AES011_SUFFIX_MISMATCH: &str =
    "AES011 SUFFIX_MISMATCH: Contract file missing _port, _protocol, or _aggregate suffix.";
pub const AES022_BYPASS_COMMENT: &str = "AES022 BYPASS_COMMENT: Bypass comment detected.";
pub const AES022_UNWRAP_EXPECT: &str = "AES022 BYPASS_COMMENT: unwrap/expect call detected.";
pub const AES022_PANIC: &str = "AES022 BYPASS_COMMENT: panic call detected.";
pub const AES023_FIX_UNUSED_IMPORT: &str = "AES023 UNUSED_IMPORT: Fixing unused import.";
pub const AES024_DEAD_INHERITANCE: &str =
    "AES024 DEAD_INHERITANCE: Empty struct or trait detected.";
pub const AES030_ORPHAN_CODE: &str = "AES030 ORPHAN_CODE: File has no imports, not an entry point.";
pub const AES0306_HIERARCHY_VIOLATION: &str =
    "AES0306 SURFACE_ROLE: Surface file is not imported from the layer barrel.";
pub const AES0306_PASSIVE_VIOLATION: &str =
    "AES0306 SURFACE_ROLE: Surface file contains active domain logic.";
pub const AES012_CIRCULAR_IMPORT: &str = "AES012 CIRCULAR_IMPORT: Circular dependencies detected.";
pub const AES0306_SURFACE_ROLE_VIOLATION: &str =
    "AES0306 SURFACE_ROLE: Surface file exceeds role mandate.";
pub const AES001_SURFACE_DEPENDENCY: &str =
    "AES001 SURFACE_DEPENDENCY: Surface imports from forbidden layer.";
pub const AES014_MANDATORY_INHERITANCE: &str =
    "AES014 MANDATORY_INHERITANCE: File imports contracts but no class implements them.";
pub const AES0303_CAPABILITY_ROUTING: &str =
    "AES0303 CAPABILITY_ROLE: Capability method not found in dispatch.";
pub const AES0303_SINGLE_BOTTLENECK: &str =
    "AES0303 CAPABILITY_ROLE: All dispatch routes go to a single capability.";
pub const AES0303_MISSING_VO: &str =
    "AES0303 CAPABILITY_ROLE: Capability method call missing required VO parameter.";
pub const AES0304_MISSING_VO: &str =
    "AES0304 INFRASTRUCTURE_ROLE: Infrastructure method call missing required VO parameter.";
