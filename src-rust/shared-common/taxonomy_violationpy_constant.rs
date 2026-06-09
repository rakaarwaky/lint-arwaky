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
    "AES011 MANDATORY_CLASS_DEFINITION: File is missing a class, dataclass, or Protocol definition.\n\
    WHY? Encapsulation in classes/Protocols is required for proper modularization and contract adherence.\n\
    FIX: Group functions into a class or implement a Protocol that defines the module interface.";
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

pub fn aes010_naming_convention(_expected_word_count: i32) -> String {
    String::from(
        "AES010 NAMING_CONVENTION: Filename must follow [layer]_[concept(s)]_[suffix] pattern.\n\
        WHY? Prefix identifies layer, suffix identifies role, concept describes feature.\n\
        FIX: Rename to at least prefix_suffix (e.g., capabilities_user_checker.py)."
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

pub fn aes031_primitive_usage(primitive: &str) -> String {
    format!(
        "AES031 PRIMITIVE_USAGE: Direct primitive '{}' in taxonomy.",
        primitive
    )
}

pub fn aes023_unused_import(name: &str) -> String {
    format!("AES023 UNUSED_IMPORT: '{}' imported but never used.", name)
}

pub fn aes032_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES032 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.",
        trait_name
    )
}

pub fn aes035_must_implement_contract(contract_name: &str) -> String {
    format!("Class must implement {}.", contract_name)
}

pub fn aes035_any_type(line: &str) -> String {
    format!(
        "Any type annotation found in agent orchestrator layer: '{}'.",
        line.trim()
    )
}
