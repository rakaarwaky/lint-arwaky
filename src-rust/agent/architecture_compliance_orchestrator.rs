// arch_compliance_orchestrator — Logic for resolving ArchitectureConfig into LayerDefinitions.
use crate::contract::IArchComplianceProtocol;
use crate::taxonomy::{
    ArchitectureConfig, ArchitectureRule, BooleanVO, ErrorMessage, LayerDefinition, LayerMapVO,
    LayerNameVO, LAYER_GLOBAL,
};
use std::collections::HashMap;

pub struct ArchitectureOrchestrator;

impl IArchComplianceProtocol for ArchitectureOrchestrator {
    fn execute(&self, _path: &crate::taxonomy::FilePath) -> crate::taxonomy::LintResultList {
        crate::taxonomy::LintResultList::new(vec![])
    }
}

impl ArchitectureOrchestrator {
    pub fn new() -> Self {
        Self
    }

    /// Merges base layer definitions with global and layer-specific rules.
    pub fn resolve_effective_layer_map(&self, config: &ArchitectureConfig) -> LayerMapVO {
        let mut layer_map = self.resolve_layers(config);
        layer_map = self.merge_layer_rules(layer_map, config);
        LayerMapVO { values: layer_map }
    }

    fn resolve_layers(&self, config: &ArchitectureConfig) -> HashMap<LayerNameVO, LayerDefinition> {
        config.layers.clone()
    }

