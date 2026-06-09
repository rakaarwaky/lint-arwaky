use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use /* UNKNOWN: ErrorMessage */ crate::shared_common::taxonomy_common_error::ErrorMessage;
use /* UNKNOWN: LayerNameVO */ crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use /* UNKNOWN: PatternList */ crate::shared_common::taxonomy_common_vo::PatternList;
use /* UNKNOWN: SuffixPolicyVO */ crate::shared_common::taxonomy_suffix_vo::SuffixPolicyVO;
use /* UNKNOWN: SuffixVO */ crate::shared_common::taxonomy_suffix_vo::SuffixVO;

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

impl ArchitectureRule {
    pub fn new(
        name: DescriptionVO,
        description: DescriptionVO,
        rule_type: ErrorCode,
        scope: LayerNameVO,
        word_count: Count,
        exceptions: PatternList,
        allowed_import: PatternList,
        forbidden_import: PatternList,
        mandatory_import: PatternList,
        suffix_policy: SuffixPolicyVO,
        allowed_suffix: PatternList,
        forbidden_suffix: PatternList,
        no_primitives: BooleanVO,
        mandatory_imports: Vec<MandatoryImportRuleVO>,
        barrel_completeness: BooleanVO,
        min_lines: Count,
        max_lines: Count,
        forbidden_bypass: PatternList,
        forbid_internal_all: BooleanVO,
        mandatory_class_definition: BooleanVO,
        dead_inheritance_bypass: BooleanVO,
        check_orphan: BooleanVO,
        orphan_entry_points: PatternList,
        check_unused_mandatory_imports: BooleanVO,
        forbidden_inheritance: PatternList,
        no_domain_logic: BooleanVO,
        must_implement_service_container_aggregate: BooleanVO,
        lazy_eager_initialization_only: BooleanVO,
        stateless_execution: BooleanVO,
        single_execution_goal: BooleanVO,
        high_level_policy_only: BooleanVO,
        coordinates_multiple_orchestrators: BooleanVO,
        crud_only: BooleanVO,
        no_decision_logic: BooleanVO,
        thread_async_safe: BooleanVO,
        no_domain_data_storage: BooleanVO,
        owns_system_health_transitions: BooleanVO,
        lifecycle_tracking_only: BooleanVO,
        forbid_any_type: BooleanVO,
    ) -> Self {
        Self {
            name,
            description,
            rule_type,
            scope,
            word_count,
            exceptions,
            allowed_import,
            forbidden_import,
            mandatory_import,
            suffix_policy,
            allowed_suffix,
            forbidden_suffix,
            no_primitives,
            mandatory_imports,
            barrel_completeness,
            min_lines,
            max_lines,
            forbidden_bypass,
            forbid_internal_all,
            mandatory_class_definition,
            dead_inheritance_bypass,
            check_orphan,
            orphan_entry_points,
            check_unused_mandatory_imports,
            forbidden_inheritance,
            no_domain_logic,
            must_implement_service_container_aggregate,
            lazy_eager_initialization_only,
            stateless_execution,
            single_execution_goal,
            high_level_policy_only,
            coordinates_multiple_orchestrators,
            crud_only,
            no_decision_logic,
            thread_async_safe,
            no_domain_data_storage,
            owns_system_health_transitions,
            lifecycle_tracking_only,
            forbid_any_type,
        }
    }
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
