// PURPOSE: naming constants — shared rule codes, adapter names, and layer prefixes for naming-rules feature

/// Rule code for AES101 — Naming Convention Consistency
pub const RULE_CODE_NAMING_CONVENTION: &str = "AES101";

/// Rule code for AES102 — Suffix/Prefix Layer Alignment
pub const RULE_CODE_SUFFIX_PREFIX: &str = "AES102";

/// Adapter name for architecture lint
pub const ADAPTER_NAME: &str = "architecture";

/// AES layer prefixes (must match extract_layer_from_prefix in LayerDetectionAnalyzer)
pub const LAYER_PREFIXES: &[&str] = &[
    "taxonomy_",
    "contract_",
    "utility_",
    "capabilities_",
    "agent_",
    "surface_",
    "root_",
];

/// Separator for snake_case naming
pub const SNAKE_CASE_SEPARATOR: &str = "_";

/// Suffix policy value for strict enforcement
pub const SUFFIX_POLICY_STRICT: &str = "strict";

/// Source file extensions recognized by naming checks
pub const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "jsx", "tsx"];
