// PURPOSE: NamingViolation — AES101/AES102 violation data for naming rules domain
// Messages are owned by the capabilities checkers, not here.

#[derive(Debug, Clone)]
pub enum NamingViolation {
    /// AES101 — filename doesn't follow prefix_concept_suffix pattern
    NamingConvention { min_words: usize, separator: String },
    /// AES102 — filename prefix is not one of the recognised layer prefixes
    UnknownPrefix {
        prefix: String,
        allowed: Vec<String>,
    },
    /// AES102 — suffix is explicitly forbidden for this layer
    SuffixForbidden {
        layer_name: String,
        forbidden_suffix: String,
    },
    /// AES102 — strict suffix policy violated (suffix not in allowed list)
    SuffixMismatch {
        layer_name: String,
        used_suffix: String,
        allowed: Vec<String>,
    },
    /// AES102 — suffix belongs to a different layer's suffix set
    PrefixSuffixMismatch {
        expected_layer: String,
        actual_suffix: String,
        suffix_layer: String,
    },
    /// AES102 — suffix does not belong to any recognised layer's suffix set (strict only)
    UnknownSuffix {
        layer_name: String,
        unknown_suffix: String,
        all_suffixes: Vec<String>,
    },
}
