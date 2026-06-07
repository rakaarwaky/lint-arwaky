use serde::{Deserialize, Serialize};

use crate::taxonomy::{
    BooleanVO, Count, CustomMessageVO, DirectoryPath, ErrorMessage, LayerNameVO,
    MandatoryImportRuleVO, PatternList, SuffixPolicyVO,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    pub path: DirectoryPath,
    pub suffix_policy: SuffixPolicyVO,
    #[serde(default, alias = "allowed_suffix")]
    pub allowed_suffix: PatternList,
    #[serde(default, alias = "forbidden_suffix")]
    pub forbidden_suffix: PatternList,
    #[serde(default)]
    pub allowed_import: PatternList,
    #[serde(default)]
    pub forbidden_import: PatternList,
    #[serde(default)]
    pub mandatory_import: PatternList,
    #[serde(default)]
    pub mandatory_import_violation_message: ErrorMessage,
    #[serde(default)]
    pub forbidden_import_violation_message: ErrorMessage,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,
    #[serde(default)]
    pub no_primitives: BooleanVO,
    #[serde(default)]
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    #[serde(default)]
    pub barrel_completeness: BooleanVO,
    #[serde(default)]
    pub min_lines: Count,
    #[serde(default)]
    pub max_lines: Count,
    #[serde(default)]
    pub word_count_violation_message: ErrorMessage,
    #[serde(default)]
    pub suffix_violation_message: ErrorMessage,
    #[serde(default)]
    pub no_primitives_violation_message: ErrorMessage,
    #[serde(default)]
    pub min_lines_violation_message: ErrorMessage,
    #[serde(default)]
    pub max_lines_violation_message: ErrorMessage,
    #[serde(default)]
    pub barrel_completeness_violation_message: ErrorMessage,
    #[serde(default)]
    pub forbid_internal_all: BooleanVO,
    #[serde(default)]
    pub forbid_internal_all_violation_message: ErrorMessage,
    #[serde(default)]
    pub forbidden_bypass: PatternList,
    #[serde(default)]
    pub forbidden_bypass_violation_message: ErrorMessage,
    #[serde(default)]
    pub forbidden_bypass_custom_messages: Vec<CustomMessageVO>,
    #[serde(default)]
    pub mandatory_class_definition: BooleanVO,
    #[serde(default)]
    pub mandatory_class_definition_violation_message: ErrorMessage,
    #[serde(default)]
    pub dead_inheritance_bypass: BooleanVO,
    #[serde(default)]
    pub dead_inheritance_bypass_violation_message: ErrorMessage,
    #[serde(default)]
    pub dead_inheritance_bypass_custom_messages: Vec<CustomMessageVO>,
    #[serde(default)]
    pub check_orphan: BooleanVO,
    #[serde(default)]
    pub orphan_entry_points: PatternList,
    #[serde(default)]
    pub orphan_violation_message: ErrorMessage,
    #[serde(default)]
    pub check_unused_mandatory_imports: BooleanVO,
    #[serde(default)]
    pub check_unused_mandatory_imports_violation_message: ErrorMessage,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
    #[serde(default)]
    pub forbidden_inheritance_violation_message: ErrorMessage,
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub no_domain_logic_violation_message: ErrorMessage,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate_violation_message: ErrorMessage,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only_violation_message: ErrorMessage,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub stateless_execution_violation_message: ErrorMessage,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub single_execution_goal_violation_message: ErrorMessage,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only_violation_message: ErrorMessage,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators_violation_message: ErrorMessage,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub crud_only_violation_message: ErrorMessage,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub no_decision_logic_violation_message: ErrorMessage,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub thread_async_safe_violation_message: ErrorMessage,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage_violation_message: ErrorMessage,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions_violation_message: ErrorMessage,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only_violation_message: ErrorMessage,
    #[serde(default)]
    pub forbid_any_type: BooleanVO,
    #[serde(default)]
    pub forbid_any_type_violation_message: ErrorMessage,
}

