//! Architecture compliance orchestration — layer-map resolution, compliance coordination,
//! watch mode, and DI mixin stubs.
//!
//! Naming: `arch` = bounded context (architecture governance), not layer name.
//! Also hosts watch orchestrators and mixin container stubs consolidated for cohesion.

use std::collections::HashMap;
use std::sync::OnceLock;

use async_trait::async_trait;

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::di_containers::contract_infra_aggregate::InfrastructureContainerAggregate;
use crate::di_containers::contract_orchestrator_aggregate::OrchestratorContainerAggregate;
use crate::file_watch::contract_commands_aggregate::WatchCommandsAggregate;
use crate::file_watch::contract_orchestrator_aggregate::WatchExecutionOrchestratorAggregate;
use crate::file_watch::contract_watch_aggregate::DirectoryWatchAggregate;
use crate::file_watch::taxonomy_result_vo::WatchResult;
use crate::layer_rules::contract_compliance_port::IArchCompliancePort;
use crate::layer_rules::contract_compliance_protocol::IArchComplianceProtocol;
use crate::layer_rules::contract_coordinator_aggregate::ArchCoordinatorAggregate;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::pipeline_jobs::taxonomy_registry_error::JobError;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::ResponseDataList;
use crate::shared_common::taxonomy_duration_vo::Duration;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_common_vo::Score;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_definition_vo::LayerMapVO;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_names_constant::LAYER_GLOBAL;
use crate::shared_common::taxonomy_rule_vo::ArchitectureRule;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

struct SimpleJobRegistry;
#[async_trait::async_trait]
impl IJobRegistryPort for SimpleJobRegistry {
    async fn create_job(&self, _action: ActionName) -> Result<JobId, JobError> {
        Ok(JobId::new("stub"))
    }
    async fn complete_job(&self, _job_id: &JobId, _result: &ResponseData) {}
    async fn fail_job(&self, _job_id: &JobId, _error: &ErrorMessage) {}
    async fn list_jobs(&self) -> ResponseDataList {
        ResponseDataList { values: vec![] }
    }
    async fn get_job(&self, _job_id: &JobId) -> Option<JobId> {
        None
    }
    async fn cancel_job(&self, _job_id: &JobId) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    async fn run_with_retry(
        &self,
        _operation: ActionName,
        _max_retries: Count,
        _base_delay: Duration,
    ) -> ResponseData {
        ResponseData::default()
    }
}

static WATCH_JOB_REGISTRY: OnceLock<SimpleJobRegistry> = OnceLock::new();

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

impl Default for WatchCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
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
        WATCH_JOB_REGISTRY.get_or_init(|| SimpleJobRegistry)
    }
}

impl Default for WatchExecutionOrchestrator {
    fn default() -> Self {
        Self::new()
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
