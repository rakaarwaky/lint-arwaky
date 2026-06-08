//! Architecture compliance orchestration — layer-map resolution, compliance coordination,
//! watch mode, and DI mixin stubs.
//!
//! Naming: `arch` = bounded context (architecture governance), not layer name.
//! Also hosts watch orchestrators and mixin container stubs consolidated for cohesion.

use std::collections::HashMap;
use std::sync::OnceLock;

use async_trait::async_trait;

use crate::contract::{
    ArchCoordinatorAggregate, DirectoryWatchAggregate, IArchCompliancePort,
    IArchComplianceProtocol, IJobRegistryPort, InfrastructureContainerAggregate,
    OrchestratorContainerAggregate, ServiceContainerAggregate, WatchCommandsAggregate,
    WatchExecutionOrchestratorAggregate,
};
use crate::infrastructure::MemoryJobRegistryAdapter;
use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ArchitectureRule, BooleanVO, ComplianceStatus, FilePath,
    LayerDefinition, LayerMapVO, LayerNameVO, LintResultList, Score, WatchResult, LAYER_GLOBAL,
};

// ═══════════════════════════════════════════════════════════════════════════════
// MIXIN CONTAINERS — DI initialization stubs (wired via DependencyInjectionContainer)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct InfrastructureMixinContainer {}

impl InfrastructureMixinContainer {
    pub fn init_infrastructure(&self) {}
}

impl InfrastructureContainerAggregate for InfrastructureMixinContainer {
    fn _init_infrastructure(&mut self) {}

    fn root_path(&self) -> Option<&FilePath> {
        None
    }
}

pub struct OrchestratorMixinContainer {}

impl OrchestratorContainerAggregate for OrchestratorMixinContainer {
    fn _init_orchestrators(&mut self) {}
}

impl OrchestratorMixinContainer {
    pub fn init_orchestrators(&self) {}
}

// ═══════════════════════════════════════════════════════════════════════════════
// WATCH ORCHESTRATORS — CLI watch command + file-change execution
// ═══════════════════════════════════════════════════════════════════════════════

static WATCH_JOB_REGISTRY: OnceLock<MemoryJobRegistryAdapter> = OnceLock::new();

pub struct WatchCommandsOrchestrator {
    execution: WatchExecutionOrchestrator,
}

#[async_trait]
impl WatchCommandsAggregate for WatchCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn watch(&self, path: &FilePath) {
        self.execution.process_event(path);
    }
}

impl WatchCommandsOrchestrator {
    pub fn new() -> Self {
        Self {
            execution: WatchExecutionOrchestrator::new(),
        }
    }
}


pub struct WatchExecutionOrchestrator {}

impl WatchExecutionOrchestratorAggregate for WatchExecutionOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    fn job_registry(&self) -> &dyn IJobRegistryPort {
        WATCH_JOB_REGISTRY.get_or_init(MemoryJobRegistryAdapter::new)
    }
}

impl WatchExecutionOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_available(&self) -> bool {
        true
    }

    pub async fn execute(&self, _request: &DirectoryWatchAggregate) -> WatchResult {
        WatchResult {
            file: FilePath::new(".".to_string()).unwrap_or_default(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
        }
    }

    pub fn process_event(&self, file_path: &FilePath) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        result.insert("file".to_string(), serde_json::json!(file_path.value));
        result.insert("score".to_string(), serde_json::json!(0.0));
        result.insert("is_passing".to_string(), serde_json::json!(false));
        result
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARCHITECTURE COMPLIANCE — config resolution + multi-orchestrator coordination
// ═══════════════════════════════════════════════════════════════════════════════

pub struct ArchitectureOrchestrator {}

impl IArchComplianceProtocol for ArchitectureOrchestrator {
    fn execute(&self, _path: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
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
        if rule.min_lines.value > 0 {
            def.min_lines = rule.min_lines.clone();
        }
        if !rule.min_lines_violation_message.value.is_empty() {
            def.min_lines_violation_message = rule.min_lines_violation_message.clone();
        }
        if rule.max_lines.value > 0 {
            def.max_lines = rule.max_lines.clone();
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
        if !rule
            .mandatory_class_definition_violation_message
            .value
            .is_empty()
        {
            def.mandatory_class_definition_violation_message =
                rule.mandatory_class_definition_violation_message.clone();
        }
        if !rule
            .dead_inheritance_bypass_violation_message
            .value
            .is_empty()
        {
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
        if !rule
            .single_execution_goal_violation_message
            .value
            .is_empty()
        {
            def.single_execution_goal_violation_message =
                rule.single_execution_goal_violation_message.clone();
        }
        if !rule
            .high_level_policy_only_violation_message
            .value
            .is_empty()
        {
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
        if !rule
            .no_domain_data_storage_violation_message
            .value
            .is_empty()
        {
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
        if !rule
            .lifecycle_tracking_only_violation_message
            .value
            .is_empty()
        {
            def.lifecycle_tracking_only_violation_message =
                rule.lifecycle_tracking_only_violation_message.clone();
        }
        if !rule.forbid_any_type_violation_message.value.is_empty() {
            def.forbid_any_type_violation_message = rule.forbid_any_type_violation_message.clone();
        }

        if rule.no_primitives != BooleanVO::new(false) {
            def.no_primitives = rule.no_primitives.clone();
        }
        if !rule.barrel_completeness.value {
            def.barrel_completeness = rule.barrel_completeness.clone();
        }

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