impl LayerDefinition {
    pub fn new(
        path: DirectoryPath,
        suffix_policy: SuffixPolicyVO,
        allowed_suffix: PatternList,
        forbidden_suffix: PatternList,
        allowed_import: PatternList,
        forbidden_import: PatternList,
        mandatory_import: PatternList,
        mandatory_import_violation_message: ErrorMessage,
        forbidden_import_violation_message: ErrorMessage,
        word_count: Count,
        exceptions: PatternList,
        recursive: BooleanVO,
        no_primitives: BooleanVO,
        mandatory_imports: Vec<MandatoryImportRuleVO>,
        barrel_completeness: BooleanVO,
        min_lines: Count,
        max_lines: Count,
        word_count_violation_message: ErrorMessage,
        suffix_violation_message: ErrorMessage,
        no_primitives_violation_message: ErrorMessage,
        min_lines_violation_message: ErrorMessage,
        max_lines_violation_message: ErrorMessage,
        barrel_completeness_violation_message: ErrorMessage,
        forbid_internal_all: BooleanVO,
        forbid_internal_all_violation_message: ErrorMessage,
        forbidden_bypass: PatternList,
        forbidden_bypass_violation_message: ErrorMessage,
        forbidden_bypass_custom_messages: Vec<CustomMessageVO>,
        mandatory_class_definition: BooleanVO,
        mandatory_class_definition_violation_message: ErrorMessage,
        dead_inheritance_bypass: BooleanVO,
        dead_inheritance_bypass_violation_message: ErrorMessage,
        dead_inheritance_bypass_custom_messages: Vec<CustomMessageVO>,
        check_orphan: BooleanVO,
        orphan_entry_points: PatternList,
        orphan_violation_message: ErrorMessage,
        check_unused_mandatory_imports: BooleanVO,
        check_unused_mandatory_imports_violation_message: ErrorMessage,
        forbidden_inheritance: PatternList,
        forbidden_inheritance_violation_message: ErrorMessage,
        no_domain_logic: BooleanVO,
        no_domain_logic_violation_message: ErrorMessage,
        must_implement_service_container_aggregate: BooleanVO,
        must_implement_service_container_aggregate_violation_message: ErrorMessage,
        lazy_eager_initialization_only: BooleanVO,
        lazy_eager_initialization_only_violation_message: ErrorMessage,
        stateless_execution: BooleanVO,
        stateless_execution_violation_message: ErrorMessage,
        single_execution_goal: BooleanVO,
        single_execution_goal_violation_message: ErrorMessage,
        high_level_policy_only: BooleanVO,
        high_level_policy_only_violation_message: ErrorMessage,
        coordinates_multiple_orchestrators: BooleanVO,
        coordinates_multiple_orchestrators_violation_message: ErrorMessage,
        crud_only: BooleanVO,
        crud_only_violation_message: ErrorMessage,
        no_decision_logic: BooleanVO,
        no_decision_logic_violation_message: ErrorMessage,
        thread_async_safe: BooleanVO,
        thread_async_safe_violation_message: ErrorMessage,
        no_domain_data_storage: BooleanVO,
        no_domain_data_storage_violation_message: ErrorMessage,
        owns_system_health_transitions: BooleanVO,
        owns_system_health_transitions_violation_message: ErrorMessage,
        lifecycle_tracking_only: BooleanVO,
        lifecycle_tracking_only_violation_message: ErrorMessage,
        forbid_any_type: BooleanVO,
        forbid_any_type_violation_message: ErrorMessage,
    ) -> Self {
        Self {
            path,
            suffix_policy,
            allowed_suffix,
            forbidden_suffix,
            allowed_import,
            forbidden_import,
            mandatory_import,
            mandatory_import_violation_message,
            forbidden_import_violation_message,
            word_count,
            exceptions,
            recursive,
            no_primitives,
            mandatory_imports,
            barrel_completeness,
            min_lines,
            max_lines,
            word_count_violation_message,
            suffix_violation_message,
            no_primitives_violation_message,
            min_lines_violation_message,
            max_lines_violation_message,
            barrel_completeness_violation_message,
            forbid_internal_all,
            forbid_internal_all_violation_message,
            forbidden_bypass,
            forbidden_bypass_violation_message,
            forbidden_bypass_custom_messages,
            mandatory_class_definition,
            mandatory_class_definition_violation_message,
            dead_inheritance_bypass,
            dead_inheritance_bypass_violation_message,
            dead_inheritance_bypass_custom_messages,
            check_orphan,
            orphan_entry_points,
            orphan_violation_message,
            check_unused_mandatory_imports,
            check_unused_mandatory_imports_violation_message,
            forbidden_inheritance,
            forbidden_inheritance_violation_message,
            no_domain_logic,
            no_domain_logic_violation_message,
            must_implement_service_container_aggregate,
            must_implement_service_container_aggregate_violation_message,
            lazy_eager_initialization_only,
            lazy_eager_initialization_only_violation_message,
            stateless_execution,
            stateless_execution_violation_message,
            single_execution_goal,
            single_execution_goal_violation_message,
            high_level_policy_only,
            high_level_policy_only_violation_message,
            coordinates_multiple_orchestrators,
            coordinates_multiple_orchestrators_violation_message,
            crud_only,
            crud_only_violation_message,
            no_decision_logic,
            no_decision_logic_violation_message,
            thread_async_safe,
            thread_async_safe_violation_message,
            no_domain_data_storage,
            no_domain_data_storage_violation_message,
            owns_system_health_transitions,
            owns_system_health_transitions_violation_message,
            lifecycle_tracking_only,
            lifecycle_tracking_only_violation_message,
            forbid_any_type,
            forbid_any_type_violation_message,
        }
    }

    pub fn path_str(&self) -> String {
        self.path.value.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerMapVO {
    pub values: std::collections::HashMap<LayerNameVO, LayerDefinition>,
}

impl LayerMapVO {
    pub fn new(value: std::collections::HashMap<LayerNameVO, LayerDefinition>) -> Self {
        Self { values: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingConfig {
    pub word_count: Count,
    pub word_count_violation_message: ErrorMessage,
}

impl NamingConfig {
    pub fn new(word_count: Count, word_count_violation_message: ErrorMessage) -> Self {
        Self {
            word_count,
            word_count_violation_message,
        }
    }
}
