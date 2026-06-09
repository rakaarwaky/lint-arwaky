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
    "AES009 MANDATORY_CLASS_DEFINITION: File is missing a struct, enum, or trait definition.\n\
    WHY? Encapsulation in structs/traits is required for proper modularization and contract adherence.\n\
    FIX: Group functions into a struct or implement a Trait that defines the module interface.";
pub const AES011_SUFFIX_FORBIDDEN: &str =
    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n\
    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
    FIX: Rename the file to use an allowed suffix or move it to the correct layer.";
pub const AES021_STATELESS_EXECUTION: &str =
    "Non-stateless behavior detected: state assignment found outside __init__.";
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

pub fn aes003_naming_convention(_expected_word_count: i32) -> String {
    String::from(
        "AES003 NAMING_CONVENTION: Filename must follow prefix_concept_suffix pattern.\n\
        WHY? Prefix identifies layer, suffix identifies role, concept describes feature.\n\
        FIX: Rename to at least prefix_suffix (e.g., capabilities_user_checker.rs)."
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
    format!(
        "AES021 AGENT_ROLE: Class must implement '{}'.",
        contract_name
    )
}

pub fn aes024_any_type(line: &str) -> String {
    format!(
        "AES024 AGENT_ANY_BYPASS: Any type annotation found in agent orchestrator layer: '{}'.",
        line.trim()
    )
}

pub const AES008_SUFFIX_MISMATCH: &str =
    "AES008 SUFFIX_MISMATCH: Contract file missing _port, _protocol, or _aggregate suffix.";
pub const AES014_BYPASS_COMMENT: &str = "AES014 BYPASS_COMMENT: Bypass comment detected.";
pub const AES014_UNWRAP_EXPECT: &str = "AES014 BYPASS_COMMENT: unwrap/expect call detected.";
pub const AES014_PANIC: &str = "AES014 BYPASS_COMMENT: panic call detected.";
pub const AES015_FIX_UNUSED_IMPORT: &str = "AES015 UNUSED_IMPORT: Fixing unused import.";
pub const AES016_DEAD_INHERITANCE: &str =
    "AES016 DEAD_INHERITANCE: Empty struct or trait detected.";
pub const AES017_ORPHAN_CODE: &str = "AES017 ORPHAN_CODE: File has no imports, not an entry point.";
pub const AES018_HIERARCHY_VIOLATION: &str =
    "AES018 SURFACE_HIERARCHY_VIOLATION: Surface file is not imported from the layer barrel.";
pub const AES019_PASSIVE_VIOLATION: &str =
    "AES019 PASSIVE_SURFACE_VIOLATION: Surface file contains active domain logic.";
pub const AES020_CIRCULAR_IMPORT: &str = "AES020 CIRCULAR_IMPORT: Circular dependencies detected.";
pub const AES022_SURFACE_ROLE_VIOLATION: &str =
    "AES022 SURFACE_ROLE: Surface file exceeds role mandate.";
pub const AES023_SURFACE_DEPENDENCY: &str =
    "AES023 SURFACE_DEPENDENCY: Surface imports from forbidden layer.";
pub const AES025_MCP_SCHEMA: &str =
    "AES025 MCP_SCHEMA: MCP tool is missing required schema elements.";
pub const AES027_MANDATORY_INHERITANCE: &str =
    "AES027 MANDATORY_INHERITANCE: File imports contracts but no class implements them.";
pub const AES030_CAPABILITY_ROUTING: &str =
    "AES030 CAPABILITY_ROUTING: Capability method not found in dispatch.";
pub const AES031_SINGLE_BOTTLENECK: &str =
    "AES031 SINGLE_BOTTLENECK: All dispatch routes go to a single capability.";
pub const AES032_MISSING_VO: &str =
    "AES032 MISSING_VO: Capability method call missing required VO parameter.";

pub fn aes014_bypass_comment(line: &str) -> String {
    format!(
        "AES014 BYPASS_COMMENT: Bypass comment detected on line: '{}'.",
        line.trim()
    )
}
pub fn aes016_dead_inheritance(type_name: &str) -> String {
    format!(
        "AES016 DEAD_INHERITANCE: Empty struct/trait '{}' detected.",
        type_name
    )
}
pub fn aes017_orphan_code(file: &str) -> String {
    format!("AES017 ORPHAN_CODE: File '{}' is unreachable/unused.", file)
}
pub fn aes018_hierarchy_violation(file: &str) -> String {
    format!("AES018 SURFACE_HIERARCHY_VIOLATION: Surface file '{}' is not imported from the layer barrel.\nWHY? All surface files must be reachable through the barrel.\nFIX: Add to __init__.py or mod.rs.", file)
}
pub fn aes019_passive_viotation_details(file: &str, details: &str) -> String {
    format!("AES019 PASSIVE_SURFACE_VIOLATION: Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.", file, details)
}
pub fn aes020_circular_import(source: &str, target: &str) -> String {
    format!(
        "AES020 CIRCULAR_IMPORT: Circular dependency detected: '{}' -> '{}'.",
        source, target
    )
}
pub fn aes027_mandatory_inheritance(contracts: &str) -> String {
    format!("AES027 MANDATORY_INHERITANCE: File imports contracts ({}) but no class inherits from them.\nWHY? Layers that import contracts must provide an implementation.\nFIX: Add impl TraitName for YourStruct.", contracts)
}
pub fn aes030_capability_routing(struct_name: &str) -> String {
    format!(
        "AES030 CAPABILITY_ROUTING: Struct '{}' has no trait impl.",
        struct_name
    )
}
pub fn aes031_single_bottleneck(target: &str) -> String {
    format!(
        "AES031 SINGLE_BOTTLENECK: All dispatch routes go to '{}'.",
        target
    )
}
pub fn aes032_missing_vo(method: &str) -> String {
    format!(
        "AES032 MISSING_VO: Capability method '{}' missing required VO parameter.",
        method
    )
}
pub fn aes025_mcp_schema(field: &str) -> String {
    format!("AES025 MCP_SCHEMA: MCP tool is missing '{}'.", field)
}
