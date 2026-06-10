// PURPOSE: AES violation message builder functions for JavaScript/TypeScript analysis
pub fn aes010_naming_convention(_expected_word_count: i32) -> String {
    String::from(
        "AES010 NAMING_CONVENTION: Filename must follow [layer]_[concept(s)]_[suffix] pattern.\n\
        WHY? Prefix identifies layer, suffix identifies role, concept describes feature.\n\
        FIX: Rename to at least prefix_suffix (e.g., infrastructure_user_checker.ts).",
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

pub fn aes0301_primitive_usage(primitive: &str) -> String {
    format!(
        "AES0301 TAXONOMY_ROLE: Direct primitive '{}' in taxonomy.",
        primitive
    )
}

pub fn aes023_unused_import(name: &str) -> String {
    format!("AES023 UNUSED_IMPORT: '{}' imported but never used.", name)
}

pub fn aes0302_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES0302 CONTRACT_ROLE: '{}' implemented from forbidden source.",
        trait_name
    )
}

pub fn aes0305_must_implement_contract(contract_name: &str) -> String {
    format!(
        "AES0305 AGENT_ROLE: Class must implement {}.",
        contract_name
    )
}

pub fn aes0305_any_type(line: &str) -> String {
    format!(
        "AES0305 AGENT_ROLE: Any type annotation found in agent orchestrator layer: '{}'.",
        line.trim()
    )
}
