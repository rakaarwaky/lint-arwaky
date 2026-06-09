pub const AES001_FORBIDDEN_IMPORT: &str =
    "AES001 FORBIDDEN_IMPORT: Layer is importing from a forbidden module.";
pub const AES002_MANDATORY_IMPORT: &str = "AES002 MANDATORY_IMPORT: Missing required import.";
pub const AES004_FILE_TOO_LARGE_MSG: &str =
    "AES004 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
    WHY? Large files violate the Single Responsibility Principle.\n\
    FIX: Split the module into smaller, more focused files";
pub const AES005_FILE_TOO_SHORT_MSG: &str =
    "AES005 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
    WHY? Excessively small files clutter the project structure.\n\
    FIX: Expand the component or merge this logic into a related module";
pub const AES006_PRIMITIVE_USAGE: &str = "AES006 PRIMITIVE_USAGE: Direct primitive in taxonomy.";
pub const AES009_MANDATORY_CLASS_DEFINITION: &str =
    "AES009 MANDATORY_CLASS_DEFINITION: File is missing a class, interface, or type definition.\n\
    WHY? Encapsulation in classes/interfaces is required for proper modularization and contract adherence.\n\
    FIX: Group functions into a class or implement an interface that defines the module boundary.";
pub const AES011_SUFFIX_FORBIDDEN: &str =
    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
    FIX: Rename the file to use an allowed suffix or move it to the correct layer.";
pub const AES021_STATELESS_EXECUTION: &str =
    "Non-stateless behavior detected: state assignment found outside constructor.";
pub const AES021_HIGH_LEVEL_POLICY: &str =
    "Low-level implementation details found (infrastructure import).";
pub const AES021_COORDINATES_MULTIPLE: &str = "Coordinator must manage multiple orchestrators.";
pub const AES021_NO_DOMAIN_LOGIC: &str = "Complex domain logic detected in a passive layer/role.";
pub const AES021_LAZY_EAGER_INIT: &str = "Complex initialization logic found in Container.";
pub const AES021_MUST_IMPLEMENT_CONTRACT: &str = "Class must implement ServiceContainerAggregate.";
pub const AES024_ANY_TYPE: &str = "Any type annotation found in agent orchestrator layer.";
pub const AES026_FORBIDDEN_INHERITANCE: &str =
    "AES026 FORBIDDEN_INHERITANCE: implemented from forbidden source.";
pub const AES033_CONSTANT_PURITY: &str =
    "AES033 CONSTANT_PURITY: _constant file contains non-constant declaration.";

pub fn aes003_naming_convention(expected_word_count: i32) -> String {
    format!(
        "AES003 NAMING_CONVENTION: Filename does not follow the {}-word underscore-separated pattern.\n\
        WHY? Strict three-word names ensure architectural consistency.\n\
        FIX: Rename the file to exactly {} words separated by underscores.", expected_word_count, expected_word_count
    )
}

pub fn aes001_forbidden_import(layer_name: &str, module: &str) -> String {
    format!(
        "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden module '{}'.",
        layer_name, module
    )
}

pub fn aes002_mandatory_import(required: &str) -> String {
    format!(
        "AES002 MANDATORY_IMPORT: Missing required import: '{}'.",
        required
    )
}

pub fn aes011_suffix_mismatch(allowed_list: &str) -> String {
    format!(
        "AES011 SUFFIX_MISMATCH: File is missing a required strict suffix for this layer.\n\
        WHY? Strict suffixes ensure every component has a clear role.\n\
        FIX: Add one of the required suffixes: {}.",
        allowed_list
    )
}

pub fn aes006_primitive_usage(primitive: &str) -> String {
    format!(
        "AES006 PRIMITIVE_USAGE: Direct primitive '{}' in taxonomy.",
        primitive
    )
}

pub fn aes015_unused_import(name: &str) -> String {
    format!("AES015 UNUSED_IMPORT: '{}' imported but never used.", name)
}

pub fn aes026_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES026 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.",
        trait_name
    )
}

pub fn aes021_must_implement_contract(contract_name: &str) -> String {
    format!("Class must implement {}.", contract_name)
}

pub fn aes024_any_type(line: &str) -> String {
    format!(
        "Any type annotation found in agent orchestrator layer: '{}'.",
        line.trim()
    )
}
