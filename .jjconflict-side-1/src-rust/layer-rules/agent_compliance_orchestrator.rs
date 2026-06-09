//! Architecture compliance orchestration — layer-map resolution, compliance coordination.
//!
//! Naming: `arch` = bounded context (architecture governance), not layer name.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::contract_compliance_port::IArchCompliancePort;
use crate::layer_rules::contract_compliance_protocol::IArchComplianceProtocol;
use crate::layer_rules::contract_coordinator_aggregate::ArchCoordinatorAggregate;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_definition_vo::LayerMapVO;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_names_constant::LAYER_GLOBAL;
use crate::shared_common::taxonomy_rule_vo::ArchitectureRule;
use crate::source_parsing::taxonomy_path_vo::FilePath;

// ═══════════════════════════════════════════════════════════════════════════════
// ARCHITECTURE COMPLIANCE — config resolution + multi-orchestrator coordination
// ═══════════════════════════════════════════════════════════════════════════════

pub struct ArchitectureOrchestrator {}

impl IArchComplianceProtocol for ArchitectureOrchestrator {
    fn execute(&self, _path: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
}

impl Default for ArchitectureOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

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
        if rule.suffix_policy.value != "strict" {
            def.suffix_policy = rule.suffix_policy.clone();
        }
        if rule.min_lines.value > 0 {
            def.min_lines = rule.min_lines.clone();
        }
        if rule.max_lines.value > 0 {
            def.max_lines = rule.max_lines.clone();
        }
        if rule.no_primitives != BooleanVO::new(false) {
            def.no_primitives = rule.no_primitives.clone();
        }
        if !rule.allowed.values.is_empty() {
            def.allowed = rule.allowed.clone();
        }
        if !rule.forbidden.values.is_empty() {
            def.forbidden = rule.forbidden.clone();
        }
        if !rule.mandatory.values.is_empty() {
            def.mandatory = rule.mandatory.clone();
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

pub struct ArchComplianceCoordinator {
    orchestrators: Vec<Box<dyn IArchComplianceProtocol + Send + Sync>>,
}

impl ArchComplianceCoordinator {
    pub fn new(
        compliance_orchestrator: Box<dyn IArchComplianceProtocol + Send + Sync>,
        additional_orchestrators: Option<Vec<Box<dyn IArchComplianceProtocol + Send + Sync>>>,
    ) -> Self {
        let mut orchestrators = vec![compliance_orchestrator];
        if let Some(additional) = additional_orchestrators {
            orchestrators.extend(additional);
        }
        Self { orchestrators }
    }

    pub fn name(&self) -> AdapterName {
        AdapterName::raw("architecture")
    }
}

#[async_trait]
impl ArchCoordinatorAggregate for ArchComplianceCoordinator {
    async fn check_compliance(&self, path: &FilePath) -> ComplianceStatus {
        let result = ArchCoordinatorAggregate::scan(self, path).await;
        ComplianceStatus::new(result.values.is_empty())
    }

    async fn scan(&self, path: &FilePath) -> LintResultList {
        let mut results = LintResultList::new(Vec::new());
        for orchestrator in &self.orchestrators {
            let mut partial = orchestrator.execute(path);
            results.values.append(&mut partial.values);
        }
        results
    }

    async fn apply_fix(&self, _path: &FilePath) -> ComplianceStatus {
        ComplianceStatus::new(false)
    }
}

#[async_trait]
impl IArchCompliancePort for ArchComplianceCoordinator {
    async fn scan(&self, path: &FilePath) -> LintResultList {
        (self as &dyn ArchCoordinatorAggregate).scan(path).await
    }

    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus {
        (self as &dyn ArchCoordinatorAggregate)
            .apply_fix(path)
            .await
    }
}