    fn merge_layer_rules(
        &self,
        mut result: HashMap<LayerNameVO, LayerDefinition>,
        config: &ArchitectureConfig,
    ) -> HashMap<LayerNameVO, LayerDefinition> {
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            let base_name = scope.split('(').next().unwrap_or(&scope).to_string();
            let base_name_vo = LayerNameVO::new(&base_name);

            if !result.contains_key(&base_name_vo) {
                continue;
            }

            let scope_vo = LayerNameVO::new(&scope);
            if !result.contains_key(&scope_vo) && scope != base_name {
                if let Some(base_def) = result.get(&base_name_vo) {
                    result.insert(scope_vo.clone(), base_def.clone());
                }
            }

            if scope == LAYER_GLOBAL {
                self.apply_global_rule(&mut result, rule);
            } else if scope.contains('(') {
                self.apply_specialized_rule(&mut result, &scope_vo, rule);
            } else {
                self.apply_base_rule(&mut result, &scope_vo, rule);
            }
        }
        result
    }

    fn apply_global_rule(
        &self,
        result: &mut HashMap<LayerNameVO, LayerDefinition>,
        rule: &ArchitectureRule,
    ) {
        let keys: Vec<LayerNameVO> = result.keys().cloned().collect();
        for layer_name in keys {
            if layer_name.to_string().contains('(') {
                continue;
            }
            if let Some(def) = result.get(&layer_name).cloned() {
                result.insert(layer_name, self.merge_rule_into_definition(def, rule));
            }
        }
    }

    fn apply_specialized_rule(
        &self,
        result: &mut HashMap<LayerNameVO, LayerDefinition>,
        scope: &LayerNameVO,
        rule: &ArchitectureRule,
    ) {
        if let Some(def) = result.get(scope).cloned() {
            result.insert(scope.clone(), self.merge_rule_into_definition(def, rule));
        }
    }

    fn apply_base_rule(
        &self,
        result: &mut HashMap<LayerNameVO, LayerDefinition>,
        scope: &LayerNameVO,
        rule: &ArchitectureRule,
    ) {
        if let Some(def) = result.get(scope).cloned() {
            result.insert(scope.clone(), self.merge_rule_into_definition(def, rule));
        }
    }

    fn merge_rule_into_definition(
        &self,
        mut def: LayerDefinition,
        rule: &ArchitectureRule,
    ) -> LayerDefinition {
        // Override simple scalar fields
        if rule.word_count.value != 0 {
            def.word_count = rule.word_count.clone();
        }
        if rule.suffix_policy.value != "strict" {
            def.suffix_policy = rule.suffix_policy.clone();
        }
        if !rule.mandatory_import_violation_message.value.is_empty() {
            def.mandatory_import_violation_message =
                rule.mandatory_import_violation_message.clone();
        }
        if !rule.forbidden_import_violation_message.value.is_empty() {
            def.forbidden_import_violation_message =
                rule.forbidden_import_violation_message.clone();
        }
        if !rule.word_count_violation_message.value.is_empty() {
            def.word_count_violation_message = rule.word_count_violation_message.clone();
        }
        if !rule.suffix_violation_message.value.is_empty() {
            def.suffix_violation_message = rule.suffix_violation_message.clone();
        }
        if !rule.no_primitives_violation_message.value.is_empty() {
            def.no_primitives_violation_message = rule.no_primitives_violation_message.clone();
        }
        if !rule.min_lines_violation_message.value.is_empty() {
            def.min_lines_violation_message = rule.min_lines_violation_message.clone();
        }
        if !rule.max_lines_violation_message.value.is_empty() {
            def.max_lines_violation_message = rule.max_lines_violation_message.clone();
        }
        if !rule.barrel_completeness_violation_message.value.is_empty() {
            def.barrel_completeness_violation_message =
                rule.barrel_completeness_violation_message.clone();
        }
        if !rule.forbid_internal_all_violation_message.value.is_empty() {
            def.forbid_internal_all_violation_message =
                rule.forbid_internal_all_violation_message.clone();
        }
        if !rule.forbidden_bypass_violation_message.value.is_empty() {
            def.forbidden_bypass_violation_message =
                rule.forbidden_bypass_violation_message.clone();
        }
        if !rule.mandatory_class_definition_violation_message.value.is_empty() {
            def.mandatory_class_definition_violation_message =
                rule.mandatory_class_definition_violation_message.clone();
        }
        if !rule.dead_inheritance_bypass_violation_message.value.is_empty() {
            def.dead_inheritance_bypass_violation_message =
                rule.dead_inheritance_bypass_violation_message.clone();
        }
        if !rule.orphan_violation_message.value.is_empty() {
            def.orphan_violation_message = rule.orphan_violation_message.clone();
        }
        if !rule
            .check_unused_mandatory_imports_violation_message
            .value
            .is_empty()
        {
            def.check_unused_mandatory_imports_violation_message = rule
                .check_unused_mandatory_imports_violation_message
                .clone();
        }
        if !rule.no_domain_logic_violation_message.value.is_empty() {
            def.no_domain_logic_violation_message = rule.no_domain_logic_violation_message.clone();
        }
        if !rule
            .must_implement_service_container_aggregate_violation_message
            .value
            .is_empty()
        {
            def.must_implement_service_container_aggregate_violation_message = rule
                .must_implement_service_container_aggregate_violation_message
                .clone();
        }
        if !rule
            .lazy_eager_initialization_only_violation_message
            .value
            .is_empty()
        {
            def.lazy_eager_initialization_only_violation_message = rule
                .lazy_eager_initialization_only_violation_message
                .clone();
        }
        if !rule.stateless_execution_violation_message.value.is_empty() {
            def.stateless_execution_violation_message =
                rule.stateless_execution_violation_message.clone();
        }
        if !rule.single_execution_goal_violation_message.value.is_empty() {
            def.single_execution_goal_violation_message =
                rule.single_execution_goal_violation_message.clone();
        }
        if !rule.high_level_policy_only_violation_message.value.is_empty() {
            def.high_level_policy_only_violation_message =
                rule.high_level_policy_only_violation_message.clone();
        }
        if !rule
            .coordinates_multiple_orchestrators_violation_message
            .value
            .is_empty()
        {
            def.coordinates_multiple_orchestrators_violation_message = rule
                .coordinates_multiple_orchestrators_violation_message
                .clone();
        }
        if !rule.crud_only_violation_message.value.is_empty() {
            def.crud_only_violation_message = rule.crud_only_violation_message.clone();
        }
        if !rule.no_decision_logic_violation_message.value.is_empty() {
            def.no_decision_logic_violation_message =
                rule.no_decision_logic_violation_message.clone();
        }
        if !rule.thread_async_safe_violation_message.value.is_empty() {
            def.thread_async_safe_violation_message =
                rule.thread_async_safe_violation_message.clone();
        }
        if !rule.no_domain_data_storage_violation_message.value.is_empty() {
            def.no_domain_data_storage_violation_message =
                rule.no_domain_data_storage_violation_message.clone();
        }
        if !rule
            .owns_system_health_transitions_violation_message
            .value
            .is_empty()
        {
            def.owns_system_health_transitions_violation_message = rule
                .owns_system_health_transitions_violation_message
                .clone();
        }
        if !rule.lifecycle_tracking_only_violation_message.value.is_empty() {
            def.lifecycle_tracking_only_violation_message =
                rule.lifecycle_tracking_only_violation_message.clone();
        }
        if !rule.forbid_any_type_violation_message.value.is_empty() {
            def.forbid_any_type_violation_message = rule.forbid_any_type_violation_message.clone();
        }

        // Override boolean fields
        if rule.no_primitives != BooleanVO::new(false) {
            def.no_primitives = rule.no_primitives.clone();
        }
        if !rule.barrel_completeness.value {
            def.barrel_completeness = rule.barrel_completeness.clone();
        }

        // Collection fields
        if !rule.allowed_import.values.is_empty() {
            def.allowed_import = rule.allowed_import.clone();
        }
        if !rule.forbidden_import.values.is_empty() {
            def.forbidden_import = rule.forbidden_import.clone();
        }
        if !rule.mandatory_import.values.is_empty() {
            def.mandatory_import = rule.mandatory_import.clone();
        }
        if !rule.allowed_suffix.values.is_empty() {
            def.allowed_suffix = rule.allowed_suffix.clone();
        }
        if !rule.forbidden_suffix.values.is_empty() {
            def.forbidden_suffix = rule.forbidden_suffix.clone();
        }
        if !rule.exceptions.values.is_empty() {
            def.exceptions = rule.exceptions.clone();
        }
        if !rule.forbidden_bypass.values.is_empty() {
            def.forbidden_bypass = rule.forbidden_bypass.clone();
        }
        if !rule.orphan_entry_points.values.is_empty() {
            def.orphan_entry_points = rule.orphan_entry_points.clone();
        }

        def
    }
}
