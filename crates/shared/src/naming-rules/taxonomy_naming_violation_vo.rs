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
                let why = Option::unwrap_or_else(
                    reason.as_ref().map(|r| r.to_string()),
                    || {
                        format!("The AES layer naming convention requires filenames to contain at least {} words separated by '{}' (e.g., prefix{}suffix). Each word must be lowercase alphanumeric. This pattern ensures every file's architectural layer (prefix) and role (suffix) is immediately identifiable — both for human readers and automated tooling.", min_words, separator, separator)
                    }
                );
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
                let why = Option::unwrap_or(
                    reason.as_ref().map(|r| r.to_string()),
                    default_why,
                );
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
                let why = Option::unwrap_or(
                    reason.as_ref().map(|r| r.to_string()),
                    default_why,
                );
                write!(
                    f,
                    "AES102 SUFFIX_FORBIDDEN: File in layer '{}' uses suffix '{}' which is forbidden.\n\
                    WHY? {}\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.",
                    layer_name, forbidden_suffix, why
                )
            }
            Self::SuffixMismatch {
                layer_name,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Files in the '{}' layer must end with a recognised role suffix ({}) so that their architectural intent is clear and automated boundary checks can verify that each file belongs exactly where it is. A missing or unrecognised suffix bypasses this safeguard.",
                    layer_name, allowed_str
                );
                let why = Option::unwrap_or(
                    reason.as_ref().map(|r| r.to_string()),
                    default_why,
                );
                write!(
                    f,
                    "AES102 SUFFIX_MISMATCH: File in layer '{}' is missing a required strict suffix.\n\
                    WHY? {}\n\
                    FIX: Rename the file to include one of the allowed suffixes: {}.",
                    layer_name, why, allowed_str
                )
            }
        }
    }
}
