use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_suffix_vo::SuffixPolicyVO;
use crate::shared_common::taxonomy_suffix_vo::SuffixVO;
use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
    pub scope: LayerNameVO,
    pub word_count: Count,
    pub exceptions: PatternList,
    pub allowed_import: PatternList,
    pub forbidden_import: PatternList,
    pub mandatory_import: PatternList,
    pub suffix_policy: SuffixPolicyVO,
    pub allowed_suffix: PatternList,
    pub forbidden_suffix: PatternList,
    pub no_primitives: BooleanVO,
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    pub barrel_completeness: BooleanVO,
    pub min_lines: Count,
    pub max_lines: Count,
    pub forbidden_bypass: PatternList,
    pub forbid_internal_all: BooleanVO,
    pub mandatory_class_definition: BooleanVO,
    pub dead_inheritance_bypass: BooleanVO,
    pub check_orphan: BooleanVO,
    pub orphan_entry_points: PatternList,
    pub check_unused_mandatory_imports: BooleanVO,
    pub forbidden_inheritance: PatternList,
    pub no_domain_logic: BooleanVO,
    pub must_implement_service_container_aggregate: BooleanVO,
    pub lazy_eager_initialization_only: BooleanVO,
    pub stateless_execution: BooleanVO,
    pub single_execution_goal: BooleanVO,
    pub high_level_policy_only: BooleanVO,
    pub coordinates_multiple_orchestrators: BooleanVO,
    pub crud_only: BooleanVO,
    pub no_decision_logic: BooleanVO,
    pub thread_async_safe: BooleanVO,
    pub no_domain_data_storage: BooleanVO,
    pub owns_system_health_transitions: BooleanVO,
    pub lifecycle_tracking_only: BooleanVO,
    pub forbid_any_type: BooleanVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomMessageVO {
    pub pattern: String,
    pub message: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegacyLayerRule {
    pub source_layer: LayerNameVO,
    pub forbidden_target: LayerNameVO,
    pub description: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LegacyLayerRuleList {
    pub values: Vec<LegacyLayerRule>,
}

impl LegacyLayerRuleList {
    pub fn new(value: Vec<LegacyLayerRule>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LegacyLayerRule> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LegacyLayerRule) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MandatoryImportRuleVO {
    pub suffix: SuffixVO,
    pub imports: PatternList,
}



impl CustomMessageVO {
    pub fn new(pattern: String, message: ErrorMessage) -> Self {
        Self { pattern, message }
    }
}

impl MandatoryImportRuleVO {
    pub fn new(suffix: SuffixVO, imports: PatternList) -> Self {
        Self { suffix, imports }
    }
}

impl LegacyLayerRule {
    pub fn new(
        source_layer: LayerNameVO,
        forbidden_target: LayerNameVO,
        description: ErrorMessage,
    ) -> Self {
        Self {
            source_layer,
            forbidden_target,
            description,
        }
    }
}
