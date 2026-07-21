// PURPOSE: NamingViolation — AES101/AES102 violation messages for naming rules domain
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NamingViolation {
    /// AES101 — filename doesn't follow prefix_concept_suffix pattern
    /// Min 2 words separated by underscore (e.g., prefix_suffix).
    NamingConvention {
        min_words: usize,
        separator: String,
        reason: Option<LintMessage>,
    },
    /// AES102 — filename prefix is not one of the recognised layer prefixes
    UnknownPrefix {
        prefix: String,
        allowed: Vec<String>,
        reason: Option<LintMessage>,
    },
    /// AES102 — suffix is explicitly forbidden for this layer
    /// Carries the layer name and the actual suffix used.
    SuffixForbidden {
        layer_name: String,
        forbidden_suffix: String,
        reason: Option<LintMessage>,
    },
    /// AES102 — strict suffix policy violated (missing required suffix)
    /// Carries the layer name and allowed suffixes from config for dynamic messages.
    SuffixMismatch {
        layer_name: String,
        used_suffix: String,
        allowed: Vec<String>,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for NamingViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamingConvention {
                min_words,
                separator,
                reason,
            } => {
                let why = Option::unwrap_or_else(reason.as_ref().map(|r| r.to_string()), || {
                    format!("The AES layer naming convention requires filenames to contain at least {} words separated by '{}' (e.g., prefix{}suffix). Each word must be lowercase alphanumeric. This pattern ensures every file's architectural layer (prefix) and role (suffix) is immediately identifiable — both for human readers and automated tooling.", min_words, separator, separator)
                });
                write!(
                    f,
                    "AES101 NAMING_CONVENTION: Filename must contain at least {} words separated by '{}'.\n\
                    WHY? {}\n\
                    FIX: Rename to follow prefix{}suffix pattern (e.g., capabilities{}user_checker.rs).",
                    min_words, separator, why, separator, separator
                )
            }
            Self::UnknownPrefix {
                prefix,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Every source file must begin with one of the recognised layer prefixes ({}) so that its architectural layer can be determined automatically. The prefix '{}' does not correspond to any known layer.",
                    allowed_str, prefix
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 UNKNOWN_PREFIX: File uses prefix '{}' which is not a recognised layer.\n\
                    WHY? {}\n\
                    FIX: Rename to start with one of the allowed prefixes: {}.",
                    prefix, why, allowed_str
                )
            }
            Self::SuffixForbidden {
                layer_name,
                forbidden_suffix,
                reason,
            } => {
                let default_why = format!(
                    "The suffix '{}' belongs to a different architectural role and is not allowed in the '{}' layer. Mixing role suffixes across layers breaks the strict layer-to-suffix mapping that tooling depends on for automatic validation.",
                    forbidden_suffix, layer_name
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 SUFFIX_FORBIDDEN: File in layer '{}' uses suffix '{}' which is forbidden.\n\
                    WHY? {}\n\
                    FIX: \
                     If this is business logic → move to the capabilities_ layer \
                     with an allowed suffix (e.g., analyzer, checker, processor). \
                     If this is stateless technical mechanics → move to the utility_ layer \
                     with an allowed suffix (e.g., parser, formatter, detector). \
                     If this is a shared interface → create a contract_ file with ONLY the trait definition \
                     (protocol/aggregate), no implementation logic. \
                     Implement the trait in capabilities_.",
                    layer_name, forbidden_suffix, why
                )
            }
            Self::SuffixMismatch {
                layer_name,
                used_suffix,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Suffix '{}' is not in the allowed list for layer '{}'. \
                     Allowed suffixes for '{}': {}. \
                     A suffix outside this list means either the file belongs in a different layer \
                     or needs a different architectural role suffix.",
                    used_suffix, layer_name, layer_name, allowed_str
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 SUFFIX_MISMATCH: File in layer '{}' uses suffix '{}' which is not in the allowed list.\n\
                    WHY? {}\n\
                    FIX: \
                     If this file implements domain types (structs, enums, macros) for the '{}' layer \
                     → rename suffix to one of: {}. \
                     If this file contains business logic (algorithms, analysis, parsing, formatting) \
                     → move to capabilities_ layer. \
                     If this file contains stateless technical mechanics (parsing, formatting, detection) \
                     → move to utility_ layer. \
                     If this logic must be shared across crates \
                     → create a contract_ file with ONLY the trait definition (protocol/aggregate), \
                       then implement that trait in a capabilities_ file. \
                       Contract files must NOT contain implementation logic.",
                    layer_name, used_suffix, why, layer_name, allowed_str
                )
            }
        }
    }
}
