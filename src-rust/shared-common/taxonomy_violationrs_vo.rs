pub fn aes010_naming_convention(_expected_word_count: i32) -> String {
    String::from(
        "AES010 NAMING_CONVENTION: Filename must follow prefix_concept_suffix pattern.\n\
        WHY? Prefix identifies layer, suffix identifies role, concept describes feature.\n\
        FIX: Rename to at least prefix_suffix (e.g., capabilities_user_checker.rs).",
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
    format!(
        "AES035 AGENT_ROLE: Class must implement '{}'.",
        contract_name
    )
}

pub fn aes035_any_type(line: &str) -> String {
    format!(
        "AES035 AGENT_ANY_BYPASS: Any type annotation found in agent orchestrator layer: '{}'.",
        line.trim()
    )
}

pub fn aes022_bypass_comment(line: &str) -> String {
    format!(
        "AES022 BYPASS_COMMENT: Bypass comment detected on line: '{}'.",
        line.trim()
    )
}

pub fn aes024_dead_inheritance(type_name: &str) -> String {
    format!(
        "AES024 DEAD_INHERITANCE: Empty struct/trait '{}' detected.",
        type_name
    )
}

pub fn aes030_orphan_code(file: &str) -> String {
    format!("AES030 ORPHAN_CODE: File '{}' is unreachable/unused.", file)
}

pub fn aes036_hierarchy_violation(file: &str) -> String {
    format!("AES036 SURFACE_HIERARCHY_VIOLATION: Surface file '{}' is not imported from the layer barrel.\nWHY? All surface files must be reachable through the barrel.\nFIX: Add to __init__.py or mod.rs.", file)
}

pub fn aes036_passive_violation_details(file: &str, details: &str) -> String {
    format!("AES036 PASSIVE_SURFACE_VIOLATION: Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.", file, details)
}

pub fn aes012_circular_import(source: &str, target: &str) -> String {
    format!(
        "AES012 CIRCULAR_IMPORT: Circular dependency detected: '{}' -> '{}'.",
        source, target
    )
}

pub fn aes034_mandatory_inheritance(contracts: &str) -> String {
    format!("AES034 MANDATORY_INHERITANCE: File imports contracts ({}) but no class inherits from them.\nWHY? Layers that import contracts must provide an implementation.\nFIX: Add impl TraitName for YourStruct.", contracts)
}

pub fn aes033_capability_routing(struct_name: &str) -> String {
    format!(
        "AES033 CAPABILITY_ROUTING: Struct '{}' has no trait impl.",
        struct_name
    )
}

pub fn aes033_single_bottleneck(target: &str) -> String {
    format!(
        "AES033 SINGLE_BOTTLENECK: All dispatch routes go to '{}'.",
        target
    )
}

pub fn aes033_missing_vo(method: &str) -> String {
    format!(
        "AES033 MISSING_VO: Capability method '{}' missing required VO parameter.",
        method
    )
}
