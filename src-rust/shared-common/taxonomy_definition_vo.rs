use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_rule_vo::MandatoryImportRuleVO;
use crate::shared_common::taxonomy_suffix_vo::SuffixPolicyVO;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

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
    pub forbid_internal_all: BooleanVO,
    #[serde(default)]
    pub forbidden_bypass: PatternList,
    #[serde(default)]
    pub mandatory_class_definition: BooleanVO,
    #[serde(default)]
    pub dead_inheritance_bypass: BooleanVO,
    #[serde(default)]
    pub check_orphan: BooleanVO,
    #[serde(default)]
    pub orphan_entry_points: PatternList,
    #[serde(default)]
    pub check_unused_mandatory_imports: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub forbid_any_type: BooleanVO,
}

impl LayerDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: DirectoryPath,
        suffix_policy: SuffixPolicyVO,
        allowed_suffix: PatternList,
        forbidden_suffix: PatternList,
        allowed_import: PatternList,
        forbidden_import: PatternList,
        mandatory_import: PatternList,
        word_count: Count,
        exceptions: PatternList,
        recursive: BooleanVO,
        no_primitives: BooleanVO,
        mandatory_imports: Vec<MandatoryImportRuleVO>,
        barrel_completeness: BooleanVO,
        min_lines: Count,
        max_lines: Count,
        forbid_internal_all: BooleanVO,
        forbidden_bypass: PatternList,
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
            path,
            suffix_policy,
            allowed_suffix,
            forbidden_suffix,
            allowed_import,
            forbidden_import,
            mandatory_import,
            word_count,
            exceptions,
            recursive,
            no_primitives,
            mandatory_imports,
            barrel_completeness,
            min_lines,
            max_lines,
            forbid_internal_all,
            forbidden_bypass,
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
}

impl NamingConfig {
    pub fn new(word_count: Count) -> Self {
        Self { word_count }
    }
}
