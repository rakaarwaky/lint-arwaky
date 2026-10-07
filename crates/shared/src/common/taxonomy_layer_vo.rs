// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
//
// These value objects are used throughout the AES layer-identity system:
// - FileContentVO wraps the raw text of a source file.
// - Identity identifies a single AES architectural layer.
// - LayerNameVO is a human-readable label for a layer.
// - LineContentVO wraps a single line of source text.
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);

// ─── Merged from taxonomy_definition_vo ───────────────────────

use crate::taxonomy_common_vo::BooleanVO;
use crate::taxonomy_common_vo::Count;
use crate::taxonomy_common_vo::PatternList;
use crate::taxonomy_common_vo::SuffixPolicyVO;
use serde::{Deserialize, Serialize};

/// Wrap a single-field VO that exposes a `new(value)` constructor plus the
/// default `derive`s needed by the codebase. Used to keep the boilerplate
/// for `LayerMapVO`/`NamingConfig` uniform without introducing a new linter
/// violation cluster.
macro_rules! single_field_vo {
    ($name:ident, $field:ident: $field_ty:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        #[serde(crate = "serde")]
        pub struct $name {
            pub $field: $field_ty,
        }

        impl $name {
            pub fn new($field: $field_ty) -> Self {
                Self { $field }
            }
        }
    };
}

/// Per-layer naming configuration: suffix policy, allowed suffixes, forbidden suffixes.
/// Used by the naming-rules feature crate (AES102) to validate file suffixes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerNamingConfig {
    #[serde(default)]
    pub suffix_policy: SuffixPolicyVO,
    #[serde(default, alias = "allowed_suffix")]
    pub allowed_suffix: PatternList,
    #[serde(default, alias = "forbidden_suffix")]
    pub forbidden_suffix: PatternList,
}

/// Per-layer orphan detection configuration.
/// Used by the orphan-rules feature crate to check if orphan detection is
/// enabled for a specific layer and which entry-point patterns apply.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OrphanRuleVO {
    #[serde(default)]
    pub check_orphan: BooleanVO,
    #[serde(default, alias = "entry_points")]
    pub orphan_entry_points: PatternList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,
    #[serde(default)]
    pub naming: LayerNamingConfig,
    #[serde(default)]
    pub orphan: OrphanRuleVO,
    #[serde(flatten, default)]
    pub code_analysis: crate::taxonomy_code_analysis_vo::CodeAnalysisRuleVO,
}

single_field_vo!(LayerMapVO, values: std::collections::HashMap<LayerNameVO, LayerDefinition>);
single_field_vo!(NamingConfig, word_count: Count);
