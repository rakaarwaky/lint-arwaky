// PURPOSE: AES violation message constants for JavaScript/TypeScript analysis
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
pub const AES0301_PRIMITIVE_USAGE: &str = "AES0301 PRIMITIVE_USAGE: Direct primitive in taxonomy.";
pub const AES011_MANDATORY_CLASS_DEFINITION: &str =
    "AES011 MANDATORY_CLASS_DEFINITION: File is missing a class, interface, or type definition.\n\
    WHY? Encapsulation in classes/interfaces is required for proper modularization and contract adherence.\n\
    FIX: Group functions into a class or implement an interface that defines the module boundary.";
pub const AES011_SUFFIX_FORBIDDEN: &str =
    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
    FIX: Rename the file to use an allowed suffix or move it to the correct layer.";
pub const AES0305_STATELESS_EXECUTION: &str =
    "Non-stateless behavior detected: state assignment found outside constructor.";
pub const AES0305_HIGH_LEVEL_POLICY: &str =
    "Low-level implementation details found (infrastructure import).";
pub const AES0305_COORDINATES_MULTIPLE: &str = "Orchestrator must manage multiple subsystems.";
pub const AES0305_NO_DOMAIN_LOGIC: &str = "Complex domain logic detected in a passive layer/role.";
pub const AES0305_LAZY_EAGER_INIT: &str = "Complex initialization logic found in Container.";
pub const AES0305_MUST_IMPLEMENT_CONTRACT: &str = "Class must implement ServiceContainerAggregate.";
pub const AES0305_ANY_TYPE: &str = "Any type annotation found in agent orchestrator layer.";
pub const AES0302_FORBIDDEN_INHERITANCE: &str =
    "AES013 FORBIDDEN_INHERITANCE: implemented from forbidden source.";
pub const AES0301_CONSTANT_PURITY: &str =
    "AES0301 CONSTANT_PURITY: _constant file contains non-constant declaration.";
