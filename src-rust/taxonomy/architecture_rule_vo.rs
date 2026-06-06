use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub mandatory_import_violation_message: ErrorMessage,
    pub forbidden_import_violation_message: ErrorMessage,
    pub suffix_policy: SuffixPolicyVO,
    pub allowed_suffix: PatternList,
    pub forbidden_suffix: PatternList,
    pub no_primitives: BooleanVO,
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    pub barrel_completeness: BooleanVO,
    pub min_lines: Count,
    pub max_lines: Count,
    pub word_count_violation_message: ErrorMessage,
    pub suffix_violation_message: ErrorMessage,
    pub no_primitives_violation_message: ErrorMessage,
    pub min_lines_violation_message: ErrorMessage,
    pub max_lines_violation_message: ErrorMessage,
    pub barrel_completeness_violation_message: ErrorMessage,
    pub forbidden_bypass: PatternList,
    pub forbidden_bypass_violation_message: ErrorMessage,
    pub forbidden_bypass_custom_messages: Vec<CustomMessageVO>,
    pub forbid_internal_all: BooleanVO,
    pub forbid_internal_all_violation_message: ErrorMessage,
    pub mandatory_class_definition: BooleanVO,
    pub mandatory_class_definition_violation_message: ErrorMessage,
    pub dead_inheritance_bypass: BooleanVO,
    pub dead_inheritance_bypass_violation_message: ErrorMessage,
    pub dead_inheritance_bypass_custom_messages: Vec<CustomMessageVO>,
    pub check_orphan: BooleanVO,
    pub orphan_entry_points: PatternList,
    pub orphan_violation_message: ErrorMessage,
    pub check_unused_mandatory_imports: BooleanVO,
    pub check_unused_mandatory_imports_violation_message: ErrorMessage,
    pub forbidden_inheritance: PatternList,
    pub forbidden_inheritance_violation_message: ErrorMessage,
    pub no_domain_logic: BooleanVO,
    pub no_domain_logic_violation_message: ErrorMessage,
    pub must_implement_service_container_aggregate: BooleanVO,
    pub must_implement_service_container_aggregate_violation_message: ErrorMessage,
    pub lazy_eager_initialization_only: BooleanVO,
    pub lazy_eager_initialization_only_violation_message: ErrorMessage,
    pub stateless_execution: BooleanVO,
    pub stateless_execution_violation_message: ErrorMessage,
    pub single_execution_goal: BooleanVO,
    pub single_execution_goal_violation_message: ErrorMessage,
    pub high_level_policy_only: BooleanVO,
    pub high_level_policy_only_violation_message: ErrorMessage,
    pub coordinates_multiple_orchestrators: BooleanVO,
    pub coordinates_multiple_orchestrators_violation_message: ErrorMessage,
    pub crud_only: BooleanVO,
    pub crud_only_violation_message: ErrorMessage,
    pub no_decision_logic: BooleanVO,
    pub no_decision_logic_violation_message: ErrorMessage,
    pub thread_async_safe: BooleanVO,
    pub thread_async_safe_violation_message: ErrorMessage,
    pub no_domain_data_storage: BooleanVO,
    pub no_domain_data_storage_violation_message: ErrorMessage,
    pub owns_system_health_transitions: BooleanVO,
    pub owns_system_health_transitions_violation_message: ErrorMessage,
    pub lifecycle_tracking_only: BooleanVO,
    pub lifecycle_tracking_only_violation_message: ErrorMessage,
    pub forbid_any_type: BooleanVO,
    pub forbid_any_type_violation_message: ErrorMessage,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub fn new(name: DescriptionVO, description: DescriptionVO, rule_type: ErrorCode, scope: LayerNameVO, word_count: Count, exceptions: PatternList, allowed_import: PatternList, forbidden_import: PatternList, mandatory_import: PatternList, mandatory_import_violation_message: ErrorMessage, forbidden_import_violation_message: ErrorMessage, suffix_policy: SuffixPolicyVO, allowed_suffix: PatternList, forbidden_suffix: PatternList, no_primitives: BooleanVO, mandatory_imports: Vec<MandatoryImportRuleVO>, barrel_completeness: BooleanVO, min_lines: Count, max_lines: Count, word_count_violation_message: ErrorMessage, suffix_violation_message: ErrorMessage, no_primitives_violation_message: ErrorMessage, min_lines_violation_message: ErrorMessage, max_lines_violation_message: ErrorMessage, barrel_completeness_violation_message: ErrorMessage, forbidden_bypass: PatternList, forbidden_bypass_violation_message: ErrorMessage, forbidden_bypass_custom_messages: Vec<CustomMessageVO>, forbid_internal_all: BooleanVO, forbid_internal_all_violation_message: ErrorMessage, mandatory_class_definition: BooleanVO, mandatory_class_definition_violation_message: ErrorMessage, dead_inheritance_bypass: BooleanVO, dead_inheritance_bypass_violation_message: ErrorMessage, dead_inheritance_bypass_custom_messages: Vec<CustomMessageVO>, check_orphan: BooleanVO, orphan_entry_points: PatternList, orphan_violation_message: ErrorMessage, check_unused_mandatory_imports: BooleanVO, check_unused_mandatory_imports_violation_message: ErrorMessage, forbidden_inheritance: PatternList, forbidden_inheritance_violation_message: ErrorMessage, no_domain_logic: BooleanVO, no_domain_logic_violation_message: ErrorMessage, must_implement_service_container_aggregate: BooleanVO, must_implement_service_container_aggregate_violation_message: ErrorMessage, lazy_eager_initialization_only: BooleanVO, lazy_eager_initialization_only_violation_message: ErrorMessage, stateless_execution: BooleanVO, stateless_execution_violation_message: ErrorMessage, single_execution_goal: BooleanVO, single_execution_goal_violation_message: ErrorMessage, high_level_policy_only: BooleanVO, high_level_policy_only_violation_message: ErrorMessage, coordinates_multiple_orchestrators: BooleanVO, coordinates_multiple_orchestrators_violation_message: ErrorMessage, crud_only: BooleanVO, crud_only_violation_message: ErrorMessage, no_decision_logic: BooleanVO, no_decision_logic_violation_message: ErrorMessage, thread_async_safe: BooleanVO, thread_async_safe_violation_message: ErrorMessage, no_domain_data_storage: BooleanVO, no_domain_data_storage_violation_message: ErrorMessage, owns_system_health_transitions: BooleanVO, owns_system_health_transitions_violation_message: ErrorMessage, lifecycle_tracking_only: BooleanVO, lifecycle_tracking_only_violation_message: ErrorMessage, forbid_any_type: BooleanVO, forbid_any_type_violation_message: ErrorMessage,) -> Self {
        Self { name, description, rule_type, scope, word_count, exceptions, allowed_import, forbidden_import, mandatory_import, mandatory_import_violation_message, forbidden_import_violation_message, suffix_policy, allowed_suffix, forbidden_suffix, no_primitives, mandatory_imports, barrel_completeness, min_lines, max_lines, word_count_violation_message, suffix_violation_message, no_primitives_violation_message, min_lines_violation_message, max_lines_violation_message, barrel_completeness_violation_message, forbidden_bypass, forbidden_bypass_violation_message, forbidden_bypass_custom_messages, forbid_internal_all, forbid_internal_all_violation_message, mandatory_class_definition, mandatory_class_definition_violation_message, dead_inheritance_bypass, dead_inheritance_bypass_violation_message, dead_inheritance_bypass_custom_messages, check_orphan, orphan_entry_points, orphan_violation_message, check_unused_mandatory_imports, check_unused_mandatory_imports_violation_message, forbidden_inheritance, forbidden_inheritance_violation_message, no_domain_logic, no_domain_logic_violation_message, must_implement_service_container_aggregate, must_implement_service_container_aggregate_violation_message, lazy_eager_initialization_only, lazy_eager_initialization_only_violation_message, stateless_execution, stateless_execution_violation_message, single_execution_goal, single_execution_goal_violation_message, high_level_policy_only, high_level_policy_only_violation_message, coordinates_multiple_orchestrators, coordinates_multiple_orchestrators_violation_message, crud_only, crud_only_violation_message, no_decision_logic, no_decision_logic_violation_message, thread_async_safe, thread_async_safe_violation_message, no_domain_data_storage, no_domain_data_storage_violation_message, owns_system_health_transitions, owns_system_health_transitions_violation_message, lifecycle_tracking_only, lifecycle_tracking_only_violation_message, forbid_any_type, forbid_any_type_violation_message }
    }
}

impl CustomMessageVO {
    pub fn new(pattern: String, message: ErrorMessage,) -> Self {
        Self { pattern, message }
    }
}

impl MandatoryImportRuleVO {
    pub fn new(suffix: SuffixVO, imports: PatternList,) -> Self {
        Self { suffix, imports }
    }
}

impl LegacyLayerRule {
    pub fn new(source_layer: LayerNameVO, forbidden_target: LayerNameVO, description: ErrorMessage,) -> Self {
        Self { source_layer, forbidden_target, description }
    }
}
